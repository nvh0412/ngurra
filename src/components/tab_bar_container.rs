use gpui::{div, prelude::*, EventEmitter, Render, View, WindowContext};

use crate::theme::Theme;

use self::tab_bar::TabBar;

mod tab_bar;

#[derive(IntoElement, Clone)]
pub struct TabBarContainer {
    pub view: View<TabBarView>,
}

pub struct TabBarView {
}

impl TabBarContainer {
    pub fn new(cx: &mut WindowContext) -> Self {
        let view = TabBarView::init(cx);
        Self { view }
    }
}

impl TabBarView {
    pub fn init(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            TabBarView {}
        });

        view
    }
}

pub enum TabEvent {
    Deck,
    Add,
    Browse,
}

impl EventEmitter<TabEvent> for TabBarView {}

impl Render for TabBarView {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .w_full()
            .flex()
            .justify_center()
            .border_t_1()
            .border_color(theme.crust)
            .child(
                div()
                    .bg(theme.mantle)
                    .flex()
                    .child(TabBar::new("deck", String::from("Deck")))
                    .child(TabBar::new("add", String::from("Add")))
                    .child(TabBar::new("browse", String::from("Browse")))
            )
    }
}

impl RenderOnce for TabBarContainer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        self.view
    }
}
