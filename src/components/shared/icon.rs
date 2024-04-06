use core::fmt;

use gpui::{div, svg, IntoElement, ParentElement, RenderOnce, SharedString, Styled};

use crate::theme::Theme;

#[derive(Debug, IntoElement)]
pub enum Icon {
    BookText,
    FilePlus,
    Settings,
    FileSearch,
    MoveLeft,
}

fn to_kebap(s: &str) -> String {
    s.chars().fold(String::new(), |mut s, c| {
        if c.is_uppercase() || c.is_numeric() {
            if !s.is_empty() {
                s.push('-');
            }
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c);
        }
        s
    })
}

impl Icon {
    pub fn path(&self) -> SharedString {
        let binding = self.to_string();
        let name = to_kebap(binding.as_str());
        SharedString::from(format!("icons/{}.svg", name))
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl RenderOnce for Icon {
    fn render(self, cx: &mut gpui::WindowContext) -> impl gpui::prelude::IntoElement {
        let theme = cx.global::<Theme>();
        let svg = svg().path(self.path()).text_color(theme.text).size_full();
        let img = svg.into_any_element();

        div()
            .overflow_hidden()
            .flex()
            .items_center()
            .justify_center()
            .size_4()
            .child(img)
    }
}
