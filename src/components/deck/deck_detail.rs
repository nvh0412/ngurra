use gpui::{
    div, AnyView, FontWeight, IntoElement, ParentElement, Pixels, Render, Styled, ViewContext,
    VisualContext, WindowContext,
};


use crate::{
    models::collection::{self, Collection}, state::{StackableView, StackableViewState}, theme::Theme, ui::{button::button::Button, clickable::Clickable}, Deck, FlashCard
};

use super::flash_card::FlashCardBuilder;

pub struct DeckDetail {
    pub deck_id: i32,
}

impl DeckDetail {
    pub fn view(deck_id: i32, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|_vc| Self { deck_id }).into()
    }

    fn get_deck(&self, collection: &Collection) -> Deck {
        let mut deck = Deck::load(self.deck_id, &collection.storage.conn).unwrap();
        deck.cards = FlashCard::get_all_cards_in_deck(deck.id.unwrap(), &collection.storage.conn, 10).unwrap();
        deck
    }
}

impl Render for DeckDetail {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let collection = cx.global::<collection::Collection>();
        let deck = self.get_deck(collection);

        let stats = deck.get_deck_stats();

        div()
            .flex()
            .w_full()
            .flex_col()
            .pt_20()
            .text_color(theme.text)
            .child(
                div()
                    .flex()
                    .justify_center()
                    .text_xl()
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .child(deck.name),
            )
            .child(
                div().size_full().flex().justify_center().child(
                    div()
                        .mt_10()
                        .w(Pixels(300.0))
                        .justify_center()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .border_1()
                                .border_color(theme.crust)
                                .p_4()
                                .rounded_xl()
                                .justify_center()
                                .text_sm()
                                .child(
                                    div().flex().justify_between().child("New").child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.blue)
                                            .child(format!("{}", stats.new)),
                                    ),
                                )
                                .child(
                                    div().flex().justify_between().child("Learning").child(
                                        div()
                                            .text_xl()
                                            .text_color(theme.red)
                                            .font_weight(FontWeight::BOLD)
                                            .child(format!("{}", stats.learning)),
                                    ),
                                )
                                .child(
                                    div().flex().justify_between().child("To review").child(
                                        div()
                                            .text_xl()
                                            .text_color(theme.green)
                                            .font_weight(FontWeight::BOLD)
                                            .child(format!("{}", stats.due)),
                                    ),
                                ),
                        )
                        .child(div().mt_5().flex().justify_center().child(
                            Button::new("study-btn", "Study Now").on_click(move |_e, cx| {
                                StackableViewState::update(
                                    |state, cx| {
                                        state.push(FlashCardBuilder { cards: &deck.cards }, cx)
                                    },
                                    cx,
                                );
                            }),
                        )),
                ),
            )
    }
}

pub struct DeckDetailBuilder {
    pub deck_id: i32,
}

impl StackableView for DeckDetailBuilder {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        DeckDetail::view(self.deck_id, cx).into()
    }
}
