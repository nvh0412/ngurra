
use gpui::{AnyView, Context, Global, Model, WindowContext};

use crate::components::{tab_bar_container::TabBarContainer, tab_panel::TabPanelBuilder};

pub struct ViewState {
    pub tabbar: TabBarContainer,
    pub view: AnyView
}

pub struct State {
    pub view_stack: Vec<ViewState>
}

impl ViewState {
    fn init(view: impl StateView, cx: &mut WindowContext) -> Self {
        let tabbar = TabBarContainer::new(cx);

        Self { view: view.build(&tabbar, cx), tabbar }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub model: Model<State>
}

impl Global for AppState {}

pub trait StateView {
    fn build(
        &self,
        tabbar: &TabBarContainer,
        cx: &mut WindowContext
    ) -> AnyView;
}

impl AppState {
    pub fn init(cx: &mut WindowContext) -> Self {
        let app_state = AppState {
            model: cx.new_model(|_| State { view_stack: vec![] })
        };

        app_state.push(TabPanelBuilder {}, cx);

        app_state
    }

    pub fn push(&self, view: impl StateView, cx: &mut WindowContext) {
        let view_state = ViewState::init(view, cx);
        self.model.update(cx, |model, cx| {
            model.view_stack.push(view_state);
            cx.notify();
        });
    }
}
