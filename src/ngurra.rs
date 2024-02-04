use gpui::*;

actions!(
    ngurra,
    [
        Hide
    ]
);

pub struct Ngurra {
    text: SharedString
}

impl Ngurra {
    pub fn new() -> Self {
        Ngurra {
            text: "abc".into()
        }
    }
}

impl Render for Ngurra {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child("Hello, World!")
    }
}

pub fn init(cx: &mut AppContext) {
}
