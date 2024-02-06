use gpui::{div, rems, rgb, AnyElement, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled};

#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    children: Vec<AnyElement>
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new()
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
        const HEIGHT_IN_REMS: f32 = 29. / 16.;

        div()
            .id(self.id)
            .group("tab_bar")
            .flex()
            .border_1()
            .border_color(rgb(0xffffff))
            .h(rems(HEIGHT_IN_REMS))
            .text_color(rgb(0xffffff))
            .child(format!("Tab bar!"))
    }
}
