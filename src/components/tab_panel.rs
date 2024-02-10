use std::rc::Rc;

use crate::state::StateView;
use gpui::{prelude::*, AnyView, ViewContext};

use super::{add_card_container::AddCardContainer, browse_container::BrowseContainer, deck_list_container::DeckListContainer, tab_bar_container::{TabBarContainer, TabBarView, TabEvent}};

pub struct TabPanelBuilder;

pub struct TabPanel {
    content: AnyView
}

impl Render for TabPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.content.clone()
    }
}

impl StateView for TabPanelBuilder {
    fn build(
        &self,
        tabbar: &TabBarContainer,
        cx: &mut gpui::WindowContext
    ) -> gpui::AnyView {
        let panel = TabPanel {
            content: DeckListContainer::view(cx).into()
        };

        let tab_view = Rc::clone(&tabbar.view);

        cx.new_view(|cx| {
            cx.subscribe(
                &*tab_view.borrow_mut(),
                move |subscriber: &mut TabPanel, _emitter, event, cx| {
                    match event {
                        TabEvent::Deck => {
                            subscriber.content = DeckListContainer::view(cx).into()
                        }
                        TabEvent::Add => {
                            subscriber.content = AddCardContainer::view(cx).into()
                        }
                        TabEvent::Browse => {
                            subscriber.content = BrowseContainer::view(cx).into()
                        }
                        _ => {}
                    }
                }
            ).detach();

            panel
        }).into()
    }
}
