mod components;
mod db;
mod models;
mod ngurra;
mod repositories;
mod state;
mod theme;
mod ui;

use db::init_db;
use gpui::*;
use ngurra::Ngurra;
pub use repositories::deck::Deck;
pub use repositories::flash_card::FlashCard;
use rusqlite::Connection;

use std::io::{IsTerminal, Write};

use crate::theme::Theme;

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
        Theme::init(cx);

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Point {
                        x: 100.0.into(),
                        y: 100.0.into(),
                    },
                    size: Size {
                        width: 800.0.into(),
                        height: 600.0.into(),
                    },
                }),
                titlebar: Some(TitlebarOptions {
                    title: Default::default(),
                    appears_transparent: Default::default(),
                    traffic_light_position: Default::default(),
                }),
                center: true,
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                display_id: None,
            },
            |cx| Ngurra::view(cx),
        );
    });
}

fn init_logger() {
    if stdout_is_pty() {
        env_logger::builder()
            .format(|buf, record| {
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
            })
            .init();
    }
}

fn stdout_is_pty() -> bool {
    std::io::stdout().is_terminal()
}
