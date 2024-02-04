use std::ops::Range;

use gpui::{
    div, EventEmitter, FocusHandle, HighlightStyle, InteractiveElement, InteractiveText,
    IntoElement, KeyDownEvent, ParentElement, Render, RenderOnce, Styled, StyledText, TextStyle,
    View, ViewContext, VisualContext, WindowContext,
};

use crate::theme::Theme;

#[derive(IntoElement, Clone)]
pub struct TextField {
    focus_handle: FocusHandle,
    pub view: View<TextView>,
}

impl TextField {
    pub fn new(cx: &mut WindowContext, placeholder: String) -> Self {
        let focus_handle = cx.focus_handle();
        let view = TextView::init(cx, &focus_handle, placeholder);
        Self { focus_handle, view }
    }

    pub fn set_place_holder(&mut self, placeholder: String, cx: &mut WindowContext) {
        self.view.update(cx, |text_view, cx| {
            text_view.placeholder = placeholder;
            cx.notify();
        });
    }
}

impl RenderOnce for TextField {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        cx.focus(&self.focus_handle);
        let theme = cx.global::<Theme>();

        let clone = self.view.clone();

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(move |event, cx| {
                self.view.update(cx, |text_view, vc| {
                    let prev = text_view.text.clone();
                    vc.emit(TextEvent::KeyDown(event.clone()));
                    if let Some(ime_key) = &event.keystroke.ime_key {
                        text_view
                            .text
                            .replace_range(text_view.selection.clone(), ime_key);
                        let i = text_view.selection.start + ime_key.len();
                        text_view.selection = i..i;
                    }
                    if prev != text_view.text {
                        vc.emit(TextEvent::Input {
                            text: text_view.text.clone(),
                        });
                    }
                    vc.notify();
                })
            })
            .rounded_lg()
            .border_1()
            .border_color(theme.crust)
            .py_1p5()
            .px_3()
            .min_w_20()
            .child(clone)
    }
}

pub enum TextEvent {
    Input { text: String },
    Blur,
    Back,
    KeyDown(KeyDownEvent),
}

impl EventEmitter<TextEvent> for TextView {}

pub struct TextView {
    pub text: String,
    pub placeholder: String,
    pub word_click: (usize, u16),
    pub selection: Range<usize>,
}

impl TextView {
    pub fn init(
        cx: &mut WindowContext,
        focus_handle: &FocusHandle,
        placeholder: String,
    ) -> View<Self> {
        let m = Self {
            text: String::new(),
            placeholder,
            word_click: (0, 0),
            selection: 0..0,
        };

        let view = cx.new_view(|cx| {
            cx.on_blur(focus_handle, |_: &mut TextView, cx| {
                cx.emit(TextEvent::Blur);
            })
            .detach();
            cx.on_focus(focus_handle, |view, cx| {
                view.select_all(cx);
            })
            .detach();
            m
        });

        cx.subscribe(&view, |subscriber, emitter: &TextEvent, cx| match emitter {
            TextEvent::Input { text: _ } => {
                subscriber.update(cx, |editor, _cx| {
                    editor.word_click = (0, 0);
                });
            }
            _ => {}
        })
        .detach();

        view
    }

    pub fn select_all(&mut self, cx: &mut ViewContext<Self>) {
        self.selection = 0..self.text.len();
        cx.notify();
    }

    pub fn word_ranges(&self) -> Vec<Range<usize>> {
        let mut words = Vec::new();
        let mut last_was_boundary = true;
        let mut word_start = 0;
        let s = self.text.clone();

        for (i, c) in s.char_indices() {
            if c.is_alphanumeric() || c == '_' {
                if last_was_boundary {
                    word_start = i;
                }
                last_was_boundary = false;
            } else {
                if !last_was_boundary {
                    words.push(word_start..i);
                }
                last_was_boundary = true;
            }
        }

        // Check if the last characters form a word and push it if so
        if !last_was_boundary {
            words.push(word_start..s.len());
        }

        words
    }
}

impl Render for TextView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let mut text = self.text.clone();
        let mut selection_style = HighlightStyle::default();

        let mut color = theme.lavender;

        let mut style = TextStyle::default();
        style.color = theme.text;
        style.font_family = theme.font_sans.clone();

        if text.len() == 0 {
            text = self.placeholder.to_string();
            style.color = theme.subtext0;
        }

        let styled_text = StyledText::new(text + " ").with_highlights(&style, vec![]);
        let view = cx.view().clone();

        InteractiveText::new("text", styled_text).on_click(self.word_ranges(), move |ev, cx| {
            view.update(cx, |text_view, cx| {
                let (index, mut count) = text_view.word_click;
                if index == ev {
                    count += 1;
                } else {
                    count = 1;
                }
                match count {
                    2 => {
                        let word_ranges = text_view.word_ranges();
                        text_view.selection = word_ranges.get(ev).unwrap().clone();
                    }
                    3 => {
                        // Should select the line
                    }
                    4 => {
                        count = 0;
                        text_view.selection = 0..text_view.text.len();
                    }
                    _ => {}
                }
                text_view.word_click = (ev, count);
                cx.notify();
            });
        })
    }
}
