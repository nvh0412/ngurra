use crate::{prelude::*, theme::Theme, ui::style::StyledExt};
use gpui::{
    div, px, Action, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle,
    FocusableView, IntoElement, Render, SharedString, Subscription, View, ViewContext,
    VisualContext, WindowContext,
};
use std::{rc::Rc, time::Duration};

use super::list::{list::List, list_item::ListItem};

enum ContextMenuItem {
    Entry {
        label: SharedString,
        handler: Rc<dyn Fn(&mut WindowContext)>,
        action: Option<Box<dyn Action>>,
    },
}

pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    focus_handle: FocusHandle,
    selected_index: Option<usize>,
    delayed: bool,
    clicked: bool,
    _on_blur_subscription: Subscription,
}

impl FocusableView for ContextMenu {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ContextMenu {}

impl FluentBuilder for ContextMenu {}

impl ContextMenu {
    pub fn build(
        cx: &mut WindowContext,
        f: impl FnOnce(Self, &mut WindowContext) -> Self,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let focus_handle = cx.focus_handle();
            let _on_blur_subscription =
                cx.on_blur(&focus_handle, |this: &mut ContextMenu, cx| this.cancel(cx));
            cx.refresh();
            f(
                Self {
                    items: Default::default(),
                    focus_handle,
                    selected_index: None,
                    delayed: false,
                    clicked: false,
                    _on_blur_subscription,
                },
                cx,
            )
        })
    }

    pub fn entry(
        mut self,
        label: impl Into<SharedString>,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut WindowContext) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry {
            label: label.into(),
            handler: Rc::new(handler),
            action,
        });
        self
    }

    pub fn cancel(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
        cx.emit(DismissEvent);
    }

    pub fn on_action_dispatch(&mut self, dispatched: &Box<dyn Action>, cx: &mut ViewContext<Self>) {
        if self.clicked {
            cx.propagate();
            return;
        }

        if let Some(ix) = self.items.iter().position(|item| {
            if let ContextMenuItem::Entry {
                action: Some(action),
                ..
            } = item
            {
                action.partial_eq(&**dispatched)
            } else {
                false
            }
        }) {
            self.selected_index = Some(ix);
            self.delayed = true;
            cx.notify();
            let action = dispatched.boxed_clone();
            cx.spawn(|this, mut cx| async move {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                this.update(&mut cx, |this, cx| {
                    this.cancel(cx);
                    cx.dispatch_action(action);
                })
            })
            .detach_and_log_err(cx);
        } else {
            cx.propagate()
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().elevation_2(cx).flex().flex_row().child(
            div()
                .flex()
                .flex_col()
                .min_w(px(100.))
                .track_focus(&self.focus_handle)
                .on_mouse_down_out(cx.listener(|this, _, cx| this.cancel(cx)))
                .key_context("menu")
                .when(!self.delayed, |mut el| {
                    for item in self.items.iter() {
                        if let ContextMenuItem::Entry {
                            action: Some(action),
                            ..
                        } = item
                        {
                            el = el.on_boxed_action(
                                &**action,
                                cx.listener(ContextMenu::on_action_dispatch),
                            );
                        }
                    }
                    el
                })
                .flex_none()
                .child(List::new().children(self.items.iter_mut().enumerate().map(
                    |(ix, item)| match item {
                        ContextMenuItem::Entry {
                            label,
                            handler,
                            action,
                        } => {
                            let handler = handler.clone();
                            let menu = cx.view().downgrade();

                            let label_element = label.clone();

                            ListItem::new(ix)
                                .inset(true)
                                .on_click(move |_, cx| {
                                    handler(cx);
                                    menu.update(cx, |menu, cx| {
                                        menu.clicked = true;
                                        cx.emit(DismissEvent);
                                    })
                                    .ok();
                                })
                                .child(
                                    div()
                                        .flex()
                                        .w_full()
                                        .justify_between()
                                        .text_sm()
                                        .child(label_element)
                                        .debug_selector(|| format!("MENU_ITEM-{}", label)),
                                )
                                .into_any_element()
                        }
                    },
                ))),
        )
    }
}
