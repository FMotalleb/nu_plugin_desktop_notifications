use std::time::Duration;

use notify_rust::{Notification, Timeout};
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, LabeledError, Signature, SyntaxShape, Value};

use crate::NotifyPlugin;

#[derive(Default)]
pub struct NotifyCommand;
impl NotifyCommand {
    pub(crate) fn new() -> NotifyCommand {
        NotifyCommand {}
    }
    pub fn load_string(call: &EvaluatedCall, name: &str) -> Option<String> {
        let value = call.get_flag_value(name);
        match value {
            Some(Value::String { val, .. }) => Some(val),
            _ => None,
        }
    }
}

impl SimplePluginCommand for NotifyCommand {
    type Plugin = NotifyPlugin;

    fn name(&self) -> &str {
        "notify"
    }

    fn signature(&self) -> Signature {
        Signature::build("notify")
            .named(
                "summary",
                SyntaxShape::String,
                "summary of the notification",
                Some('s'),
            )
            .named(
                "body",
                SyntaxShape::String,
                "body of the notification",
                Some('t'),
            )
            .named(
                "subtitle",
                SyntaxShape::String,
                "subtitle of the notification [macOS only]",
                None,
            )
            .named(
                "app-name",
                SyntaxShape::String,
                "app name of the notification",
                Some('a'),
            )
            .named(
                "icon",
                SyntaxShape::Filepath,
                "path to the icon of the notification",
                Some('i'),
            )
            .named(
                "timeout",
                SyntaxShape::Duration,
                "duration of the notification [XDG Desktops only] (defaults to system default)",
                None,
            )
            .named(
                "crash-on-error",
                SyntaxShape::Filepath,
                "returns notification error if encountered",
                None,
            )
            .category(Category::Experimental)
    }

    fn description(&self) -> &str {
        "Send a desktop notification with customizable parameters."
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let mut notification = Notification::new();
        if let Some(summary) = Self::load_string(call, "summary") {
            notification.summary(&summary);
        }
        if let Some(body) = Self::load_string(call, "body") {
            notification.body(&body);
        }
        if let Some(subtitle) = Self::load_string(call, "subtitle") {
            notification.subtitle(&subtitle);
        }
        if let Some(app_name) = Self::load_string(call, "app-name") {
            notification.appname(&app_name);
        }

        if let Some(icon) = Self::load_string(call, "icon") {
            notification.icon(&icon);
        } else {
            notification.auto_icon();
        }

        if let Some(duration_value) = call.get_flag_value("timeout") {
            match duration_value.as_duration() {
                Ok(timeout) => {
                    if let Ok(nanos) = timeout.try_into() {
                        let duration = Timeout::from(Duration::from_nanos(nanos));
                        notification.timeout(duration);
                    }
                }
                Err(_) => {}
            }
        }

        match notification.show() {
            Ok(_) => Ok(input.clone()),
            Err(err) => {
                if let Ok(true) = call.has_flag("crash-on-error") {
                    return Err(LabeledError::new(err.to_string())
                        .with_label("Notification Exception", call.head));
                }
                Ok(input.clone())
            }
        }
    }
}
