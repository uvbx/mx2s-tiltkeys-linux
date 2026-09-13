# MX2S Tiltkeys Linux

This is a simple rust utility that allows configuration of the the MX Anywhere's tilting scrollwheel.
Generally, customizing the MX2S's wheel tilt buttons requires Logitech's proprietary software, which is only available on Windows.

As a Linux user, I wanted to fully utilize my mouse.

This utility simply listens for horizontal wheel events; then translates that into media keypresses through a virtual keyboard.


## Features
 · Automatically detects the mouse, and reconnects when/if the mouse sleeps

 · Maps horizontal wheel movement to configuration

 · Lightweight


## Requirements

 · `uinput`

 · Access to `/dev/input/event*/`

 · Rust (to build from source)


## Installation
please note before running the installer!
the installer automatically performs the following:

 · builds/compiles the rust binary

 · generates a base config file at ~/.config/mx2s-tiltkeys

 · installs the binary to /usr/local/bin

 · configures uinput permissions

 · creates a user systemd service

 · adds the user to the input group



 
run the install script with
      ``` bash
      ./install.sh   
```
```
```
