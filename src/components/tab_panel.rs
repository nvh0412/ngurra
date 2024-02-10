use crate::state::StateView;
use gpui::{prelude::*, AnyView, ViewContext};

use super::{deck_list_container::DeckListContainer, tab_bar_container::TabBarContainer};

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

        cx.new_view(|cx| {
            cx.subscribe(
                &tabbar.view,
                move|subscriber, _emitter, event, cx| {
                    match event {
                        _ => {}
                    }
                }
            ).detach();

            panel
        }).into()
    }
}
