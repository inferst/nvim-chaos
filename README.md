# Chaos Neovim

Neovim plugin that makes coding more difficult if you're a Twitch streamer.

## Features

* Displays messages from chatters inside the editor
* Changes the current colorscheme
* Enables Vim Motions Hell mode, which inverts Vim motion bindings

## Installation

It's written in [Rust](https://www.rust-lang.org/tools/install) (don't ask why) which is *required* for installation.

Tested on Ubuntu, Mac OS.

### lazy.nvim

```lua
{ 'inferst/nvim-chaos', build = './install.sh' }
```

### packer.nvim

```lua
use { 'inferst/nvim-chaos', run = './install.sh' }
```

### vim-plug

```viml
Plug 'inferst/nvim-chaos', { 'do': './install.sh' }
```

## Configuration

```lua
-- You need to set up the plugin and specify your Twitch channel.
-- Other parameters are optional.
-- Duration for the Colorscheme command is 5 minutes, for Vim Motions Hell it is 1 minute.

local chaos = require 'nvim_chaos'
chaos.setup {
    channel = 'your_twitch_channel', -- set your Twitch channel here
    commands = {
      message = '!msg', -- name of the message command
      colorscheme = {
        -- First argument is the colorscheme name.
        -- Second argument is the background (dark or light).
        name = '!colorscheme',
        duration = 60 * 5,
      },
      hell = {
        name = '!vimhell',
        duration = 60,
      }
    },
}
```
