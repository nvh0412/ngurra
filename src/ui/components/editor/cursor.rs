use gpui::{blue, fill, green, point, px, size, Bounds, ElementContext, Hsla, Pixels, Point};

use crate::theme::{self, Theme};

#[derive(Default)]
pub enum CursorShape {
    #[default]
    Bar,
    Underscore,
}

pub struct Cursor {
    origin: Point<Pixels>,
    shape: CursorShape,
    line_height: Pixels,
}

impl Cursor {
    pub fn new(origin: gpui::Point<Pixels>, shape: CursorShape, line_height: Pixels) -> Self {
        Self {
            origin,
            shape,
            line_height,
        }
    }

    pub fn paint(&self, origin: Point<Pixels>, cx: &mut ElementContext) {
        let theme = cx.global::<Theme>();
        // Only support bar cursor for now
        let bounds = Bounds {
            origin: self.origin + point(Pixels(12.0), Pixels(6.0)),
            size: size(px(2.0), self.line_height - px(12.0)),
        };

        let cursor = fill(bounds, theme.blue);

        cx.paint_quad(cursor);
    }
}
