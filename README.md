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
