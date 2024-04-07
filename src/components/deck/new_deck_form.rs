use gpui::{
    div, AnyView, ClickEvent, FontWeight, ParentElement, Pixels, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::{
    repositories::{self},
    state::{StackableView, StackableViewState},
    theme::Theme,
    ui::{button::button::Button, clickable::Clickable, text_field::text_field::TextField},
};

struct NewDeckForm {
    text_input: TextField,
}

impl NewDeckForm {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx: &mut gpui::ViewContext<'_, NewDeckForm>| Self {
            text_input: TextField::new(cx, "Deck Name".to_string()),
        })
    }

    fn save_click(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();
        let text = &self.text_input.view.read(&cx).text;

        let mut deck = repositories::deck::Deck::new(text);
        match deck.save(&collection.storage.conn) {
            Ok(_) => {
                StackableViewState::update(|state, cx| state.pop(cx), cx);
                cx.notify();
            }
            Err(e) => {
                log::error!("Error saving deck: {:?}", e);
            }
        }
    }
}

impl Render for NewDeckForm {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let theme = cx.global::<Theme>();

        div().flex().size_full().justify_center().child(
            div().mt_20().child(
                div()
                    .flex()
                    .w_full()
                    .flex_col()
                    .text_color(theme.text)
                    .relative()
                    .h_full()
                    .child(
                        div()
                            .w(Pixels(500.0))
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .pb_5()
                                    .border_b_1()
                                    .border_color(theme.crust)
                                    .child("New Deck"),
                            )
                            .child(div().mt_6().child(self.text_input.clone()))
                            .child(
                                div()
                                    .mt_6()
                                    .justify_end()
                                    .flex()
                                    .child(
                                        Button::new("btn-save", "Create", None)
                                            .on_click(cx.listener(Self::save_click)),
                                    )
                                    .child(div().ml_2().child(
                                        Button::new("btn-cancel", "Cancel", None).on_click(
                                            |event, _cx| {
                                                log::info!("Button clicked: {:?}", event);
                                            },
                                        ),
                                    )),
                            ),
                    ),
            ),
        )
    }
}

pub struct NewDeckFormBuilder {}

impl StackableView for NewDeckFormBuilder {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        NewDeckForm::view(cx).into()
    }
}
