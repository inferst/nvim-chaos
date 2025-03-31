use std::{cell::RefCell, rc::Rc};

use nvim_oxi::{
    self as nvim,
    api::{
        opts::{BufDeleteOpts, CreateAutocmdOpts},
        types::{WindowConfig, WindowRelativeTo, WindowStyle},
    },
    Function,
};

use super::ModeCommand;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Command {}

impl ModeCommand for Command {
    fn start(&self) -> nvim::Result<()> {
        let buf = nvim::api::create_buf(false, true)?;
        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Cursor)
            .style(WindowStyle::Minimal)
            .width(300)
            .height(10)
            .row(-5)
            .col(1)
            .build();

        let mut w = nvim::api::open_win(&buf, false, &config)?;
        w.set_option("winblend", 5)?;

        let win = Rc::new(RefCell::new(Some(w)));

        let opts = CreateAutocmdOpts::builder()
            .once(true)
            .callback(Function::from_fn(move |_| {
                if win.borrow().is_some() {
                    win.borrow_mut().take().unwrap().close(true).unwrap();
                }
                let opts = BufDeleteOpts::builder().force(true).build();
                buf.clone().delete(&opts).unwrap();
                true
            }))
            .build();

        nvim::api::create_autocmd(vec!["CursorMoved"], &opts)?;

        Ok(())
    }

    fn is_valid(&self) -> nvim::Result<bool> {
        Ok(true)
    }

    fn stop(&self) -> nvim::Result<()> {
        Ok(())
    }

    fn name(&self) -> String {
        "Only cursor line".to_string()
    }
}
