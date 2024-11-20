# Hyprclock

![hyprclock_001](https://github.com/user-attachments/assets/a80249e7-860c-45ca-8d37-818f89fca74c)

**Hyprclock** is a modern, highly customizable clock application designed for use with the Hyprland window manager on Linux. It offers real-time updates, animated effects, and easy theming options, making it a perfect fit for your Hyprland setup.

## Features

- [x] **Real-time Clock**: Displays the current time and updates every second.
- [ ] **Customizable Themes**: Supports different themes that can be applied through a simple configuration file or the application’s settings.
- [ ] **Animations**: Includes support for basic animations like blur and fade-in effects.
- [ ] **Easy Configuration**: Configure appearance and behavior through a user-friendly configuration file.

## Status

**Hyprclock** is currently in active development. The application is nearing its release, but I'm still adding features and refining its functionality. Stay tuned for updates! This is my first rust built widget as I'm new to the rust language. I'm not a professional software engineer, or programmer. I'm simply a nerd who enjoys learning.

## Installation

To install **Hyprclock**, follow these steps:

1. Download the hyprclock-0.1.0a.tar.gz file from the Releases section of the hyprclock repository.
2. Open a terminal
3. cd ~/path/to/where/you/saved/the/file
4. tar -xzf hyprclock-0.1.0a.tar.gz
5. cd hyprclock
6. chmod +x hyprclock
7. ./hyprclock
8. Move to a directory such as /usr/local/bin
    sudo mv hyprclock /usr/local/bin/   
    after moving it you can simply use hyprclock to launch the application from a terminal

## Configuration

Example for hyprclock.conf:

```
# ~/.config/hypr/hyprclock.conf

[animation]
blur = true
fade_in = true

[env]
environment = "development"

[general]
clock_format = "24-hour"

[theme]
background_color = "#000000"
font_color = "#59F87E"
font_size = 200
```

## Usage

Once installed, you can launch Hyprclock from your application menu or by using a keybind in Hyprland. You can also add it to your autostart configuration to have it launch on startup.

## Running Hyprclock

```
hyprclock
```

## Auto-start

Add the following line to your hyprland.conf

```
exec-once = hyprclock
```

# blackbeard.nvim

**blackbeard.nvim** is a modern, customizable Neovim color scheme designed for productivity. It’s optimized for fast startup times and supports Lua bytecode compilation. Heavily inspired by TokyoNight, Kanagawa, Cyberdream, and Gruvbox.

## Preview
![blackbeard-nvim-preview](https://github.com/cvusmo/blackbeard-nvim/blob/dev/assets/preview/blackbeard-nvim-preview.png?raw=true)

## Features

- Dark and Light themes inspired by popular colorschemes like Gruvbox, Kanagawa, TokyoNight, and Cyberdream
- Support for TreeSitter syntax highlighting.
- Customizable palette and themes.
- Transparent background support.
- Integration with popular plugins such as Telescope, LSP, and more.

## Installation

To install `blackbeard.nvim`, you can use your favorite plugin manager.

### Using LazyVim:
```lua
-- ~/.config/nvim/init.lua

require("lazy").setup({
  {
    "blackbeard-nvim",
    config = function()
      require("blackbeard").setup({
        transparent = true,  -- Enable transparency
        theme = "dark",      -- Set theme to dark or light
        terminalColors = true, -- Enable terminal colors
      })
      vim.cmd("colorscheme blackbeard")  -- Apply the colorscheme
    end
  },
})
```

### Adding blackbeard-nvim as a plugin in LazyVim
```lua
return {
    -- Install the plugin from GitHub using the correct plugin name
    "cvusmo/blackbeard-nvim",  -- Directly specify the GitHub repo
    config = function()
        -- Setup the plugin
        require("blackbeard").setup({
            compile = false,  -- Enable compiling the colorscheme
            undercurl = true,  -- Enable undercurls
            commentStyle = { italic = true },
            functionStyle = {},
            keywordStyle = { italic = true },
            statementStyle = { bold = true },
            typeStyle = {},
            transparent = false,  -- Do not set background color
            dimInactive = false,  -- Dim inactive window (`hl-NormalNC`)
            terminalColors = true,  -- Define `vim.g.terminal_color_{0,17}`
            colors = {  -- Add/modify theme and palette colorsan
                palette = {},
                theme = { dark = {}, light = {} },
            },
            overrides = function(colors)  -- Modify built-in highlights
                return {}
            end,
            theme = "dark",  -- Set theme to dark or light
            background = {  -- Map the value of `background` option to a theme
                dark = "dark",  -- `vim.o.background = "dark"`
                light = "light"  -- `vim.o.background = "light"`
            },
        })
    end,
}
```

### Using Packer:
```lua
-- ~/.config/nvim/init.lua

-- Ensure Packer is installed
local ensure_packer = function()
  local fn = vim.fn
  local install_path = fn.stdpath("data") .. "/site/pack/packer/start/packer.nvim"
  if fn.empty(fn.glob(install_path)) > 0 then
    print("Installing packer.nvim...")
    fn.system({"git", "clone", "https://github.com/wbthomason/packer.nvim", install_path})
    vim.cmd("packadd packer.nvim")
  end
end

ensure_packer()

-- Packer setup
require("packer").startup(function(use)
  use "wbthomason/packer.nvim"

  use {
    "blackbeard-nvim",  -- Replace this with your Blackbeard Nvim repository
    config = function()
      require("blackbeard").setup({
        transparent = true,  -- Enable transparency
        theme = "dark",      -- Set theme to dark
        terminalColors = true,  -- Enable terminal colors
      })
      vim.cmd("colorscheme blackbeard")  -- Apply the colorscheme
    end
  }

  -- Add more plugins here as needed
end)
```

### Requirements
- Neovim (latest stable version)
- Truecolor terminal support
- Optional: Undercurl terminal support

## Usage

Once the plugin is installed, you can set the theme by adding the following line to your `init.lua` or `init.vim`:

```lua
vim.cmd("colorscheme blackbeard")
```

## Configuration

You can customize the `blackbeard.nvim` color scheme by setting the following options:

### Default Options:
```lua
require('blackbeard').setup({
    compile = false,             -- Enable compiling the colorscheme
    undercurl = true,            -- Enable undercurls
    commentStyle = { italic = true },
    functionStyle = {},
    keywordStyle = { italic = true },
    statementStyle = { bold = true },
    typeStyle = {},
    transparent = false,         -- Do not set background color
    dimInactive = false,         -- Dim inactive window (`hl-NormalNC`)
    terminalColors = true,       -- Define `vim.g.terminal_color_{0,17}`
    colors = {                   -- Add/modify theme and palette colors
        palette = {},
        theme = { dark = {}, light = {} },
    },
    overrides = function(colors)  -- Modify built-in highlights
        return {}
    end,
    theme = "dark",              -- Set theme to dark or light
    background = {               -- Map the value of `background` option to a theme
        dark = "dark",           -- `vim.o.background = "dark"`
        light = "light"          -- `vim.o.background = "light"`
    },
})
```

### Using Overrides:
You can modify any highlight groups using the `overrides` function:
```lua
require('blackbeard').setup({
    overrides = function(colors)
        return {
            String = { fg = colors.palette.carpYellow, italic = true },  -- Customize String color
            SomePluginHl = { fg = colors.theme.syn.type, bold = true },  -- Customize plugin highlight
        }
    end
})
```

## Color Palette

| Name             | Hex      | Usage                            |
|------------------|----------|----------------------------------|
| deepRetroBrown   | #1C1B1A  | Dark background                  |
| warmRetroCream   | #F4E3C1  | Default foreground               |
| darkGrayishBlue  | #1F1F28  | Default background               |
| mutedAvocadoGreen| #73A857  | Git Add                          |
| vibrantRetroRed  | #D13438  | Git Delete                       |
| goldenMustard    | #F1C232  | Git Change                       |
| retroOrange      | #F27835  | Cursor color                     |
| softRetroTeal    | #5A8CA5  | Diff Change (background)         |
| softLavender     | #A066C9  | Diff Deleted (background)        |
| mintGreen        | #46B9A0  | Diff Line (background)           |
| warmRetroWhite   | #F4E3C1  | Default foreground               |
| softerDeepBrown  | #484441  | Non-text, comment color          |
| vibrantCoralRed  | #FF5F56  | Bright Git Add                   |
| brightRetroLime  | #88C070  | Bright Git Change                |
| lightRetroTeal   | #73B3D8  | Bright Diff Line                 |
| pastelLilac      | #B794F4  | Git Diff (background)            |
| brightGoldenYellow| #FADF60 | Bright Git Change                |
| brightAquaGreen  | #6FE2CA  | Bright Diff Add                  |
| pureWhite        | #FFFFFF  | Bright Git Delete                |

---

### Acknowledgements

- [Kanagawa](https://github.com/rebelot/kanagawa.nvim)
- [Cyberdream](https://github.com/scottmckendry/cyberdream.nvim)
- [Tokyonight](https://github.com/folke/tokyonight.nvim)
- [Gruvbox](https://github.com/morhetz/gruvbox)

### Support Me

## Donate

![TWLOHA](https://panels.twitch.tv/panel-32185066-image-1aa09e79-4ba3-415d-a9f1-321b4ee42f91)
- Don't buy me a coffee. [Donate](https://www.twitch.tv/charity/cvusmo) To Write Love on Her Arms is a nonprofit movement dedicated to presenting hope and finding help for people struggling with depression, addiction, self-injury, and suicide. TWLOHA exists to encourage, inform, inspire, and invest directly into treatment and recovery. To Write Love on Her Arms before subscribing. I would rather any amount of $ go to helping someone get the help they need, than to me.

## Twitch
- I stream Weds-Sun on [twitch](https://www.twitch.tv/cvusmo) from 05:00 EST to 11:00 AM EST. Come hang out in chat, and let me know what you're working on! All active subscribers will be added to credits for Lustre game engine and other software I develop.

## Youtube
- [youtube](https://www.youtube.com/@cvusmo) Not as active as I used to be but plan on uploading Rust/Lua related content starting January 2025. Help me reach monetization by simply subscribing to the channel. Leave a comment and let me know what you'd be interested in. I plan on going through creating this blackbeard-nvim as a series. Then diving into Rust related projects to show how to create basic applications.

## x
- [x](https://www.x.com/cvusmo) Follow on x for more of day to day memes, random thoughts, and spicy fresh hot takes.

