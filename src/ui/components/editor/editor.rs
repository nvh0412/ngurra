use std::time::Duration;

use gpui::{
    AppContext, Context, FocusHandle, FocusableView, IntoElement, Model, Pixels, Point, Render,
    Subscription, ViewContext, ViewInputHandler,
};

use super::{blink_manager::BlinkManager, element::EditorElement};

const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);

pub enum EditorMode {
    SingleLine,
    AutoHeight,
}

pub struct Editor {
    pub mode: EditorMode,
    focus_handle: FocusHandle,
    blink_manager: Model<BlinkManager>,
    pub pixel_position_of_newest_cursor: Option<Point<Pixels>>,
    _subscriptions: Vec<Subscription>,
}

impl Editor {
    pub fn single_line(cx: &mut ViewContext<Self>) -> Editor {
        let blink_manager = cx.new_model(|cx| BlinkManager::new(CURSOR_BLINK_INTERVAL, cx));

        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, Self::handle_focus).detach();

        Editor {
            mode: EditorMode::SingleLine,
            focus_handle: cx.focus_handle(),
            pixel_position_of_newest_cursor: None,
            blink_manager: blink_manager.clone(),
            _subscriptions: vec![
                cx.observe(&blink_manager, |_, _, cx| cx.notify()),
                cx.observe_window_activation(|e, cx| {
                    let active = cx.is_window_active();
                    e.blink_manager.update(cx, |b, cx| {
                        if active {
                            b.enable(cx);
                        } else {
                            b.show_cursor(cx);
                        }
                    });
                }),
            ],
        }
    }

    pub fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.blink_manager.update(cx, BlinkManager::enable);
    }
}

impl Render for Editor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        EditorElement::new(cx.view())
    }
}

impl FocusableView for Editor {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ViewInputHandler for Editor {
    fn text_for_range(
        &mut self,
        range: std::ops::Range<usize>,
        cx: &mut ViewContext<Self>,
    ) -> Option<String> {
        todo!()
    }

    fn selected_text_range(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> Option<std::ops::Range<usize>> {
        todo!()
    }

    fn marked_text_range(&self, cx: &mut ViewContext<Self>) -> Option<std::ops::Range<usize>> {
        todo!()
    }

    fn unmark_text(&mut self, cx: &mut ViewContext<Self>) {
        todo!()
    }

    fn replace_text_in_range(
        &mut self,
        range: Option<std::ops::Range<usize>>,
        text: &str,
        cx: &mut ViewContext<Self>,
    ) {
        // self.handle_input(text, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<std::ops::Range<usize>>,
        new_text: &str,
        new_selected_range: Option<std::ops::Range<usize>>,
        cx: &mut ViewContext<Self>,
    ) {
        todo!()
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: std::ops::Range<usize>,
        element_bounds: gpui::Bounds<gpui::Pixels>,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::Bounds<gpui::Pixels>> {
        todo!()
    }
}
