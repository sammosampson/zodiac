use app_state::TestState;
use illicit::from_env;
use moxie::*;
use log::info;

pub enum Message {
    IncreaseBorderSizeAfterTime
}

pub trait StateProcessor<TMessage> {
    fn process(&self, message: TMessage) -> Self;
}

impl StateProcessor<Message> for TestState {
    fn process(&self, message: Message) -> Self {
        match message {
            Message::IncreaseBorderSizeAfterTime => {
                if self.time.elapsed().as_secs() <= self.border_size {
                    return *self;
                }
                info!("increase_border_size: {}", self.border_size);
                Self {
                    border_size: self.border_size + 1,
                    ..*self
                }
            }
        }
    }
}

#[topo::nested]
#[from_env(state: &Key<TestState>)]
pub fn increase_border_size() {
    state.update(|state| Some(state.process(Message::IncreaseBorderSizeAfterTime)));
}