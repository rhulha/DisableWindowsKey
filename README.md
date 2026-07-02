# Disable Windows Key

A tiny Windows system-tray app that disables the **left Windows key** while it runs.

No configuration, no options — it starts blocking the key immediately and keeps
running in the system tray until you quit it.

## Behavior

- Installs a low-level keyboard hook on startup that swallows every left Windows
  key press. The right Windows key and all other keys are untouched.
- Lives in the system tray with an icon that shows the current state:
  - **Blue Windows flag with a red "no" symbol** — blocking is on.
  - **Grey Windows flag** — blocking is off, the key works normally.
- **Right-click** the tray icon for a menu with **Enable/Disable** (toggles
  blocking; the icon and label update to match) and **Quit** (restores the key
  and exits).

## Build

Requires [Rust](https://rustup.rs/) on Windows.

```
cargo build --release
```

The binary is produced at `target\release\disablewinkey.exe`.

## Run

```
target\release\disablewinkey.exe
```

> If you want the key to be blocked inside apps that run elevated (as
> administrator), run this app as administrator too. A low-level keyboard hook
> from a normal-privilege process cannot intercept keys sent to a
> higher-privilege window.

## License

MIT — see [LICENSE](LICENSE).
