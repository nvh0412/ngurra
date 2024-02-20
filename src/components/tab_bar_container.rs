use std::{cell::RefCell, rc::Rc};

use gpui::{div, prelude::*, EventEmitter, Render, View, WindowContext};

use crate::theme::Theme;

use super::shared::icon::Icon;

#[derive(IntoElement, Clone)]
pub struct TabBarContainer {
    pub view: Rc<RefCell<View<TabBarView>>>,
}

pub struct TabBarView {}

impl TabBarContainer {
    pub fn new(cx: &mut WindowContext) -> Self {
        let view = TabBarView::init(cx);
        Self {
            view: Rc::new(RefCell::new(view)),
        }
    }
}

impl TabBarView {
    pub fn init(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| TabBarView {});

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
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {}
}

impl RenderOnce for TabBarContainer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let view_clone1 = Rc::clone(&self.view);
        let view_clone2 = Rc::clone(&self.view);
        let view_clone3 = Rc::clone(&self.view);

        div()
            .bg(theme.mantle)
            .border_t_1()
            .flex()
            .flex_col()
            .p_1()
            .child(
                div()
                    .group("tab_bar")
                    .px_2()
                    .py_2()
                    .text_color(theme.text)
                    .child(Icon::BookText)
                    .on_mouse_down(gpui::MouseButton::Left, move |_ev, cx| {
                        view_clone1.borrow_mut().update(cx, |e, cx| {
                            cx.emit(TabEvent::Deck);
                            cx.notify();
                        })
                    }),
            )
            .child(
                div()
                    .group("tab_bar")
                    .px_2()
                    .py_2()
                    .text_color(theme.text)
                    .child(Icon::FilePlus)
                    .on_mouse_down(gpui::MouseButton::Left, move |_ev, cx| {
                        view_clone2.borrow_mut().update(cx, |e, cx| {
                            cx.emit(TabEvent::Add);
                            cx.notify();
                        })
                    }),
            )
            .child(
                div()
                    .group("tab_bar")
                    .px_2()
                    .py_2()
                    .child(Icon::FileSearch)
                    .on_mouse_down(gpui::MouseButton::Left, move |_ev, cx| {
                        view_clone3.borrow_mut().update(cx, |e, cx| {
                            cx.emit(TabEvent::Browse);
                            cx.notify();
                        })
                    }),
            )
    }
}
