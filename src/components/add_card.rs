use gpui::{
    div, prelude::*, AnyView, ClickEvent, EventEmitter, FocusHandle, Pixels, Render, View,
    ViewContext, WindowContext,
};

use crate::{
    components::tab_bar_container::TabEvent,
    repositories,
    state::{StackableView, StackableViewState},
    theme::Theme,
    ui::{button::button::Button, clickable::Clickable, text_field::text_field::TextField},
    Deck,
};

pub struct AddCardView {
    deck: Deck,
    front_input: TextField,
    back_input: TextField,
    deck_input: TextField,
    focus_handle: FocusHandle,
    focused_at: usize,
}

impl EventEmitter<TabEvent> for AddCardView {}

const TABBALE_FIELDS: [&str; 3] = ["front", "back", "submit"];

impl AddCardView {
    pub fn view(deck_id: u32, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx: &mut gpui::ViewContext<'_, AddCardView>| {
            let deck = repositories::deck::Deck::load(
                deck_id,
                &cx.global::<crate::Collection>().storage.conn,
            )
            .unwrap();

            let deck_input = TextField::new(cx, "".to_string(), true);

            deck_input.view.update(cx, |view, _| {
                view.text = deck.name.clone();
            });

            let front_input = TextField::new(cx, "".to_string(), false);
            front_input.focus(cx);

            let focus_handle = cx.focus_handle();

            Self {
                deck,
                front_input,
                back_input: TextField::new(cx, "".to_string(), false),
                deck_input,
                focused_at: 0,
                focus_handle,
            }
        })
    }

    fn save_click(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();
        let front = &self.front_input.view.read(&cx).text;
        let back = &self.back_input.view.read(&cx).text;

        let mut card =
            repositories::flash_card::FlashCard::new(self.deck.id.unwrap(), front, back, None);
        match card.save(&collection.storage.conn) {
            Ok(_) => {
                StackableViewState::update(|state, cx| state.pop(cx), cx);
            }
            Err(e) => {
                log::error!("Error saving card: {:?}", e);
            }
        }
    }

    fn save_key_down(&mut self, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();
        let front = &self.front_input.view.read(&cx).text;
        let back = &self.back_input.view.read(&cx).text;

        let mut card =
            repositories::flash_card::FlashCard::new(self.deck.id.unwrap(), front, back, None);
        match card.save(&collection.storage.conn) {
            Ok(_) => {
                StackableViewState::update(|state, cx| state.pop(cx), cx);
                cx.notify();
            }
            Err(e) => {
                log::error!("Error saving card: {:?}", e);
            }
        }
    }
}

impl Render for AddCardView {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let view = cx.view().clone();
        let theme = cx.global::<Theme>();

        let mut submit_btn =
            Button::new("create", "Create card", None).on_click(cx.listener(Self::save_click));

        let focused_at = self.focused_at;
        let front_input = self.front_input.clone();
        let back_input = self.back_input.clone();

        if self.focused_at == 2 {
            submit_btn.focus();
        }

        div().flex().size_full().justify_center().child(
            div().mt_20().child(
                div()
                    .track_focus(&self.focus_handle)
                    .flex()
                    .w_full()
                    .flex_col()
                    .text_color(theme.text)
                    .relative()
                    .h_full()
                    .on_key_down(move |event, wc| {
                        view.update(wc, |add_view, vc| {
                            let keystroke = &event.keystroke.key;

                            match keystroke.as_str() {
                                "tab" => {
                                    let next = (focused_at + 1) % TABBALE_FIELDS.len();

                                    match TABBALE_FIELDS[next] {
                                        "front" => front_input.focus(vc),
                                        "back" => back_input.focus(vc),
                                        "submit" => vc.focus(&add_view.focus_handle),
                                        _ => {}
                                    }
                                    add_view.focused_at = next;
                                }
                                "enter" => {
                                    add_view.save_key_down(vc);
                                }
                                _ => {}
                            }
                        });
                    })
                    .child(
                        div()
                            .w(Pixels(500.0))
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(gpui::FontWeight::EXTRA_BOLD)
                                    .child("Add a new card"),
                            )
                            .child(
                                div()
                                    .mt_5()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .child("Deck"),
                                    )
                                    .child(self.deck_input.clone())
                                    .child(
                                        div()
                                            .mt_5()
                                            .text_lg()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .child("Front"),
                                    )
                                    .child(self.front_input.clone())
                                    .child(
                                        div()
                                            .mt_5()
                                            .text_lg()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .child("Back"),
                                    )
                                    .child(self.back_input.clone())
                                    .child(div().mt_5().flex().justify_end().child(submit_btn)),
                            ),
                    ),
            ),
        )
    }
}

pub struct AddCardBuilder {
    pub deck_id: u32,
}

impl StackableView for AddCardBuilder {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        AddCardView::view(self.deck_id, cx).into()
    }
}
