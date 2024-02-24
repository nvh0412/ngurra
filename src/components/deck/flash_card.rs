use gpui::{
    div, green, AnyView, FontWeight, IntoElement, ParentElement, Pixels, Render, Styled,
    ViewContext, VisualContext, WindowContext,
};

use crate::{repositories::flash_card, state::StackableView, theme::Theme};

pub struct FlashCard {
    cards: Vec<flash_card::FlashCard>,
}

impl FlashCard {
    pub fn view(cx: &mut WindowContext, cards: Vec<flash_card::FlashCard>) -> AnyView {
        cx.new_view(|vc| Self { cards }).into()
    }
}

impl Render for FlashCard {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let card = self.cards.first().unwrap();

        div()
            .flex()
            .w_full()
            .flex_col()
            .pt_20()
            .text_color(theme.text)
            .justify_center()
            .items_center()
            .child(
                div()
                    .max_w(Pixels(500.0))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .pb_5()
                            .border_b_1()
                            .border_color(theme.crust)
                            .child(card.get_question().to_string()),
                    )
                    .child(div().pt_5().text_xl().child(card.get_answer().to_string())),
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
