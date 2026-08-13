use rinja::Template;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CliResolverField {
    pub associated_type: String,
    pub field: String,
    pub resolve_fn: String,
    pub doc: String,
    pub bounds: String,
    pub type_lifetime: Option<String>,
}

impl CliResolverField {
    pub fn bounds(&self) -> String {
        let lifetime = self.type_lifetime.as_deref().unwrap_or("'static");
        format!("{} + {}", self.bounds, lifetime)
    }
}

#[derive(Deserialize, Clone)]
pub struct CliArg {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
    pub help: String,
    pub attributes: Option<Vec<String>>,
}

impl CliArg {
    pub fn attributes_slice(&self) -> Vec<String> {
        if let Some(ref attrs) = self.attributes {
            attrs.clone()
        } else if self.name != "arg1"
            && self.name != "arg2"
            && self.name != "arg3"
            && self.name != "target"
        {
            vec![format!("#[arg(long = \"{}\")]", self.name)]
        } else {
            vec![]
        }
    }

    pub fn rust_type(&self) -> String {
        match self.arg_type.as_str() {
            "string" => "Option<&'a str>".to_string(),
            "int" => "Option<i32>".to_string(),
            "float" => "Option<f32>".to_string(),
            "bool" => "Option<bool>".to_string(),
            custom => resolve_crate_path(custom),
        }
    }

    pub fn rust_type_sample(&self) -> String {
        self.rust_type()
            .replace("$crate::", "controller::")
            .replace("&'a str", "&str")
    }
}

fn resolve_crate_path(custom: &str) -> String {
    let mut result = custom.to_string();
    if let Some(idx) = result.find("_controller::") {
        let bytes = result.as_bytes();
        let mut start = idx;
        while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
            start -= 1;
        }
        if !(start >= 8 && &result[start - 8..start] == "$crate::") {
            result.insert_str(start, "$crate::");
        }
    }
    result
}

#[derive(Deserialize, Clone)]
pub struct CliCommand {
    pub group: String,
    pub cmd_name: String,
    pub variant: String,
    pub subcommand_type: String,
    #[serde(default)]
    pub async_cli: bool,
    pub handler: String,
    pub help: String,
    pub args: Option<Vec<CliArg>>,
}

impl CliCommand {
    pub fn args_slice(&self) -> &[CliArg] {
        self.args.as_deref().unwrap_or(&[])
    }

    pub fn handler_short_name(&self) -> &str {
        self.handler.split("::").last().unwrap()
    }

    pub fn subcommand_type_path(&self) -> String {
        if self.subcommand_type.contains("::") && !self.subcommand_type.starts_with("platform::") {
            format!("controller::{}", self.subcommand_type)
        } else {
            self.subcommand_type.clone()
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct ShellConfigToml {
    #[serde(default)]
    pub cli_resolver_fields: Vec<CliResolverField>,
    #[serde(default)]
    pub cli_commands: Vec<CliCommand>,
}

/// Rinja template for rendering a sample CLI implementation.
#[derive(Template)]
#[template(path = "sample_cli.rs.jinja", escape = "none")]
pub struct SampleCliTemplate {
    pub cli_commands: Vec<CliCommand>,
}

/// Rinja template for rendering a single CLI handler function skeleton.
#[derive(Template)]
#[template(path = "cli_handler_skeleton.rs.jinja", escape = "none")]
pub struct CliHandlerSkeletonTemplate {
    pub cmd: CliCommand,
}
