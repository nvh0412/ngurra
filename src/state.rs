use gpui::{AnyView, Context, Global, Model, WindowContext};

use crate::components::tab_bar_container::TabBarContainer;

pub struct TabViewState {
    pub tabbar: TabBarContainer,
    pub view: AnyView,
}

impl TabViewState {
    pub fn init(view: impl TabView, cx: &mut WindowContext) -> Self {
        let tabbar = TabBarContainer::new(cx);

        Self {
            view: view.build(&tabbar, cx),
            tabbar,
        }
    }
}

pub trait TabView {
    fn build(&self, tabbar: &TabBarContainer, cx: &mut WindowContext) -> AnyView;
}

pub struct ViewState {
    pub view: AnyView,
}

impl ViewState {
    pub fn init(view: impl StackableView, cx: &mut WindowContext) -> Self {
        Self {
            view: view.build(cx),
        }
    }
}

pub struct State {
    pub view_stack: Vec<ViewState>,
}

#[derive(Clone)]
pub struct StackableViewState {
    pub model: Model<State>,
}

impl Global for StackableViewState {}

pub trait StackableView {
    fn build(&self, cx: &mut WindowContext) -> AnyView;
}

impl StackableViewState {
    pub fn init(cx: &mut WindowContext) -> Self {
        let state = StackableViewState {
            model: cx.new_model(|_| State { view_stack: vec![] }),
        };
        cx.set_global(state.clone());
        state
    }

    pub fn push(&self, view: impl StackableView, cx: &mut WindowContext) {
        let view_state = ViewState::init(view, cx);
        self.model.update(cx, |model, cx| {
            model.view_stack.push(view_state);
            cx.notify();
        });
    }

    pub fn pop(&self, cx: &mut WindowContext) {
        self.model.update(cx, |model, cx| {
            if model.view_stack.len() > 1 {
                model.view_stack.pop();
                cx.notify();
            }
        });
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        cx.update_global(|state, cx| f(state, cx))
    }
}
