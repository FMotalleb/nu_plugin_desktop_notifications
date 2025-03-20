# 🔔 nu_plugin_desktop_notifications  

A [Nushell](https://www.nushell.sh/) plugin for sending desktop notifications using [notify-rust](https://github.com/hoodie/notify-rust).  

---

## ✨ Features  

- **Send notifications** with custom title, body, icon, and app name.  
- **Supports macOS, Windows, and Linux (XDG Desktop)**.  
- **Configurable timeout** (for macOS and XDG desktops).  
- **Error handling** with optional crash reporting.  

---

## 📌 Usage  

### **Sending a Notification**  

```bash
notify -t "Test notification body" --summary "Test title"
```  

### **Flags**  

- `-h, --help`                 → Show help message.  
- `-s, --summary <string>`     → Title of the notification.  
- `-t, --body <string>`        → Body message of the notification.  
- `--subtitle <string>`        → Subtitle (macOS & Windows only).  
- `-a, --app-name <string>`    → App name for the notification.  
- `-i, --icon <filepath>`      → Path to an icon for the notification.  
- `--timeout <duration>`       → Duration before the notification disappears _(macOS & XDG Desktop only)_. Defaults to system settings.  
- `--crash-on-error <filepath>` → Return an error if the notification fails.  

---

## 🎯 Example: Notify on Task Completion  

Send a notification after a task completes, displaying the elapsed time:  

![image](https://github.com/FMotalleb/nu_plugin_desktop_notifications/assets/30149519/a4fbc2a9-6537-4d18-8d98-e55ebcd6b0bd)  

```bash
def "notify on done" [
    task: closure
] {
    let start = date now
    let result = do $task
    let end = date now
    let total = $end - $start | format duration sec
    let body = $"Task completed in ($total)"
    notify -s "Task Finished" -t $body
    return $result
}

notify on done { port scan 8.8.8.8 53 }
```  

---

## 🔧 Installation  

### 🚀 Recommended: Using [nupm](https://github.com/nushell/nupm)  

```bash
git clone https://github.com/FMotalleb/nu_plugin_desktop_notifications.git  
nupm install --path nu_plugin_desktop_notifications -f  
```  

### 🛠️ Manual Compilation  

```bash
git clone https://github.com/FMotalleb/nu_plugin_desktop_notifications.git  
cd nu_plugin_desktop_notifications  
cargo build -r  
register target/release/nu_plugin_desktop_notifications  
```  

### 📦 Install via Cargo (using git)  

```bash
cargo install --git https://github.com/FMotalleb/nu_plugin_desktop_notifications.git  
register ~/.cargo/bin/nu_plugin_desktop_notifications  
```  

### 📦 Install via Cargo (crates.io) _Not Recommended_  
>
> _Since I live in Iran and crates.io often restricts package updates, the version there might be outdated._  

```bash
cargo install nu_plugin_desktop_notifications  
register ~/.cargo/bin/nu_plugin_desktop_notifications  
```  
