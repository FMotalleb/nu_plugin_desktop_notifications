use notify_rust::Notification;
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
                "summery of the notification",
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
                "subtitle of the notification",
                None,
            )
            .named(
                "app name",
                SyntaxShape::String,
                "app name of the notification",
                Some('a'),
            )
            .named(
                "icon",
                SyntaxShape::Filepath,
                "icon of the notification",
                Some('i'),
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
            notification.body(&subtitle);
        }
        if let Some(app_name) = load_string(call, "app name") {
            notification.appname(&app_name);
        }
        if let Some(icon) = load_string(call, "icon") {
            notification.icon(&icon);
        } else {
            notification.auto_icon();
        }

        let _ = notification.show();
        Ok(input.clone())
    }
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
