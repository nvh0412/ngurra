use std::collections::VecDeque;

use gpui::*;

use crate::{
    models::{
        answer::Answer,
        collection::Collection,
        queue::{Queue, QueueEntry},
    },
    repositories::flash_card,
    state::StackableView,
    theme::Theme,
    ui::{button::button::Button, clickable::Clickable},
};

pub struct FlashCard {
    pub focus_handle: FocusHandle,
    queue: VecDeque<QueueEntry>,
    show_answer: bool,
}

impl FlashCard {
    pub fn view(cx: &mut WindowContext, card_queue: &Queue) -> AnyView {
        let focus_handle = cx.focus_handle();
        cx.new_view(|_| Self {
            focus_handle,
            queue: card_queue.core.clone(),
            show_answer: false,
        })
        .into()
    }

    pub fn answer(&mut self, answer: Answer, collection: &Collection) {
        if self.show_answer {
            self.show_answer = false;
            let current_card = self.queue.pop_back().unwrap();

            collection.answer_card(current_card.card_id as u32, answer);
        } else {
            self.show_answer = true;
        }
    }

    fn key_down(&mut self, event: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();

        match event.keystroke.key.as_str() {
            "enter" | "space" => {
                self.answer(Answer::Good, collection);
                cx.notify();
            }
            _ => {
                // do nothing
            }
        };
    }

    fn again_click(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.show_answer = false;
        let card = self.queue.pop_back().unwrap();
        self.queue.push_front(card);

        cx.notify();
    }

    fn easy_click(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();
        self.answer(Answer::Easy, collection);
        cx.notify();
    }

    fn hard_click(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();
        self.answer(Answer::Hard, collection);
        cx.notify();
    }

    fn good_click(&mut self, _event: &ClickEvent, cx: &mut ViewContext<Self>) {
        let collection = cx.global::<crate::Collection>();
        self.answer(Answer::Good, collection);
        cx.notify();
    }

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
        let collection = cx.global::<crate::Collection>();
        let theme: &Theme = cx.global::<Theme>();

        if self.queue.is_empty() {
            return div()
                .flex()
                .track_focus(&self.focus_handle)
                .size_full()
                .justify_center()
                .child(div().mt_20().child(self.render_congrats(cx)));
        }

        let id = self.queue.back().unwrap().card_id;

        let card = flash_card::FlashCard::load(id, &collection.storage.conn).unwrap();
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
                    .child(Button::new("easy", "Easy").on_click(cx.listener(Self::easy_click)))
                    .child(Button::new("good", "Good").on_click(cx.listener(Self::good_click)))
                    .child(Button::new("hard", "Hard").on_click(cx.listener(Self::hard_click))),
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
    pub card_queue: &'a Queue,
}

impl<'a> StackableView for FlashCardBuilder<'a> {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        FlashCard::view(cx, self.card_queue)
    }
}
