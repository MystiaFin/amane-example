# amane-example

A small but complete desktop shell built with [Amane](https://github.com/MystiaFin/amane), meant to be read. Every file does one thing, so you can open any of them and see how that part works.

It has:

- a wallpaper on every monitor
- a bar on every monitor, with a Linux menu button, workspaces, a clock, volume, network, bluetooth, notifications and battery
- a start menu: sleep, lock, log out and shut down on the left, app search on the right
- small panels for wifi, bluetooth and notifications
- notification popups
- a lock screen

The colors are [Catppuccin Mocha](https://catppuccin.com/palette/), written as hex right where they're used.

## Running it

You need the `amane` CLI first, see [Amane's installation guide](https://github.com/MystiaFin/amane#installation).

```sh
git clone https://github.com/MystiaFin/amane-example.git
cd amane-example
amane target .
amane dev
```

`amane target .` tells Amane to build this folder instead of `~/.config/amane`. `amane dev` builds the shell, starts it, and restarts it every time you save a file.

Amane writes its own `Cargo.toml` here on the first build, with paths from your machine. That's why it's in `.gitignore`.

### Before you start

- Stop your notification daemon (mako, dunst, swaync), or the popups stay empty. Only one program can show notifications at a time.
- Workspaces work on niri, Hyprland and Sway.
- The wallpaper is `src/wallpaper-example/wallpaper_2.jpg`. To use your own, change `WALLPAPER` in `src/wallpaper.rs`, either to a path inside this project or to a full path. If the image is missing you get a plain dark background.
- Read [Testing safely](https://mystiafin.github.io/amane/lock_screen.html#testing-safely) before trying the lock screen. A broken lock screen can lock you out of your session.

## Keybinds

The start menu and the lock screen also open over IPC, so you can bind them in your compositor.

niri:

```kdl
binds {
    Mod+D { spawn "amane" "ipc" "call" "launcher"; }
    Mod+Alt+L { spawn "amane" "ipc" "call" "lock"; }
}
```

Hyprland:

```ini
bind = SUPER, D, exec, amane ipc call launcher
bind = SUPER ALT, L, exec, amane ipc call lock
```

Sway:

```
bindsym $mod+d exec amane ipc call launcher
bindsym $mod+Alt+l exec amane ipc call lock
```

Click the Linux button in the top left to open the start menu. Type to search, press Enter to open the first match, or press Escape to close it. The buttons on the left use `systemctl` for sleep and shut down, and `loginctl` to log out. Log out and shut down turn red and ask "sure?" first, so they need a second click. In the bar, click the volume to mute it and scroll over it to change it.

Click the network, `bt` or `notifs` label in the bar to open its panel, and click it again (or press Escape) to close it. In the wifi panel, click a network to join or leave it. New secured networks ask for their password first. In the bluetooth panel, click a device to pair, connect or disconnect it. Devices that need a pairing code have to be paired once with `bluetoothctl`.

Wifi needs NetworkManager, and bluetooth needs BlueZ.

## Where things are

```
src/
├── main.rs            puts the windows together, and the ipc handlers
├── files.rs           finds files inside this project, wherever the shell was started
├── clock.rs           Clock service, shared by the bar and the lock screen
├── bar.rs             the bar window: left, center and right
├── bar/
│   ├── apps.rs        Linux button that opens the start menu
│   ├── workspaces.rs  one button per workspace on this monitor
│   ├── clock.rs       date and time
│   └── status.rs      volume, network, bluetooth, notifications, battery
├── panel.rs           panel window, which panel is open, shared header and list item
├── panel/
│   ├── wifi.rs        networks and the password box
│   ├── bluetooth.rs   devices
│   └── notifications.rs  every notification, and clear all
├── launcher.rs        start menu window, its state and search
├── launcher/
│   ├── power.rs       sleep, lock, log out, shut down
│   └── entry.rs       one app in the list
├── notifications.rs   popups in the top right corner
├── lock_screen.rs     clock and password box
└── wallpaper.rs       an image behind everything
assets/
└── linux.svg          the menu button's icon
```

A good order to read them in: `main.rs`, `bar.rs`, then the files in `bar/`, then `launcher.rs`, `panel.rs` and the files in `panel/`, `notifications.rs`, `lock_screen.rs` and `wallpaper.rs`.

## Credits

The Linux icon is from [Simple Icons](https://simpleicons.org/) (CC0).
