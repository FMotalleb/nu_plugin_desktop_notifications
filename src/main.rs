use nu_plugin::{serve_plugin, Plugin};

use crate::notify::NotifyCommand;
mod notify;
pub struct NotifyPlugin;

impl Plugin for NotifyPlugin {
    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(NotifyCommand::new())]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}

fn main() {
    serve_plugin(&mut NotifyPlugin {}, nu_plugin::MsgPackSerializer {})
}
