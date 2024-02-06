use gpui::{div, prelude::*, Render, View, WindowContext};

use self::deck_list::DeckList;

mod deck_list;

pub struct DeckListContainer;

impl DeckListContainer {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for DeckListContainer {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .mt_20()
            .flex()
            .justify_center()
            .child(DeckList::view(cx))
    }
}
