use gpui::{
    div, AnyView, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext,
    WindowContext,
};

use crate::state::StackableView;

pub struct DeckDetail;

impl DeckDetail {
    pub fn view(cx: &mut WindowContext) -> AnyView {
        cx.new_view(|vc| Self).into()
    }
}

impl StackableView for DeckDetail {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        DeckDetail::view(cx).into()
    }
}

impl Render for DeckDetail {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child("1231232")
    }
}

pub struct DeckDetailBuilder;
impl StackableView for DeckDetailBuilder {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        DeckDetail::view(cx).into()
    }
}
