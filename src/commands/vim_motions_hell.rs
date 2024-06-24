use nvim_oxi::{api, Result};

use super::ModeCommand;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct VimMotionsHellCommand {}

impl ModeCommand for VimMotionsHellCommand {
    fn start(&self) -> Result<()> {
        api::command("noremap l h")?;
        api::command("noremap k j")?;
        api::command("noremap j k")?;
        api::command("noremap h l")?;

        api::command("noremap w b")?;
        api::command("noremap b w")?;
        api::command("noremap e ge")?;
        api::command("noremap ge e")?;

        Ok(())
    }

    fn is_valid(&self) -> Result<bool> {
        Ok(true)
    }

    fn stop(&self) -> Result<()> {
        api::command("noremap l l")?;
        api::command("noremap k k")?;
        api::command("noremap j j")?;
        api::command("noremap h h")?;

        api::command("noremap w w")?;
        api::command("noremap b b")?;
        api::command("noremap e e")?;
        api::command("noremap ge ge")?;

        Ok(())
    }

    fn name(&self) -> String {
        String::from("Vim Motions Hell")
    }
}
