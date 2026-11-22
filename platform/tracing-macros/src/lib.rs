extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, ItemFn, Meta, Token};

/// Procedural macro wrapper for function instrumentation.
///
/// Supports an optional `domain = "sensor" | "audio" | "perf"` attribute argument:
/// - Default (`perf`): emits `::tracing_defmt::instrument` gated on `feature = "trace-perf"`.
/// - `domain = "sensor"`: gates span emission on `feature = "trace-sensor-spans"` so `PERF`
///   mode excludes sensor polling spans while keeping AST instrumentation attributes intact.
/// - `domain = "audio"`: gates span emission on `feature = "trace-audio"`.
#[proc_macro_attribute]
pub fn instrument(args: TokenStream, item: TokenStream) -> TokenStream {
    let args_parsed = if args.is_empty() {
        Punctuated::<Meta, Token![,]>::new()
    } else {
        parse_macro_input!(args with Punctuated::<Meta, Token![,]>::parse_terminated)
    };

    let mut item_fn = parse_macro_input!(item as ItemFn);
    let fn_name = item_fn.sig.ident.to_string();

    let mut custom_name = None;
    let mut custom_domain = None;
    let mut skip_all = false;
    let mut other_args = Vec::new();

    for meta in args_parsed {
        let path = meta.path();
        if path.is_ident("name") {
            if let Meta::NameValue(nv) = &meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = &nv.value
                {
                    custom_name = Some(lit.value());
                } else if let syn::Expr::Macro(syn::ExprMacro {
                    mac:
                        syn::Macro {
                            path: mac_path,
                            tokens,
                            ..
                        },
                    ..
                }) = &nv.value
                {
                    if mac_path.is_ident("stringify") {
                        custom_name = Some(tokens.to_string().trim().to_string());
                    }
                }
            }
        } else if path.is_ident("domain") {
            if let Meta::NameValue(nv) = &meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = &nv.value
                {
                    custom_domain = Some(lit.value());
                }
            }
        } else if path.is_ident("skip_all") {
            skip_all = true;
        } else if skip_all && path.is_ident("skip") {
            // `skip_all` supersedes explicit `skip(...)` lists.
        } else {
            other_args.push(meta);
        }
    }

    if skip_all {
        other_args.retain(|m| !m.path().is_ident("skip"));
        let param_idents: Vec<syn::Ident> = item_fn
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let syn::FnArg::Typed(pat_type) = arg {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        return Some(pat_ident.ident.clone());
                    }
                }
                None
            })
            .collect();
        if !param_idents.is_empty() {
            let skip_meta: Meta = parse_quote! { skip(#(#param_idents),*) };
            other_args.push(skip_meta);
        }
    }

    let feature_gate = match custom_domain.as_deref() {
        Some("sensor") => "trace-sensor-spans",
        Some("audio") => "trace-audio",
        _ => "trace-perf",
    };

    let name_val = custom_name.unwrap_or(fn_name);
    let other_args_tokens: Vec<proc_macro2::TokenStream> =
        other_args.iter().map(|arg| quote! { #arg }).collect();

    // Check if the function has .data.core1_func link_section in its attributes
    let mut is_core1 = false;
    for attr in &item_fn.attrs {
        let attr_str = quote! { #attr }.to_string();
        if attr_str.contains(".data.core1_func") {
            is_core1 = true;
            break;
        }
    }

    let core0_prefixed_name = format!("Core 0: {}", name_val);

    if item_fn.sig.constness.is_none() {
        let host_guard_stmt: syn::Stmt = parse_quote! {
            #[cfg(all(not(all(target_arch = "arm", target_os = "none")), feature = #feature_gate))]
            let _host_trace_span_guard = ::platform::tracing::HostSpanGuard::enter(#core0_prefixed_name, module_path!());
        };
        item_fn.block.stmts.insert(0, host_guard_stmt);
    }

    if is_core1 {
        let core1_prefixed_name = format!("Core 1: {}", name_val);

        quote! {
            #[cfg_attr(all(feature = #feature_gate, target_arch = "arm"), ::tracing_defmt::instrument(name = #core1_prefixed_name, #(#other_args_tokens),*))]
            #[cfg_attr(all(feature = #feature_gate, not(target_arch = "arm")), ::tracing_defmt::instrument(name = #core0_prefixed_name, #(#other_args_tokens),*))]
            #item_fn
        }
    } else {
        quote! {
            #[cfg_attr(feature = #feature_gate, ::tracing_defmt::instrument(name = #core0_prefixed_name, #(#other_args_tokens),*))]
            #item_fn
        }
    }
    .into()
}

/// Procedural macro marker attribute for controller context structures.
#[proc_macro_attribute]
pub fn controller_context(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}
