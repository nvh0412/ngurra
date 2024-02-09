use catppuccin::Flavour;
use gpui::*;

use crate::theme::Theme;

actions!(
    ngurra,
    [
        Hide
    ]
);

pub struct Ngurra {
    view: AnyView
}

impl Ngurra {
    pub fn new(view: AnyView) -> Self {
        Self { view }
    }
}

impl Render for Ngurra {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.base)
            .font(theme.font_mono.clone())
            .child(self.view.clone())
            .child(
                div()
                    .absolute()
                    .w_full()
                    .bottom_0()
                    .left_0()
                    .right_0()
                    .bg(theme.mantle)
                    .border_t_1()
                    .border_color(theme.crust)
                    .px_4()
                    .py_2()
                    .text_color(theme.text)
                    .flex()
                    .text_xs()
                    .child(
                        div().mr_2().on_mouse_down(MouseButton::Left, |_ev, cx| {
                            Theme::change(Flavour::Latte, cx)
                        }).child("Latte")
                    )
                    .child(
                        div().mr_2().on_mouse_down(MouseButton::Left, |_ev, cx| {
                            Theme::change(Flavour::Mocha, cx)
                        }).child("Mocha")
                    )
                    .child(
                        div().mr_2().on_mouse_down(MouseButton::Left, |_ev, cx| {
                            Theme::change(Flavour::Frappe, cx)
                        }).child("Frappe")
                    )
            )
    }
}

pub fn init(cx: &mut AppContext) {
    cx.on_action(|_: &Hide, cx| cx.hide() )
}
