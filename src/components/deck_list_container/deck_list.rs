use gpui::{div, prelude::*, rgb, Render, View, WindowContext};
use rusqlite::Connection;

use crate::{theme::Theme, Deck};

pub struct DeckList;

impl DeckList {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }

    fn get_all_decks(&self) -> Vec<Deck> {
        let conn = Connection::open("anki-rs.db").unwrap();
        let decks = Deck::get_all_decks(&conn);

        match decks {
            Ok(decks) => decks,
            Err(e) => {
                eprintln!("Error getting decks: {}", e);
                vec![]
            }
        }
    }
}

impl Render for DeckList {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let theme: &Theme = cx.global();

        div()
            .mt_2()
            .border_1()
            .border_color(theme.crust)
            .rounded_xl()
            .border_r_1()
            .text_color(theme.text)
            .p_3()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(div().px_2().min_w_40().child(format!("Deck")))
                    .child(div().px_2().min_w_20().flex().justify_center().child(format!("New")))
                    .child(div().px_2().min_w_20().flex().justify_center().child(format!("Learn")))
                    .child(div().px_2().min_w_20().flex().justify_center().child(format!("Due")))
            )
            .children(
                self.get_all_decks()
                    .into_iter()
                    .map(|deck| {
                        div()
                            .flex()
                            .flex_row()
                            .child(div().px_2().min_w_40().child(deck.name))
                            .child(div().px_2().min_w_20().flex().justify_center().child(format!("0")))
                            .child(div().px_2().min_w_20().flex().justify_center().child(format!("1")))
                            .child(div().px_2().min_w_20().flex().justify_center().child(format!("3")))
                    })
                    .collect::<Vec<_>>()
            )
    }
}
