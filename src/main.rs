use std::time::Duration;

use notify_rust::{Notification, Timeout};
use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::{
    eval_const::value_as_string, Category, PluginSignature, Span, SyntaxShape, Value,
};

pub struct Plugin;

impl nu_plugin::Plugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("notify")
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
            .usage("sends notification with given parameters")
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        _name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let mut notification = Notification::new();
        if let Some(summary) = load_string(call, "summary") {
            notification.summary(&summary);
        }
        if let Some(body) = load_string(call, "body") {
            notification.body(&body);
        }
        if let Some(subtitle) = load_string(call, "subtitle") {
            notification.subtitle(&subtitle);
        }
        if let Some(app_name) = load_string(call, "app-name") {
            set_appname(&app_name);
            notification.appname(&app_name);
        }

        if let Some(icon) = load_string(call, "icon") {
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
                if call.has_flag("crash-on-error") {
                    return Err(LabeledError {
                        label: "Notification Exception".to_string(),
                        msg: err.to_string(),
                        span: Some(call.head),
                    });
                }
                Ok(input.clone())
            }
        }
    }
}

fn set_appname(_: &String) {}

#[cfg(target_os = "macos")]
fn set_appname(app_name: &String) {
    set_application(app_name);
}

fn main() {
    nu_plugin::serve_plugin(&mut Plugin {}, nu_plugin::MsgPackSerializer {})
}
pub fn load_string(call: &EvaluatedCall, name: &str) -> Option<String> {
    let value = call.get_flag_value(name);
    match value {
        Some(value) => match value_as_string(value, Span::unknown()) {
            Ok(val) => Some(val),
            Err(_) => None,
        },
        None => None,
    }
}
