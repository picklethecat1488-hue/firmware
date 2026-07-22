extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Meta, Token};

/// Procedural macro wrapper for function instrumentation.
#[proc_macro_attribute]
pub fn instrument(args: TokenStream, item: TokenStream) -> TokenStream {
    let args_parsed = if args.is_empty() {
        Punctuated::<Meta, Token![,]>::new()
    } else {
        parse_macro_input!(args with Punctuated::<Meta, Token![,]>::parse_terminated)
    };

    let item_fn = parse_macro_input!(item as ItemFn);
    let fn_name = item_fn.sig.ident.to_string();

    let mut custom_name = None;
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
        } else {
            other_args.push(meta);
        }
    }

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

    if is_core1 {
        let core1_prefixed_name = format!("Core 1: {}", name_val);
        let core0_prefixed_name = format!("Core 0: {}", name_val);

        quote! {
            #[cfg_attr(all(feature = "tracing", target_arch = "arm"), ::tracing_defmt::instrument(name = #core1_prefixed_name, #(#other_args_tokens),*))]
            #[cfg_attr(all(feature = "tracing", not(target_arch = "arm")), ::tracing_defmt::instrument(name = #core0_prefixed_name, #(#other_args_tokens),*))]
            #item_fn
        }
    } else {
        let core0_prefixed_name = format!("Core 0: {}", name_val);

        quote! {
            #[cfg_attr(feature = "tracing", ::tracing_defmt::instrument(name = #core0_prefixed_name, #(#other_args_tokens),*))]
            #item_fn
        }
    }
    .into()
}
