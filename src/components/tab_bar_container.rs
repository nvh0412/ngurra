use std::{cell::RefCell, rc::Rc};

use gpui::{div, prelude::*, EventEmitter, Render, StyleRefinement, View, WindowContext};

use crate::theme::Theme;

use super::shared::icon::Icon;

#[derive(IntoElement, Clone)]
pub struct TabBarContainer {
    pub view: Rc<RefCell<View<TabBarView>>>,
    pub selected: Rc<RefCell<TabEvent>>,
}

pub struct TabBarView {}

impl TabBarContainer {
    pub fn new(cx: &mut WindowContext) -> Self {
        let view = TabBarView::init(cx);
        Self {
            view: Rc::new(RefCell::new(view)),
            selected: Rc::new(RefCell::new(TabEvent::Deck)),
        }
    }
}

impl TabBarView {
    pub fn init(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| TabBarView {});

        view
    }
}

#[derive(PartialEq)]
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

        let bg_hover = theme.overlay0;
        let selected_deck_clone = Rc::clone(&self.selected);
        let selected_add_clone = Rc::clone(&self.selected);
        let selected_browse_clone = Rc::clone(&self.selected);

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
                    .hover(|s| s.rounded_md().bg(bg_hover))
                    .text_color(theme.text)
                    .child(Icon::BookText)
                    .on_mouse_down(gpui::MouseButton::Left, move |_ev, cx| {
                        view_clone1.borrow_mut().update(cx, |_e, cx| {
                            cx.emit(TabEvent::Deck);
                            cx.notify();
                            *selected_deck_clone.borrow_mut() = TabEvent::Deck;
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
                    .hover(|s| s.rounded_md().bg(bg_hover))
                    .on_mouse_down(gpui::MouseButton::Left, move |_ev, cx| {
                        view_clone2.borrow_mut().update(cx, |_e, cx| {
                            cx.emit(TabEvent::Add);
                            cx.notify();
                            *selected_add_clone.borrow_mut() = TabEvent::Add;
                        })
                    }),
            )
            .child(
                div()
                    .group("tab_bar")
                    .px_2()
                    .py_2()
                    .hover(|s| s.rounded_md().bg(bg_hover))
                    .child(Icon::FileSearch)
                    .on_mouse_down(gpui::MouseButton::Left, move |_ev, cx| {
                        view_clone3.borrow_mut().update(cx, |_e, cx| {
                            cx.emit(TabEvent::Browse);
                            cx.notify();
                            *selected_browse_clone.borrow_mut() = TabEvent::Browse;
                        })
                    }),
            )
    }
}
