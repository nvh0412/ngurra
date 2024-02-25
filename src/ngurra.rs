use catppuccin::Flavour;
use gpui::*;

use crate::{components::tab_panel::TabPanelBuilder, state::TabViewState, theme::Theme};

actions!(ngurra, [Hide]);

pub struct Ngurra {
    state: TabViewState,
}

impl Ngurra {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let view_state = TabViewState::init(TabPanelBuilder {}, cx);
            Self { state: view_state }
        })
    }
}

impl Render for Ngurra {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let current_view = &self.state;

        div()
            .flex()
            .size_full()
            .bg(theme.base)
            .font(theme.font_mono.clone())
            .child(current_view.tabbar.clone())
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .w(Pixels(800.0))
                            .h_full()
                            .child(current_view.view.clone()),
                    ),
            )
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
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |_ev, cx| {
                                Theme::change(Flavour::Latte, cx)
                            })
                            .child("Light"),
                    )
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |_ev, cx| {
                                Theme::change(Flavour::Mocha, cx)
                            })
                            .child("Dark"),
                    ),
            )
    }
}

pub fn init(cx: &mut AppContext) {
    cx.on_action(|_: &Hide, cx| cx.hide())
}
