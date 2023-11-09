# nu_plugin_desktop_notifications

A [nushell](https://www.nushell.sh/)  plugin to send notification to desktop using [notify-rust](https://github.com/hoodie/notify-rust)

# Examples

```bash
Flags:
  -h, --help - Display the help message for this command
  -s, --summary <String> - summary of the notification
  -t, --body <String> - body of the notification
  --subtitle <String> - subtitle of the notification [macOS and windows only]
  -a, --app-name <String> - app name of the notification
  -i, --icon <Filepath> - path to icon of the notification
  --timeout <Duration> - duration of the notification [XDG Desktops only] (defaults to system default)
  --crash-on-error <Filepath> - returns notification error if encountered
```

```bash
~> notify -t "test notification body" --summary "test title"
```

* send a notification after doing a task and display the time in the notification
  
![image](https://github.com/FMotalleb/nu_plugin_desktop_notifications/assets/30149519/a4fbc2a9-6537-4d18-8d98-e55ebcd6b0bd)

```bash
def "notify on done" [
    task: closure
] {
    let start = date now
    let result = do $task
    let end = date now
    let total = $end - $start | format duration sec
    let body = $"given task finished in ($total)"
    notify -s "task is done" -t $body
    return $result
}

notify on done { port scan 8.8.8.8 53 }
```

# Installing

* via git

```bash
git clone https://github.com/FMotalleb/nu_plugin_desktop_notifications.git
cd nu_plugin_desktop_notifications
cargo build -r
register target/release/nu_plugin_desktop_notifications
```

* or using cargo

```bash
cargo install nu_plugin_desktop_notifications
register ~/.cargo/bin/nu_plugin_desktop_notifications
```
