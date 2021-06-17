use glium::glutin::*;
use std::sync::*;
use webrender::api::RenderNotifier;

pub struct Notifier {
    events_proxy: event_loop::EventLoopProxy<()>,
    sender: mpsc::Sender<()>
}

impl Notifier {
    pub fn new(events_proxy: event_loop::EventLoopProxy<()>, sender: mpsc::Sender<()>) -> Notifier {
        Notifier { 
            events_proxy,
            sender
        }
    }
}

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<dyn RenderNotifier> {
        Box::new(Notifier {
            events_proxy: self.events_proxy.clone(),
            sender: self.sender.clone()
        })
    }

    fn wake_up(&self) {
        #[cfg(not(target_os = "android"))]
        let _ = self.events_proxy.send_event(());
        self.sender.send(()).unwrap();
    }

    fn new_frame_ready(
        &self, _: webrender::api::DocumentId,
        _scrolled: bool,
        _composite_needed: bool,
        _render_time: Option<u64>) {
        self.wake_up();
    }

    fn shut_down(&self) {}
}