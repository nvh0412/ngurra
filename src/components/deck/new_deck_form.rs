use gpui::{
    div, AnyView, ClickEvent, FocusHandle, FontWeight, InteractiveElement, ParentElement, Pixels,
    Render, Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::{
    repositories::{self},
    state::{StackableView, StackableViewState},
    theme::Theme,
    ui::{button::button::Button, clickable::Clickable, text_field::text_field::TextField},
};

struct NewDeckForm {
    text_input: TextField,
    focused_at: usize,
    focus_handle: FocusHandle,
}

impl NewDeckForm {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx: &mut gpui::ViewContext<'_, NewDeckForm>| {
            let text_input = TextField::new(cx, "Input deck name".to_string(), false);
            text_input.focus(cx);

            let focus_handle = cx.focus_handle();

            Self {
                text_input,
                focused_at: 0,
                focus_handle,
            }
        })
    }

    fn save_click(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.save(cx);
    }

    fn save_keydown(&mut self, cx: &mut ViewContext<Self>) {
        self.save(cx);
    }

    fn save(&mut self, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();
        let text = &self.text_input.view.read(&cx).text;

        let mut deck = repositories::deck::Deck::new(text);
        match deck.save(&collection.storage.conn) {
            Ok(_) => {
                StackableViewState::update(|state, cx| state.pop(cx), cx);
                cx.notify();
            }
            Err(e) => {
                log::error!("Error saving deck: {:?}", e);
            }
        }
    }
}

const TABBALE_FIELDS: [&str; 2] = ["name", "submit"];

impl Render for NewDeckForm {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let view = cx.view().clone();
        let theme = cx.global::<Theme>();

        let focused_at = self.focused_at;
        let name_input = self.text_input.clone();

        let mut submit_btn =
            Button::new("btn-save", "Create", None).on_click(cx.listener(Self::save_click));

        if self.focused_at == 1 {
            submit_btn.focus();
        }

        div().flex().size_full().justify_center().child(
            div().mt_20().child(
                div()
                    .track_focus(&self.focus_handle)
                    .flex()
                    .w_full()
                    .flex_col()
                    .text_color(theme.text)
                    .relative()
                    .on_key_down(move |event, wc| {
                        view.update(wc, |add_view, vc| {
                            let keystroke = &event.keystroke.key;

                            match keystroke.as_str() {
                                "tab" => {
                                    let next = (focused_at + 1) % TABBALE_FIELDS.len();

                                    match TABBALE_FIELDS[next] {
                                        "name" => name_input.focus(vc),
                                        "submit" => vc.focus(&add_view.focus_handle),
                                        _ => {}
                                    }
                                    add_view.focused_at = next;
                                }
                                "enter" => {
                                    add_view.save_keydown(vc);
                                }
                                _ => {}
                            }
                        })
                    })
                    .h_full()
                    .child(
                        div()
                            .w(Pixels(500.0))
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .pb_5()
                                    .border_b_1()
                                    .border_color(theme.crust)
                                    .child("New Deck"),
                            )
                            .child(div().mt_6().child(self.text_input.clone()))
                            .child(div().mt_6().justify_end().flex().child(submit_btn)),
                    ),
            ),
        )
    }
}

pub struct NewDeckFormBuilder {}

impl StackableView for NewDeckFormBuilder {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        NewDeckForm::view(cx).into()
    }
}
