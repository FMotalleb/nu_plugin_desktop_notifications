# [WIP] nu_plugin_desktop_notifications

A [nushell](https://www.nushell.sh/)  plugin to send notification to desktop using [notify-rust](https://github.com/hoodie/notify-rust)

# Examples

```bash
Flags:
  -h, --help - Display the help message for this command
  -s, --summary <String> - summery of the notification
  -t, --body <String> - body of the notification
  --subtitle <String> - subtitle of the notification
  -a, --app name <String> - app name of the notification
  -i, --icon <Filepath> - path to icon of the notification
  --timeout <Duration> - duration of the notification [XDG Desktops only] (defaults to system default)
  --crash on error <Filepath> - returns notification error if encountered
```

```bash
~> notify -t "test notification body" --summary "test title"
```

# Installing

* via git

```bash
git clone https://github.com/FMotalleb/nu_plugin_desktop_notifications.git
cd nu_plugin_desktop_notifications
cargo build -r --features=all-decoders
register target/debug/nu_plugin_desktop_notifications 
```

* or using cargo

```bash
cargo install nu_plugin_desktop_notifications
register ~/.cargo/bin/nu_plugin_desktop_notifications
```
