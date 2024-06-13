use std::{cell::RefCell, rc::Rc, time::Duration};

use message::MessageState;
use nvim_oxi::{
    api::{self},
    libuv::TimerHandle,
    schedule, Result,
};

pub mod chaos_mode;
pub mod message;

use chaos_mode::ChaosModeState;

use crate::commands::{Mode, ModeType};

#[derive(Clone, Default)]
pub struct State {
    pub message: MessageState,
    pub chaos_mode: ChaosModeState,
}

#[derive(Clone, Default)]
pub struct Plugin {
    pub state: Rc<RefCell<State>>,
}

impl Plugin {
    pub fn init(&mut self) -> Result<()> {
        self.start_timer();

        let mut state = self.state.borrow_mut();
        state.chaos_mode.init()?;
        state.message.init()?;

        Ok(())
    }

    pub fn err(str: &str) {
        api::err_writeln(str);
    }

    fn update(&mut self) -> Result<()> {
        let mut state = self.state.borrow_mut();
        state.chaos_mode.tick()?;

        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode, mode_type: ModeType, seconds: u32) -> Result<()> {
        let mut state = self.state.borrow_mut();
        state.chaos_mode.set_mode(mode, mode_type, seconds)?;

        Ok(())
    }

    fn start_timer(&mut self) {
        let plugin = self.clone();

        let callback = move |_timer: &mut TimerHandle| {
            let mut plugin = plugin.clone();

            schedule(move |()| {
                plugin.update().unwrap();
            });
        };

        let _handle =
            TimerHandle::start(Duration::from_millis(0), Duration::from_secs(1), callback);
    }

    pub fn show_msg(&mut self, author: &str, message: &str) -> Result<()> {
        let mut state = self.state.borrow_mut();
        state.message.show_msg(author, message)?;

        Ok(())
    }
}
