use gpui::{div, prelude::*, Render, View, WindowContext};

use crate::ui::{button::button::Button, clickable::Clickable, text_field::text_field::TextField};

pub struct AddCardView {
    text_input: TextField,
}

impl AddCardView {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx: &mut gpui::ViewContext<'_, AddCardView>| Self {
            text_input: TextField::new(cx, "Add a card".to_string()),
        })
    }
}

impl Render for AddCardView {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .flex()
            .justify_center()
            .child(self.text_input.clone())
            .child(Button::new("", "Click me").on_click(|event, cx| {
                log::info!("Button clicked: {:?}", event);
            }))
    }
}
