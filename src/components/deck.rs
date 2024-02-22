use gpui::{div, prelude::*, MouseButton, Render, View, WindowContext};

pub mod deck_detail;
pub mod deck_list;

use crate::state::{StackableViewState, ViewState};

use self::deck_list::DeckListBuilder;

use super::shared::icon::Icon;

pub struct DeckView {
    state: StackableViewState,
}

impl DeckView {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|vc| {
            let state = StackableViewState::init(vc);
            state.push(DeckListBuilder {}, vc);
            Self { state }
        })
    }
}

impl Render for DeckView {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let view_stack: &Vec<ViewState> = self.state.model.read(cx).view_stack.as_ref();
        let current_view = view_stack.last().unwrap();

        let mut back = div();
        if view_stack.len() > 1 {
            back = div()
                .ml_2()
                .on_mouse_down(MouseButton::Left, move |_, cx| {
                    StackableViewState::update(|state, cx| state.pop(cx), cx);
                })
                .child(Icon::MoveLeft);
        }

        div()
            .size_full()
            .flex()
            .child(back)
            .child(current_view.view.clone())
    }
}
