use gpui::{
    div, prelude::FluentBuilder, AnyElement, IntoElement, ParentElement, RenderOnce, Styled,
    WindowContext,
};

use smallvec::SmallVec;

use crate::{theme::Theme, ui::style::StyledExt};

#[derive(IntoElement)]
pub struct Popover {
    children: SmallVec<[AnyElement; 2]>,
    aside: Option<AnyElement>,
}

impl RenderOnce for Popover {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .gap_1()
            .child(div().flex().elevation_2(cx).px_1().children(self.children))
            .when_some(self.aside, |this, aside| {
                this.child(div().flex().flex_col().elevation_2(cx).px_1().child(aside))
            })
    }
}

impl Popover {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
            aside: None,
        }
    }
}

impl ParentElement for Popover {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
