use gpui::{div, rems, rgb, AnyElement, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled};

use crate::theme::{self, Theme};

#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    children: Vec<AnyElement>,
    text: String
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>, text: String) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            text
        }
    }
}

impl ParentElement for TabBar {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for TabBar {
    fn render(self, cx: &mut gpui::WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id(self.id)
            .group("tab_bar")
            .flex()
            .border_1()
            .border_color(theme.crust)
            .px_4()
            .py_2()
            .text_color(theme.text)
            .child(self.text)
    }
}
