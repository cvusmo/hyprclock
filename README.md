# Hyprclock

**Hyprclock** is a modern, highly customizable clock application designed for use with the Hyprland window manager on Linux. It offers real-time updates, animated effects, and easy theming options, making it a perfect fit for your Hyprland setup.

## Features

- **Real-time Clock**: Displays the current time and updates every second.
- **Customizable Themes**: Supports different themes that can be applied through a simple configuration file or the application’s settings.
- **Animations**: Includes support for basic animations like blur and fade-in effects.
- **Easy Configuration**: Configure appearance and behavior through a user-friendly configuration file.

## Status

**Hyprclock** is currently in active development. The application is nearing its release, but I'm still adding features and refining its functionality. Stay tuned for updates! This is my first rust built widget as I'm new to the rust language. I'm not a professional software engineer, or programmer. I'm simply a nerd who enjoys learning.

## Installation

To install **Hyprclock**, follow these steps:

1. **WAIT FOR RELEASE**

## Configuration

Example for hyprclock.conf:

    ```
    # ~/.config/hypr/hyprclock.conf

    [General]
    clock_format = "24-hour"

    [Theme]
    theme = "Materia-dark"

    [Animation]
    blur_enabled = true
    fade_in_enabled = true
    ```

## Usage

Once installed, you can launch Hyprclock from your application menu or by using a keybind in Hyprland. You can also add it to your autostart configuration to have it launch on startup.

## Running Hyprclock

    ```
    bash
    hyprclock
    ```

## Auto-start

Add the following line to your hyprland.conf

    ```
    bash
    exec-once = hyprclock
    ```
