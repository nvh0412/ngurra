use std::time::Duration;

use gpui::{ModelContext, Timer};

pub struct BlinkManager {
    blink_interval: Duration,
    blink_epoch: usize,
    visible: bool,
    enabled: bool,
}

impl BlinkManager {
    pub fn new(blink_interval: Duration, cx: &mut ModelContext<Self>) -> Self {
        Self {
            blink_interval,
            blink_epoch: 0,
            visible: true,
            enabled: false,
        }
    }

    pub fn enable(&mut self, cx: &mut ModelContext<BlinkManager>) {
        if self.enabled {
            return;
        }

        self.enabled = true;
        self.visible = false;
        self.blink_cursor(self.blink_epoch, cx)
    }

    pub fn show_cursor(&mut self, cx: &mut ModelContext<BlinkManager>) {
        if !self.visible {
            self.visible = true;
            cx.notify();
        }
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    fn blink_cursor(&mut self, blink_epoch: usize, cx: &mut ModelContext<'_, BlinkManager>) {
        if blink_epoch == self.blink_epoch && self.enabled {
            self.visible = !self.visible;
            cx.notify();

            let epoch = self.next_blink_epoch();
            let interval = self.blink_interval;
            cx.spawn(|this, mut cx| async move {
                Timer::after(interval).await;
                if let Some(this) = this.upgrade() {
                    this.update(&mut cx, |this, cx| this.blink_cursor(epoch, cx))
                        .ok();
                }
            })
            .detach();
        }
    }
}
