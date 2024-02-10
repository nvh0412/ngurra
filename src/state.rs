
use gpui::{AnyView, Context, Global, Model, WindowContext};

use crate::components::deck_list_container::DeckListContainer;

pub struct ViewState {
    pub view: AnyView
}

pub struct State {
    pub view_stack: Vec<ViewState>
}

impl ViewState {
    fn init(view: AnyView) -> Self {
        Self { view: view }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SupportedTab {
    Deck,
    Add,
    Browser
}

#[derive(Clone)]
pub struct AppState {
    pub model: Model<State>
}

impl Global for AppState {}

impl AppState {
    pub fn init(cx: &mut WindowContext) -> Self {
        let app_state = AppState {
            model: cx.new_model(|_| State { view_stack: vec![] })
        };

        app_state.push(DeckListContainer::view(cx).into(), cx);

        app_state
    }

    pub fn push(&self, view: AnyView, cx: &mut WindowContext) {
        let view_state = ViewState::init(view);
        self.model.update(cx, |model, cx| {
            println!("Push a new view");
            model.view_stack.push(view_state);
            cx.notify();
        });
    }
}
