use gpui::{
    AppContext, FocusHandle, FocusableView, IntoElement, Pixels, Point, Render, ViewContext,
    ViewInputHandler,
};

use super::element::EditorElement;

pub enum EditorMode {
    SingleLine,
    AutoHeight,
}

pub struct Editor {
    pub mode: EditorMode,
    focus_handle: FocusHandle,
    pub pixel_position_of_newest_cursor: Option<Point<Pixels>>,
}

impl Editor {
    pub fn single_line(cx: &mut ViewContext<Self>) -> Editor {
        Editor {
            mode: EditorMode::SingleLine,
            focus_handle: cx.focus_handle(),
            pixel_position_of_newest_cursor: None,
        }
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
        todo!()
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
