use gpui::*;
use rusqlite::Connection;

use crate::{
    models::collection::{self, Collection}, repositories::{self, flash_card}, state::StackableView, theme::Theme, ui::{button::button::Button, clickable::Clickable}
};

pub struct FlashCard {
    pub focus_handle: FocusHandle,
    cards: Vec<flash_card::FlashCard>,
    current_card: usize,
    show_answer: bool,
}

impl FlashCard {
    pub fn view(cx: &mut WindowContext, cards: Vec<flash_card::FlashCard>) -> AnyView {
        let focus_handle = cx.focus_handle();
        cx.new_view(|vc| Self {
            focus_handle,
            cards,
            current_card: 0,
            show_answer: false,
        })
        .into()
    }

    pub fn next_card(&mut self, rate: u8, collection: &Collection) {
        if self.show_answer {
            self.show_answer = false;
            let card = self.cards.get_mut(self.current_card).unwrap();
            card.rate(rate);

            card.save(&collection.storage.conn).unwrap_or_else(|e| {
                println!("Error saving card: {:?}", e);
            });

            self.current_card += 1;
        } else {
            self.show_answer = true;
        }
    }

    fn key_down(&mut self, event: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();

        match event.keystroke.key.as_str() {
            "enter" | "space" => {
                self.next_card(3, collection);
                cx.notify();
            }
            _ => {
                // do nothing
            }
        };
    }

    fn again_click(&mut self, event: &ClickEvent, cx: &mut ViewContext<Self>) {}

    fn render_congrats(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .w_full()
            .flex_col()
            .text_color(theme.text)
            .relative()
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
                            .child("Congrats! You've finished all the cards"),
                    )
                    .child(
                        div()
                            .pt_5()
                            .child("Remember, learning is a journey, not a race. Taking the time to absorb and reflect on the material ensures deeper understanding and retention. Pace yourself, and enjoy the process of mastering each concept. Stay tuned for the next section, and happy learning!"),
                    ),
            )
    }
}

impl Render for FlashCard {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.focus(&self.focus_handle);

        let theme: &Theme = cx.global::<Theme>();

        if self.current_card >= self.cards.len() {
            return div()
                .flex()
                .track_focus(&self.focus_handle)
                .size_full()
                .justify_center()
                .child(div().mt_20().child(self.render_congrats(cx)));
        }

        let card = self.cards.get(self.current_card).unwrap();
        let answer = if self.show_answer {
            div().pt_5().child(card.get_answer().to_string())
        } else {
            div()
        };

        let actions = if self.show_answer {
            div().absolute().bottom_16().max_w(Pixels(500.0)).child(
                div()
                    .flex()
                    .justify_between()
                    .child(Button::new("again", "Again").on_click(cx.listener(Self::again_click)))
                    .child(Button::new("easy", "Easy"))
                    .child(Button::new("good", "Good"))
                    .child(Button::new("hard", "Hard")),
            )
        } else {
            div()
        };

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .size_full()
            .justify_center()
            .on_key_down(cx.listener(Self::key_down))
            .child(
                div().mt_20().child(
                    div()
                        .flex()
                        .w_full()
                        .flex_col()
                        .text_color(theme.text)
                        .relative()
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
                                        .child(card.get_question().to_string()),
                                )
                                .child(answer),
                        )
                        .child(actions),
                ),
            )
    }
}

pub struct FlashCardBuilder<'a> {
    pub cards: &'a Vec<flash_card::FlashCard>,
}

impl<'a> StackableView for FlashCardBuilder<'a> {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        FlashCard::view(cx, self.cards.clone().into())
    }
}
