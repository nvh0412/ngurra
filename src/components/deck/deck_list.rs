use gpui::{
    div, AnyView, InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, Styled, View,
    VisualContext, WindowContext,
};

use crate::{
    models::{collection::Collection, deck::get_decks},
    repositories::deck::DeckStat,
    state::{StackableView, StackableViewState},
    theme::Theme,
    Deck,
};

use super::deck_detail::DeckDetailBuilder;

pub struct DeckListView;

pub struct DeckListBuilder;
impl StackableView for DeckListBuilder {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        DeckListView::view(cx).into()
    }
}

impl DeckListView {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_vc| Self)
    }

    fn get_all_decks_and_stats(&self, collection: &Collection) -> Vec<Deck> {
        let decks = get_decks(&collection.storage.conn);
        let decks_stats = Deck::get_decks_stats(&collection.storage.conn).unwrap();

        decks
            .into_iter()
            .map(|mut deck| {
                let st = decks_stats.get(&deck.id.unwrap()).unwrap();
                deck.stats = Some(DeckStat {
                    id: Some(deck.id.unwrap()),
                    new: st.new,
                    learning: st.learning,
                    due: st.due,
                });
                deck
            })
            .collect()
    }
}

impl Render for DeckListView {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let theme = cx.global::<Theme>();
        let collection = cx.global::<crate::Collection>();

        div().flex().size_full().justify_center().child(
            div().mt_20().child(
                div()
                    .border_1()
                    .border_color(theme.crust)
                    .rounded_xl()
                    .text_color(theme.text)
                    .p_3()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .text_sm()
                            .child(div().px_2().min_w_80().child(format!("Deck")))
                            .child(div().min_w_20().flex().justify_center().child("New"))
                            .child(div().min_w_20().flex().justify_center().child("Learn"))
                            .child(div().min_w_20().flex().justify_center().child("Due"))
                            .pb_2()
                            .border_b_1()
                            .border_color(theme.crust)
                            .mb_2(),
                    )
                    .children(
                        self.get_all_decks_and_stats(collection)
                            .into_iter()
                            .map(|deck| {
                                let deck_id = deck.id.unwrap();
                                HocListItem::init(
                                    cx.new_view(|_| ListItem::new(deck)).into(),
                                    deck_id,
                                )
                            })
                            .collect::<Vec<_>>(),
                    ),
            ),
        )
    }
}

#[derive(IntoElement)]
pub struct HocListItem {
    inner: AnyView,
    deck_id: u32,
}

impl HocListItem {
    pub fn init(inner: AnyView, deck_id: u32) -> Self {
        Self { inner, deck_id }
    }
}

impl RenderOnce for HocListItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let mut bg_hover = theme.mantle;
        bg_hover.fade_out(0.5);

        div()
            .flex()
            .hover(|s| s.bg(bg_hover))
            .p_2()
            .border_1()
            .rounded_xl()
            .on_mouse_down(gpui::MouseButton::Left, move |e, cx| {
                StackableViewState::update(
                    |state, cx| {
                        state.push(
                            DeckDetailBuilder {
                                deck_id: self.deck_id,
                            },
                            cx,
                        )
                    },
                    cx,
                );
            })
            .child(self.inner)
    }
}

pub struct ListItem {
    deck: Deck,
}

impl ListItem {
    pub fn new(deck: Deck) -> Self {
        Self { deck }
    }
}

impl Render for ListItem {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let stats = self.deck.stats.as_ref().unwrap();

        div()
            .flex()
            .w_full()
            .justify_center()
            .items_center()
            .child(div().min_w_80().text_sm().child(self.deck.name.clone()))
            .child(
                div()
                    .min_w_20()
                    .flex()
                    .justify_center()
                    .text_color(theme.blue)
                    .child(format!("{}", stats.new)),
            )
            .child(
                div()
                    .min_w_20()
                    .flex()
                    .justify_center()
                    .text_color(theme.red)
                    .child(format!("{}", stats.learning)),
            )
            .child(
                div()
                    .min_w_20()
                    .flex()
                    .justify_center()
                    .text_color(theme.green)
                    .child(format!("{}", stats.due)),
            )
    }
}
