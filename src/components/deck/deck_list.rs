use std::fmt::format;

use gpui::{
    div, AnchorCorner, AnyView, InteractiveElement, IntoElement, ParentElement, Pixels, Render,
    RenderOnce, SharedString, Styled, View, VisualContext, WindowContext,
};

use crate::{
    components::shared::icon::Icon,
    models::{
        collection::{self, Collection, CollectionBuilder},
        deck::get_decks,
    },
    repositories::deck::DeckStat,
    state::{StackableView, StackableViewState},
    theme::Theme,
    ui::{
        button::button::Button, clickable::Clickable, context_menu::ContextMenu,
        popover_menu::popover_menu,
    },
    Deck,
};

use super::{deck_detail::DeckDetailBuilder, new_deck_form::NewDeckFormBuilder};

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
        let timing_at_stamp = CollectionBuilder::timing_for_timestamp(
            &collection.storage.conn,
            chrono::Local::now().timestamp(),
        );

        let decks_stats =
            Deck::get_decks_stats(&collection.storage.conn, timing_at_stamp.days_elapsed).unwrap();

        decks
            .into_iter()
            .map(|mut deck| {
                let st = decks_stats.get(&deck.id.unwrap());

                if let Some(st) = st {
                    deck.stats = Some(DeckStat {
                        id: Some(deck.id.unwrap()),
                        new: st.new,
                        learning: st.learning,
                        due: st.due,
                    });
                }

                deck
            })
            .collect()
    }

    fn new_deck_click(&mut self, _event: &gpui::ClickEvent, cx: &mut gpui::ViewContext<Self>) {
        StackableViewState::update(|state, cx| state.push(NewDeckFormBuilder {}, cx), cx);
    }
}

impl Render for DeckListView {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let theme = cx.global::<Theme>();
        let collection = cx.global::<crate::Collection>();

        div()
            .flex()
            .flex_col()
            .size_full()
            .justify_between()
            .child(
                div().mt_20().flex().justify_center().child(
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
            .child(
                div().mb_16().flex().justify_center().child(
                    div().w(Pixels(300.0)).flex().justify_center().child(
                        div().child(
                            Button::new("create_deck", "Create Deck", None)
                                .on_click(cx.listener(Self::new_deck_click)),
                        ),
                    ),
                ),
            )
    }
}

#[derive(IntoElement)]
pub struct HocListItem {
    inner: AnyView,
    deck_id: u32,
    menu_item: String,
}

impl HocListItem {
    pub fn init(inner: AnyView, deck_id: u32) -> Self {
        let menu_item = format!("menu-item-{}", deck_id);
        Self {
            inner,
            deck_id,
            menu_item,
        }
    }

    fn build_deck_menu(cx: &mut WindowContext, deck_id: u32) -> View<ContextMenu> {
        ContextMenu::build(cx, move |menu, _wc| {
            menu.entry("Delete", None, move |wc| {
                let collection = wc.global::<Collection>();
                Deck::delete(deck_id, &collection.storage.conn).unwrap();
            })
        })
    }
}

impl RenderOnce for HocListItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let menu_id = SharedString::from(self.menu_item);
        let menu_btn = SharedString::from(format!("btn-{}", self.deck_id));

        div()
            .flex()
            .p_2()
            .border_1()
            .rounded_xl()
            .child(self.inner)
            .child(
                div().child(
                    popover_menu(menu_id)
                        .menu(move |cx| Some(Self::build_deck_menu(cx, self.deck_id)))
                        .anchor(AnchorCorner::TopLeft)
                        .trigger(Button::new(menu_btn, "M", Some(Icon::Settings))),
                ),
            )
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
        let mut bg_hover = theme.mantle;
        bg_hover.fade_out(0.5);

        let deck_id = self.deck.id.unwrap();

        let stats = match self.deck.stats.as_ref() {
            Some(stats) => stats,
            None => &DeckStat {
                id: None,
                new: 0,
                learning: 0,
                due: 0,
            },
        };

        div()
            .flex()
            .w_full()
            .hover(|s| s.bg(bg_hover))
            .justify_center()
            .items_center()
            .on_mouse_down(gpui::MouseButton::Left, move |_e, cx| {
                StackableViewState::update(
                    |state, cx| state.push(DeckDetailBuilder { deck_id }, cx),
                    cx,
                );
            })
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
