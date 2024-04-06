use gpui::{rems, IntoElement, Pixels, Rems, Styled, WindowContext};

use crate::theme::Theme;

use super::elevation::ElevationIndex;

fn elevated<E: Styled>(this: E, cx: &mut WindowContext, index: ElevationIndex) -> E {
    let theme = cx.global::<Theme>();

    this.bg(theme.base)
        .rounded(px(8.))
        .border()
        .border_color(theme.crust)
        .shadow(index.shadow())
}

pub const fn px(pixels: f32) -> Pixels {
    Pixels(pixels)
}

pub trait StyledExt: Styled + Sized {
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    fn elevation_1(self, cx: &mut WindowContext) -> Self {
        elevated(self, cx, ElevationIndex::Surface)
    }

    fn elevation_2(self, cx: &mut WindowContext) -> Self {
        elevated(self, cx, ElevationIndex::ElevatedSurface)
    }
}

impl<E: Styled> StyledExt for E {}

pub trait FluentBuilder {
    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U
    where
        Self: Sized,
    {
        f(self)
    }

    /// Conditionally unwrap and modify self with the given closure, if the given option is Some.
    fn when_some<T>(self, option: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self
    where
        Self: Sized,
    {
        self.map(|this| {
            if let Some(value) = option {
                then(this, value)
            } else {
                this
            }
        })
    }
}

impl<T: IntoElement> FluentBuilder for T {}

pub(crate) const BASE_REM_SIZE_IN_PX: f32 = 16.;

#[inline(always)]
pub fn rems_from_px(px: f32) -> Rems {
    rems(px / BASE_REM_SIZE_IN_PX)
}
