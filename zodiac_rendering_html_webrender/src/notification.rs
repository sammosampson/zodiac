use webrender::api::RenderNotifier;
pub struct Notifier {
    events_proxy: winit::EventsLoopProxy,
}

impl Notifier {
    pub fn new(events_proxy: winit::EventsLoopProxy) -> Notifier {
        Notifier { events_proxy }
    }
}

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<dyn RenderNotifier> {
        Box::new(Notifier {
            events_proxy: self.events_proxy.clone(),
        })
    }

    fn wake_up(&self) {
        #[cfg(not(target_os = "android"))]
        let _ = self.events_proxy.wakeup();
    }

    fn new_frame_ready(&self, _: webrender::api::DocumentId, _scrolled: bool, composite_needed: bool, _render_time: Option<u64>) {
        self.wake_up();
    }

    fn external_event(&self, _evt: webrender::api::ExternalEvent) {
        unimplemented!()
    }

    fn shut_down(&self) {}
}