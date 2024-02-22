use gpui::{
    div, AnchorCorner, AnyView, AppContext, Context, EventEmitter, FocusHandle, InteractiveElement,
    IntoElement, KeyDownEvent, Model, ParentElement, Pixels, Render, RenderOnce, SharedString,
    Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::{
    components::{add_card::AddCardBuilder, shared::icon::Icon},
    models::{
        collection::{Collection, CollectionBuilder},
        deck::get_decks,
    },
    repositories::deck::DeckStat,
    state::{StackableView, StackableViewState},
    theme::Theme,
    ui::{
        button::button::Button, clickable::Clickable, context_menu::ContextMenu,
        popover_menu::popover_menu, text_field::text_field::TextEvent,
    },
    Deck,
};

use super::{deck_detail::DeckDetailBuilder, new_deck_form::NewDeckFormBuilder};

pub struct DeckListView {
    selected: Model<u32>,
    items: Vec<Deck>,
    focus_handle: FocusHandle,
}

pub struct DeckListBuilder;
impl StackableView for DeckListBuilder {
    fn build(&self, cx: &mut WindowContext) -> AnyView {
        DeckListView::view(cx).into()
    }
}

impl EventEmitter<TextEvent> for DeckListView {}

impl DeckListView {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        let collection = cx.global::<crate::Collection>();
        let items = Self::get_all_decks_and_stats(collection);
        let selected = cx.new_model(|_| {
            if items.len() > 0 {
                items[0].id.unwrap_or(0)
            } else {
                0
            }
        });
        let focus_handle = cx.focus_handle();

        focus_handle.focus(cx);

        let list = Self {
            selected,
            items,
            focus_handle,
        };

        let view = cx.new_view(move |cx| {
            cx.observe(&list.selected, move |_this: &mut DeckListView, _, cx| {
                cx.notify();
            })
            .detach();

            list
        });
        view
    }

    pub fn selected(&self, cx: &AppContext) -> Option<(usize, Deck)> {
        let id = self.selected.read(cx);

        self.items
            .clone()
            .into_iter()
            .enumerate()
            .find(|(_, item)| item.id.unwrap().eq(id))
    }

    fn key_down(&mut self, event: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        match event.keystroke.key.as_str() {
            "up" => {
                let index = if let Some((index, _)) = self.selected(cx) {
                    if index > self.items.len() - 1 {
                        index - 1
                    } else {
                        0
                    }
                } else {
                    0
                };

                self.selected.update(cx, |this, cx| {
                    *this = self.items[index].id.unwrap();

                    cx.notify();
                });
            }
            "down" => {
                let index = if let Some((index, _)) = self.selected(cx) {
                    let size = self.items.len() - 1;
                    if index < size {
                        index + 1
                    } else {
                        size
                    }
                } else {
                    0
                };

                self.selected.update(cx, |this, cx| {
                    *this = self.items[index].id.unwrap();

                    cx.notify();
                });
            }
            "tab" => {
                let index = if let Some((index, _)) = self.selected(cx) {
                    let size = self.items.len() - 1;
                    if index < size {
                        index + 1
                    } else {
                        0
                    }
                } else {
                    0
                };

                self.selected.update(cx, |this, cx| {
                    *this = self.items[index].id.unwrap();

                    cx.notify();
                });
            }
            "enter" | "space" => {
                if let Some((_index, deck)) = self.selected(cx) {
                    StackableViewState::update(
                        |state, cx| {
                            state.push(
                                DeckDetailBuilder {
                                    deck_id: deck.id.unwrap(),
                                },
                                cx,
                            )
                        },
                        cx,
                    );

                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn get_all_decks_and_stats(collection: &Collection) -> Vec<Deck> {
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
        cx.focus(&self.focus_handle);

        self.items = Self::get_all_decks_and_stats(cx.global::<Collection>());

        if self.items.len() == 0 {
            self.items.push(Deck::new(""));
        }

        let theme = cx.global::<Theme>();
        let collection = cx.global::<crate::Collection>();
        let selected = self.selected.read(cx);

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .justify_between()
            .on_key_down(cx.listener(Self::key_down))
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
                            Self::get_all_decks_and_stats(collection)
                                .into_iter()
                                .map(|deck| {
                                    let deck_id = deck.id.unwrap();
                                    HocListItem::init(deck, deck_id, deck_id.eq(selected))
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

#[derive(IntoElement, Clone)]
pub struct HocListItem {
    deck: Deck,
    menu_item: String,
    selected: bool,
}

impl HocListItem {
    pub fn init(deck: Deck, deck_id: u32, selected: bool) -> Self {
        let menu_item = format!("menu-item-{}", deck_id);
        Self {
            deck,
            menu_item,
            selected,
        }
    }

    fn build_deck_menu(cx: &mut WindowContext, deck_id: u32) -> View<ContextMenu> {
        ContextMenu::build(cx, move |menu, _wc| {
            menu.entry("Add new card", None, move |wc| {
                StackableViewState::update(
                    |state, cx| state.push(AddCardBuilder { deck_id }, cx),
                    wc,
                );
            })
            .entry("Delete", None, move |wc| {
                let collection = wc.global::<Collection>();
                Deck::delete(deck_id, &collection.storage.conn).unwrap();
            })
        })
    }
}

impl RenderOnce for HocListItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let menu_id = SharedString::from(self.menu_item);
        let menu_btn = SharedString::from(format!("btn-{}", self.deck.id.unwrap()));

        let deck_id = self.deck.id.unwrap();
        let inner = cx.new_view(|_| ListItem::new(self.deck, self.selected));

        div()
            .flex()
            .py_2()
            .border_1()
            .rounded_xl()
            .child(inner)
            .child(
                div().child(
                    popover_menu(menu_id)
                        .menu(move |cx| Some(Self::build_deck_menu(cx, deck_id)))
                        .anchor(AnchorCorner::TopLeft)
                        .trigger(Button::new(menu_btn, "M", Some(Icon::Settings))),
                ),
            )
    }
}

pub struct ListItem {
    selected: bool,
    deck: Deck,
}

impl ListItem {
    pub fn new(deck: Deck, selected: bool) -> Self {
        Self { deck, selected }
    }
}

impl Render for ListItem {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let mut bg_hover = theme.overlay0;
        bg_hover.fade_out(0.5);

        if let None = self.deck.id {
            return div().child("No decks found");
        }

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

        if self.selected {
            div().border_color(theme.crust).bg(bg_hover).rounded_md()
        } else {
            div().hover(|s| s.px_2().rounded_lg().bg(bg_hover))
        }
        .flex()
        .w_full()
        .justify_center()
        .items_center()
        .on_mouse_down(gpui::MouseButton::Left, move |_e, cx| {
            StackableViewState::update(
                |state, cx| state.push(DeckDetailBuilder { deck_id }, cx),
                cx,
            );
        })
        .child(
            div()
                .min_w_80()
                .text_sm()
                .pl_2()
                .child(self.deck.name.clone()),
        )
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
