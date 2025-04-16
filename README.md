# Hyprclock

[Hyprclock Showcase](https://github.com/cvusmo/hyprclock/blob/dev/assets/showcase_001.png?raw=true

**Hyprclock** is a modern, highly customizable clock application designed for use with the Hyprland window manager on Linux. It offers real-time updates, animated effects, and easy theming options, making it a perfect fit for your Hyprland setup.

## Features

- [x] **Real-time Clock**: Displays the current time and updates every second.
- [x] **Customizable Themes**: Supports different themes that can be applied through a simple configuration file or the application’s settings.
- [ ] **Animations**: Includes support for basic animations like blur and fade-in effects.
- [ ] **Easy Configuration**: Configure appearance and behavior through a user-friendly configuration file.
- [ ] **Thunderbird Integration**: Integrating with Thunderbird to manage emails and scheduling.
- [ ] **Waybar/eww Integration**: Integrating with waybar/eww bars for dynamic clock that's simple but powerful.

## Status

**Hyprclock** is currently in active development. This is a community project for [cvusmo](https://www.twitch.tv/cvusmo) and the wormhole community. If you'd like to be a part of this project or more, join the [wormhole](https://discord.gg/WZH4XNgpem). Stay tuned for updates! 

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

