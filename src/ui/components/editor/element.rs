use gpui::{
    fill, font, green, point, quad, relative, size, white, Bounds, ContentMask, Corners,
    DispatchPhase, Edges, Element, ElementContext, ElementInputHandler, Entity, InteractiveBounds,
    IntoElement, KeyContext, ModifiersChangedEvent, Pixels, Point, Size, Style, TextRun, View,
    WindowContext,
};

use crate::theme::Theme;

use super::{
    cursor::{Cursor, CursorShape},
    editor::{Editor, EditorMode},
};

pub struct EditorElement {
    editor: View<Editor>,
}

struct LayoutState {
    text_size: Size<Pixels>,
}

impl EditorElement {
    pub fn new(editor: &View<Editor>) -> EditorElement {
        EditorElement {
            editor: editor.clone(),
        }
    }

    fn compute_layout(&self, bounds: Bounds<Pixels>, cx: &mut WindowContext) -> LayoutState {
        self.editor.update(cx, |editor, cx| {
            let text_width = bounds.size.width;
            let text_size = size(text_width, bounds.size.height);

            LayoutState { text_size }
        })
    }

    fn register_actions(&self, cx: &mut WindowContext) {}

    fn register_key_listeners(&self, cx: &mut ElementContext, text_bounds: Bounds<Pixels>) {
        let stacking_order = cx.stacking_order().clone();

        cx.on_key_event({
            let editor = self.editor.clone();
            move |event: &ModifiersChangedEvent, phase, cx| {
                if phase != DispatchPhase::Bubble {
                    return;
                }

                editor.update(cx, |editor, cx| {
                    println!("Modifiers changed: {:?}", event.modifiers);
                })
            }
        })
    }

    fn paint_background(
        &self,
        text_bounds: Bounds<Pixels>,
        layout: &LayoutState,
        cx: &mut ElementContext,
    ) {
        let theme = cx.global::<Theme>();
        cx.paint_quad(quad(
            text_bounds,
            Corners {
                top_left: Pixels(8.0),
                top_right: Pixels(8.0),
                bottom_right: Pixels(8.0),
                bottom_left: Pixels(8.0),
            },
            theme.base,
            Edges {
                top: Pixels(1.0),
                right: Pixels(1.0),
                bottom: Pixels(1.0),
                left: Pixels(1.0),
            },
            theme.crust,
        ));
    }

    fn paint_text(
        &self,
        text_bounds: Bounds<Pixels>,
        layout: &mut LayoutState,
        cx: &mut ElementContext,
    ) {
        let content_origin = text_bounds.origin;

        cx.with_content_mask(
            Some(ContentMask {
                bounds: text_bounds,
            }),
            |cx| {
                let interactive_text_bounds = InteractiveBounds {
                    bounds: text_bounds,
                    stacking_order: cx.stacking_order().clone(),
                };

                self.editor.update(cx, |editor, cx| {
                    editor.pixel_position_of_newest_cursor = Some(point(
                        text_bounds.origin.x + Pixels(20.0),
                        text_bounds.origin.y,
                    ))
                });

                let cursor = Cursor::new(content_origin, text_bounds.size.height);

                let run = TextRun {
                    len: "input here".len(),
                    font: font("Helvetica"),
                    color: white(),
                    background_color: None,
                    underline: Default::default(),
                };

                cx.text_system()
                    .shape_line("input here".to_string().into(), Pixels(16.0), &[run])
                    .unwrap()
                    .paint(content_origin, text_bounds.size.height, cx);

                // Draw the cursor
                cx.with_z_index(1, |cx| {
                    cursor.paint(content_origin, cx);
                })
            },
        )
    }
}

impl IntoElement for EditorElement {
    type Element = Self;

    fn element_id(&self) -> Option<gpui::ElementId> {
        self.editor.element_id()
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for EditorElement {
    type State = ();

    fn request_layout(
        &mut self,
        _state: Option<Self::State>,
        cx: &mut gpui::ElementContext,
    ) -> (gpui::LayoutId, Self::State) {
        cx.with_view_id(self.editor.entity_id(), |cx| {
            self.editor.update(cx, |editor, cx| {
                let layout = match editor.mode {
                    EditorMode::SingleLine => {
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        cx.with_element_context(|cx| cx.request_layout(&style, None))
                    }
                    EditorMode::AutoHeight => cx.with_element_context(|cx| {
                        let mut style = Style::default();
                        style.size.width = relative(1.).into();
                        cx.with_element_context(|cx| cx.request_layout(&style, None))
                    }),
                };

                (layout, ())
            })
        })
    }

    fn paint(
        &mut self,
        bounds: gpui::Bounds<gpui::Pixels>,
        state: &mut Self::State,
        cx: &mut gpui::ElementContext,
    ) {
        let editor = self.editor.clone();

        cx.paint_view(editor.entity_id(), |cx| {
            cx.with_text_style(
                Some(gpui::TextStyleRefinement {
                    ..Default::default()
                }),
                |cx| {
                    let mut layout = self.compute_layout(bounds, cx);
                    let focus_handle = editor.focus_handle(cx);

                    let text_bounds = Bounds {
                        origin: bounds.origin,
                        size: layout.text_size,
                    };

                    cx.with_key_dispatch(
                        Some(KeyContext::default()),
                        Some(focus_handle.clone()),
                        |_, cx| {
                            self.register_actions(cx);

                            cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
                                self.register_key_listeners(cx, text_bounds);
                                cx.handle_input(
                                    &focus_handle,
                                    ElementInputHandler::new(bounds, self.editor.clone()),
                                );

                                self.paint_background(text_bounds, &layout, cx);
                                self.paint_text(text_bounds, &mut layout, cx);
                            })
                        },
                    )
                },
            )
        })
    }
}
