// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file).

use accesskit::{ActionHandler, TreeUpdate};
use accesskit_macos::{Adapter as MacOSAdapter, SubclassingAdapter};
use winit::{platform::macos::WindowExtMacOS, window::Window};

pub struct Adapter {
    adapter: SubclassingAdapter,
}

impl Adapter {
    pub fn new(
        window: &Window,
        source: Box<dyn FnOnce() -> TreeUpdate>,
        action_handler: Box<dyn ActionHandler>,
    ) -> Self {
        let view = window.ns_view();
        let adapter = unsafe { MacOSAdapter::new(view, source, action_handler) };
        let adapter = unsafe { SubclassingAdapter::new(view, adapter) };
        Self { adapter }
    }

    pub fn update(&self, update: TreeUpdate) {
        let events = self.adapter.update(update);
        events.raise();
    }

    pub fn update_if_active(&self, updater: impl FnOnce() -> TreeUpdate) {
        if let Some(events) = self.adapter.update_if_active(updater) {
            events.raise();
        }
    }
}
