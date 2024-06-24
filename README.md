# Chaos Neovim

Neovim plugin which makes coding difficult if you're a Twitch streamer.

## Features
Showing message from chatter inside the editor

Changing current colorscheme

Enabling Vim Motions Hell mode which inverts vim motions bindings

## Installation

It's written in [Rust](https://www.rust-lang.org/tools/install) (don't ask why) which is *required* for installation

Tested only on Linux (Ubuntu)

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
-- You need to setup plugin and set your twitch channel.
-- Other parameters are optional.
-- Duration for Colorscheme command is 5 min, form vim motions hell is 1 min.

local chaos = require 'nvim_chaos'
chaos.setup {
    channel = 'your_twitch_channel', -- set your twitch channel here
    commands = {
      message = '!msg', -- name of message command
      colorscheme = '!colorscheme', -- first argument is colorscheme name, second argument is background (dark, light)
      hell = '!vimhell', -- name of vim motions hell command
    },
}
```
