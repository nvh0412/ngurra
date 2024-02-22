use gpui::{
    div, AnyView, FocusHandle, FontWeight, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, Pixels, Render, Styled, ViewContext, VisualContext, WindowContext,
};

use crate::{
    models::{
        builder::Builder,
        collection::{self, Collection, CollectionBuilder},
        queue::{self, Queue, QueueBuilder},
    },
    repositories::deck::DeckStat,
    state::{StackableView, StackableViewState},
    theme::Theme,
    ui::{button::button::Button, clickable::Clickable},
    Deck,
};

use super::flash_card::FlashCardBuilder;

pub struct DeckDetail {
    pub focus_handle: FocusHandle,
    pub deck_id: u32,
    pub queue: Queue,
}

impl DeckDetail {
    pub fn view(deck_id: u32, cx: &mut WindowContext) -> AnyView {
        let focus_handle = cx.focus_handle();
        let col = cx.global::<collection::Collection>();

        let mut queue_builder = QueueBuilder::new(deck_id);
        queue_builder.collect_cards(&col);
        let queue = queue_builder.build().unwrap();

        cx.new_view(|_vc| Self {
            deck_id,
            focus_handle,
            queue,
        })
        .into()
    }

    fn get_deck(&self, collection: &Collection) -> Deck {
        Deck::load(self.deck_id, &collection.storage.conn).unwrap()
    }

    fn key_down(&mut self, event: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        match event.keystroke.key.as_str() {
            "enter" | "space" => {
                StackableViewState::update(
                    |state, cx| {
                        state.push(
                            FlashCardBuilder {
                                card_queue: &self.queue,
                            },
                            cx,
                        )
                    },
                    cx,
                );

                cx.notify();
            }
            "backspace" => {
                StackableViewState::update(|state, cx| state.pop(cx), cx);
                cx.notify();
            }
            _ => {}
        };
    }
}

impl Render for DeckDetail {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.focus(&self.focus_handle);
        let theme = cx.global::<Theme>();
        let col = cx.global::<collection::Collection>();
        let mut deck = self.get_deck(col);

        let timing_at_stamp = CollectionBuilder::timing_for_timestamp(
            &col.storage.conn,
            chrono::Local::now().timestamp(),
        );

        let decks_stats =
            Deck::get_decks_stats(&col.storage.conn, timing_at_stamp.days_elapsed).unwrap();

        if let Some(st) = decks_stats.get(&deck.id.unwrap()) {
            deck.stats = Some(DeckStat {
                id: Some(deck.id.unwrap()),
                new: st.new,
                learning: st.learning,
                due: st.due,
            });
        }

        let stats = match deck.stats {
            Some(stats) => stats,
            None => DeckStat {
                id: Some(deck.id.unwrap()),
                new: 0,
                learning: 0,
                due: 0,
            },
        };

        let queue = self.queue.clone();

        let mut study_btn = Button::new("study-btn", "Study Now", None).on_click(move |_e, cx| {
            StackableViewState::update(
                |state, cx| state.push(FlashCardBuilder { card_queue: &queue }, cx),
                cx,
            );
        });

        study_btn.focus();

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .w_full()
            .flex_col()
            .pt_20()
            .text_color(theme.text)
            .on_key_down(cx.listener(Self::key_down))
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
                        .child(div().mt_5().flex().justify_center().child(study_btn)),
                ),
            )
    }
}

pub struct DeckDetailBuilder {
    pub deck_id: u32,
}

impl StackableView for DeckDetailBuilder {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        DeckDetail::view(self.deck_id, cx).into()
    }
}
