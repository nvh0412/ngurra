use gpui::{div, prelude::*, Render, View, WindowContext};

use crate::ui::{button::button::Button, clickable::Clickable, editor::editor::Editor};

pub struct AddCardView;

impl AddCardView {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for AddCardView {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .mt_20()
            .flex()
            .justify_center()
            .child(cx.new_view(|cx| Editor::single_line(cx)))
            .child(Button::new("", "Click me").on_click(|event, cx| {
                log::info!("Button clicked: {:?}", event);
            }))
    }
}
