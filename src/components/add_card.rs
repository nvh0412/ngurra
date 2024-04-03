use gpui::{div, prelude::*, Pixels, Render, View, WindowContext};

use crate::{
    theme::Theme,
    ui::{button::button::Button, clickable::Clickable, text_field::text_field::TextField},
};

pub struct AddCardView {
    front_input: TextField,
    back_input: TextField,
    deck_input: TextField,
}

impl AddCardView {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx: &mut gpui::ViewContext<'_, AddCardView>| Self {
            front_input: TextField::new(cx, "".to_string()),
            back_input: TextField::new(cx, "".to_string()),
            deck_input: TextField::new(cx, "".to_string()),
        })
    }
}

impl Render for AddCardView {
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
                                    .child(
                                        div()
                                            .mt_5()
                                            .flex()
                                            .justify_end()
                                            .child(Button::new("create", "Create card")),
                                    ),
                            ),
                    ),
            ),
        )
    }
}
