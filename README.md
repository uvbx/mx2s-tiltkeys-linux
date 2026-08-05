# MX2S Tiltkeys Linux

This is a simple, small rust utility that maps the MX Anywhere 2S's horizontally tilting scrollwheel buttons to volume functionality.

## Why
Generally, customizing the MX2S's wheel tilt buttons requires Logitech's proprietary software, which is only available on Windows.

As a Linux user, I wanted to fully utilize my mouse.

This utility simply listens for horizontal wheel events (using evdev); then translates that into media keypresses through a virtual keyboard (created with uinput).


## Features
 · Automatically detects the mouse, and reconnects when/if the mouse sleeps

 · Maps horizontal wheel movement to volume controls (left - down; right - up)
 
 · Lightweight, and simple program


## Requirements
 · Linux

 · `uinput`

 · Access to `/dev/input/event*/`

 · Rust


## Installation
please note before running the installer!
the installer automatically performs the following:
 · builds/compiles the rust binary

 · installs the binary to /usr/local/bin

 · configures uninput permissions

 · creates a user systemd service

 · adds the user to the input group



 1· make the included install script executable
```bash
      sudo chmod +x install.sh
```
 2· run the install script with
``` bash
      ./install.sh
```