use rinja::Template;
use serde::Deserialize;

#[allow(dead_code)]
pub const SUBCOMMAND_CRATES: &[&str] = &["controller", "platform"];

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

#[derive(Clone, Debug, serde::Serialize)]
pub struct SubcommandInfo {
    pub name: String,
    pub doc: String,
}

fn scan_items(
    items: &[syn::Item],
    enums: &mut std::collections::HashMap<String, Vec<SubcommandInfo>>,
) {
    for item in items {
        match item {
            syn::Item::Macro(item_macro) => {
                let is_subcommand_enum = item_macro.mac.path.is_ident("subcommand_enum")
                    || item_macro
                        .mac
                        .path
                        .segments
                        .last()
                        .is_some_and(|s| s.ident == "subcommand_enum");
                if is_subcommand_enum {
                    let enum_parser = |input: syn::parse::ParseStream| {
                        let item_enum = input.parse::<syn::ItemEnum>()?;
                        let _ = input.parse::<proc_macro2::TokenStream>();
                        Ok(item_enum)
                    };
                    use syn::parse::Parser as _;
                    if let Ok(item_enum) = enum_parser.parse2(item_macro.mac.tokens.clone()) {
                        let enum_name = item_enum.ident.to_string();
                        let mut variants = Vec::new();
                        for variant in item_enum.variants {
                            let var_ident = variant.ident.to_string();
                            let mut var_name = String::new();
                            let mut has_explicit = false;
                            if let Some((
                                _,
                                syn::Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Str(lit_str),
                                    ..
                                }),
                            )) = &variant.discriminant
                            {
                                var_name = lit_str.value();
                                has_explicit = true;
                            }
                            if !has_explicit {
                                for (i, c) in var_ident.char_indices() {
                                    if c.is_uppercase() {
                                        if i > 0 {
                                            var_name.push('_');
                                        }
                                        var_name.push(c.to_ascii_lowercase());
                                    } else {
                                        var_name.push(c);
                                    }
                                }
                            }

                            // Extract doc comments
                            let mut doc = String::new();
                            for attr in &variant.attrs {
                                if attr.path().is_ident("doc") {
                                    if let syn::Meta::NameValue(syn::MetaNameValue {
                                        value:
                                            syn::Expr::Lit(syn::ExprLit {
                                                lit: syn::Lit::Str(lit_str),
                                                ..
                                            }),
                                        ..
                                    }) = &attr.meta
                                    {
                                        doc = lit_str.value().trim().to_string();
                                    }
                                }
                            }
                            variants.push(SubcommandInfo {
                                name: var_name,
                                doc,
                            });
                        }
                        enums.insert(enum_name, variants);
                    }
                }
            }
            syn::Item::Mod(item_mod) => {
                if let Some((_, content)) = &item_mod.content {
                    scan_items(content, enums);
                }
            }
            _ => {}
        }
    }
}

pub fn parse_subcommand_enums(
    dir: &std::path::Path,
    enums: &mut std::collections::HashMap<String, Vec<SubcommandInfo>>,
) {
    if !dir.exists() {
        return;
    }
    if dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    parse_subcommand_enums(&path, enums);
                } else if path.extension().is_some_and(|ext| ext == "rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(file) = syn::parse_str::<syn::File>(&content) {
                            scan_items(&file.items, enums);
                        }
                    }
                }
            }
        }
    }
}

pub fn find_workspace_root() -> std::path::PathBuf {
    let mut path = std::env::current_dir().unwrap();
    loop {
        if path.join("pyproject.toml").exists() {
            return path;
        }
        if !path.pop() {
            panic!("Could not locate workspace root (looking for pyproject.toml)!");
        }
    }
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

    pub fn help_string(
        &self,
        subcommands_map: &std::collections::HashMap<String, Vec<SubcommandInfo>>,
    ) -> String {
        let enum_name = self
            .subcommand_type
            .split("::")
            .last()
            .unwrap_or(&self.subcommand_type);
        if let Some(subs) = subcommands_map.get(enum_name) {
            if !subs.is_empty() {
                let sub_list = subs
                    .iter()
                    .map(|sub| format!("{} {}", self.cmd_name, sub.name))
                    .collect::<Vec<_>>()
                    .join(", ");
                return format!("{} ({})", self.help, sub_list);
            }
        }
        self.help.clone()
    }

    pub fn get_subcommands(
        &self,
        subcommands_map: &std::collections::HashMap<String, Vec<SubcommandInfo>>,
    ) -> Vec<SubcommandInfo> {
        let enum_name = self
            .subcommand_type
            .split("::")
            .last()
            .unwrap_or(&self.subcommand_type);
        subcommands_map.get(enum_name).cloned().unwrap_or_default()
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
