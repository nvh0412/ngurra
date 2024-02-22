use gpui::{
    div, prelude::FluentBuilder, AnyElement, AnyView, ClickEvent, DefiniteLength, Div, ElementId,
    InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce,
    StatefulInteractiveElement, WindowContext,
};

use crate::ui::{clickable::Clickable, disableable::Disableable};

use super::button_common::ButtonCommon;

#[derive(IntoElement)]
pub struct ButtonBase {
    id: ElementId,
    base: Div,
    pub(super) disabled: bool,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    children: Vec<AnyElement>,
}

impl ButtonBase {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            base: div(),
            disabled: false,
            tooltip: None,
            on_click: None,
            children: Vec::new(),
        }
    }
}

pub enum ButtonBaseRounding {
    All,
}

impl Disableable for ButtonBase {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Clickable for ButtonBase {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ButtonCommon for ButtonBase {
    fn id(&self) -> &ElementId {
        &self.id
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl RenderOnce for ButtonBase {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .id(self.id)
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, |_, cx| cx.prevent_default())
                        .on_click(move |event, cx| {
                            cx.stop_propagation();
                            (on_click)(event, cx);
                        })
                },
            )
            .children(self.children)
    }
}

impl ParentElement for ButtonBase {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
