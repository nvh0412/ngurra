use gpui::{
    div, Div, ElementId, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
};

use crate::{
    components::shared::icon::Icon,
    theme::Theme,
    ui::{clickable::Clickable, disableable::Disableable, selectable::Selectable},
};

use super::{button_base::ButtonBase, button_common::ButtonCommon};

#[derive(IntoElement)]
pub struct Button {
    pub label: SharedString,
    pub base: ButtonBase,
    pub icon: Option<Icon>,
    pub selected: bool,
}

impl Button {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        icon: Option<Icon>,
    ) -> Self {
        Self {
            label: label.into(),
            base: ButtonBase::new(id),
            icon,
            selected: false,
        }
    }
}

impl Clickable for Button {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut gpui::WindowContext) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}

impl Selectable for Button {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl ButtonCommon for Button {
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    fn tooltip(
        mut self,
        tooltip: impl Fn(&mut gpui::WindowContext) -> gpui::AnyView + 'static,
    ) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut gpui::WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let is_disabled = self.base.disabled;

        let text_color = if is_disabled {
            theme.text_disabled
        } else {
            theme.text
        };

        if let Some(icon) = self.icon {
            self.base.child(
                div().flex().flex_row().px_3().py_2().child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(icon),
                ),
            )
        } else {
            self.base.child(
                div()
                    .flex()
                    .flex_row()
                    .border_1()
                    .border_color(theme.crust)
                    .items_center()
                    .rounded_lg()
                    .px_3()
                    .py_2()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .text_color(text_color)
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .child(self.label),
                    ),
            )
        }
    }
}
