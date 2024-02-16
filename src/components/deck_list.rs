use gpui::{div, prelude::*, AnyView, Render, View, WindowContext};
use rusqlite::Connection;

use crate::{models::deck::get_decks, theme::Theme, Deck};

pub struct DeckListView;

impl DeckListView {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }

    fn get_all_decks(&self) -> Vec<Deck> {
        let conn = Connection::open("anki-rs.db").unwrap();

        get_decks(&conn)
    }
}

impl Render for DeckListView {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let theme: &Theme = cx.global();

        div().mt_20().flex().justify_center().child(
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
                        .text_sm()
                        .child(div().px_2().min_w_80().child(format!("Deck")))
                        .child(
                            div()
                                .min_w_20()
                                .flex()
                                .justify_center()
                                .child(format!("New")),
                        )
                        .child(
                            div()
                                .min_w_20()
                                .flex()
                                .justify_center()
                                .child(format!("Learn")),
                        )
                        .child(
                            div()
                                .min_w_20()
                                .flex()
                                .justify_center()
                                .child(format!("Due")),
                        )
                        .pb_2()
                        .border_b_1()
                        .border_color(theme.crust)
                        .mb_2(),
                )
                .children(
                    self.get_all_decks()
                        .into_iter()
                        .map(|deck| HocListItem::init(cx.new_view(|_| ListItem::new(deck)).into()))
                        .collect::<Vec<_>>(),
                ),
        )
    }
}

pub struct ListItem {
    deck: Deck,
}

impl ListItem {
    pub fn new(deck: Deck) -> Self {
        Self { deck }
    }
}

impl Render for ListItem {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let deck_stat = self.deck.get_deck_stats();

        div()
            .flex()
            .w_full()
            .child(div().min_w_80().text_sm().child(self.deck.name.clone()))
            .child(
                div()
                    .min_w_20()
                    .flex()
                    .justify_center()
                    .text_color(theme.blue)
                    .child(format!("{}", deck_stat.new)),
            )
            .child(
                div()
                    .min_w_20()
                    .flex()
                    .justify_center()
                    .text_color(theme.red)
                    .child(format!("{}", deck_stat.learning)),
            )
            .child(
                div()
                    .min_w_20()
                    .flex()
                    .justify_center()
                    .text_color(theme.green)
                    .child(format!("{}", deck_stat.due)),
            )
    }
}

#[derive(IntoElement)]
pub struct HocListItem {
    inner: AnyView,
    selected: bool,
}

impl HocListItem {
    pub fn init(inner: AnyView) -> Self {
        Self {
            inner,
            selected: false,
        }
    }
}

impl RenderOnce for HocListItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let mut bg_hover = theme.mantle;
        bg_hover.fade_out(0.5);

        div()
            .flex()
            .hover(|s| s.bg(bg_hover))
            .p_2()
            .border_1()
            .rounded_xl()
            .child(self.inner)
    }
}
