mod models;
mod db;
mod ngurra;

use db::init_db;
use gpui::*;
pub use models::flash_card::FlashCard;
pub use models::deck::Deck;
use ngurra::Ngurra;
use rusqlite::{Connection, Result};

use std::io::{IsTerminal, Write};

fn main() {
    if let Ok(conn) = Connection::open("anki-rs.db") {
        init_db(&conn).unwrap();
    } else {
        panic!("Failed to open database");
    }

    init_logger();

    log::info!("========== starting Ngurra ==========");
    App::new().run(|cx: &mut AppContext| {
        ngurra::init(cx);

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Ngurra::new())
        });
    });
}

fn init_logger() {
    if stdout_is_pty() {
        env_logger::builder().format(|buf, record| {
            write!(buf, "{}", "[")?;
            write!(
                buf,
                "{} ",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z")
            )?;
            write!(buf, "{:<5}", buf.default_styled_level(record.level()))?;
            if let Some(path) = record.module_path() {
                write!(buf, " {}", path)?;
            }
            write!(buf, "{}", "]")?;
            writeln!(buf, "{}", record.args())
        }).init();
    }
}

fn stdout_is_pty() -> bool {
    std::io::stdout().is_terminal()
}
