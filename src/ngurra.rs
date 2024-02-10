use std::env::current_dir;

use catppuccin::Flavour;
use gpui::*;

use crate::{state::{AppState, ViewState}, theme::Theme};

actions!(
    ngurra,
    [
        Hide
    ]
);

pub struct Ngurra {
    state: AppState
}

impl Ngurra {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let app_state = AppState::init(cx);
            cx.set_global(app_state.clone());
            Self { state: app_state }
        })
    }
}

impl Render for Ngurra {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let view_stack: &Vec<ViewState> = self.state.model.read(cx).view_stack.as_ref();
        let current_view = view_stack.last().unwrap();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.base)
            .font(theme.font_mono.clone())
            .child(current_view.tabbar.clone())
            .child(current_view.view.clone())
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
