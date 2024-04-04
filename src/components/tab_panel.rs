use std::rc::Rc;

use crate::state::TabView;
use gpui::{div, prelude::*, AnyView, ViewContext};

use super::{
    add_card::AddCardView,
    card_browser::CardBrowserView,
    deck::DeckView,
    tab_bar_container::{TabBarContainer, TabEvent},
};

pub struct TabPanelBuilder;

pub struct TabPanel {
    content: AnyView,
}

impl Render for TabPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child(self.content.clone())
    }
}

impl TabView for TabPanelBuilder {
    fn build(&self, tabbar: &TabBarContainer, cx: &mut gpui::WindowContext) -> gpui::AnyView {
        let panel = TabPanel {
            content: DeckView::view(cx).into(),
        };

        let tab_view = Rc::clone(&tabbar.view);

        cx.new_view(|cx| {
            cx.subscribe(
                &*tab_view.borrow_mut(),
                move |subscriber: &mut TabPanel, _emitter, event, cx| match event {
                    TabEvent::Deck => {
                        subscriber.content = DeckView::view(cx).into();
                    }
                    TabEvent::Add => subscriber.content = AddCardView::view(cx).into(),
                    TabEvent::Browse => subscriber.content = CardBrowserView::view(cx).into(),
                },
            )
            .detach();

            panel
        })
        .into()
    }
}
