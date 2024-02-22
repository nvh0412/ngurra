mod action;
mod assets;
mod components;
mod db;
mod errors;
mod models;
mod ngurra;
mod repositories;
mod state;
mod storage;
mod theme;
mod ui;

use gpui::*;
use ngurra::Ngurra;
pub use repositories::deck::Deck;
pub use repositories::flash_card::FlashCard;

use std::{
    io::{IsTerminal, Write},
    path::PathBuf,
};

use crate::{
    assets::Assets,
    models::{
        builder::Builder,
        collection::{Collection, CollectionBuilder},
    },
    theme::Theme,
};

fn main() {
    let collection = CollectionBuilder::new(PathBuf::from("ngurra.db"))
        .build()
        .unwrap_or_else(|e| {
            panic!("Error opening collection: {:?}", e);
        });

    init_logger();

    log::info!("========== starting Ngurra ==========");
    App::new().with_assets(Assets).run(|cx: &mut AppContext| {
        ngurra::init(cx);
        Theme::init(cx);
        Collection::init(collection, cx);

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Point {
                        x: 100.0.into(),
                        y: 100.0.into(),
                    },
                    size: Size {
                        width: 840.0.into(),
                        height: 600.0.into(),
                    },
                }),
                titlebar: Some(TitlebarOptions {
                    // title: SharedString::from("Ngurra Flash Card").into(),
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.0), px(12.0))),
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
