extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Meta, Token};

/// Procedural macro wrapper for function instrumentation.
/// Supports `#[instrument(core1 = "feature")]` to conditionally segment CPU core activity.
#[proc_macro_attribute]
pub fn instrument(args: TokenStream, item: TokenStream) -> TokenStream {
    let args_parsed = if args.is_empty() {
        Punctuated::<Meta, Token![,]>::new()
    } else {
        parse_macro_input!(args with Punctuated::<Meta, Token![,]>::parse_terminated)
    };

    let item_fn = parse_macro_input!(item as ItemFn);
    let fn_name = item_fn.sig.ident.to_string();

    let mut selected_core: Option<(String, String)> = None; // (core_name, feature_name)
    let mut custom_name = None;
    let mut other_args = Vec::new();

    for meta in args_parsed {
        let path = meta.path();
        if let Some(ident) = path.get_ident() {
            let ident_str = ident.to_string();
            if ident_str.starts_with("core")
                && ident_str["core".len()..]
                    .chars()
                    .all(|c| c.is_ascii_digit())
            {
                let core_num = &ident_str["core".len()..];
                let core_name = format!("Core {}", core_num);

                let feature_name = match &meta {
                    Meta::NameValue(nv) => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        }) = &nv.value
                        {
                            lit.value()
                        } else {
                            ident_str.clone()
                        }
                    }
                    _ => ident_str.clone(),
                };

                selected_core = Some((core_name, feature_name));
                continue;
            }
        }

        if path.is_ident("name") {
            if let Meta::NameValue(nv) = meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = nv.value
                {
                    custom_name = Some(lit.value());
                }
            }
        } else {
            other_args.push(meta);
        }
    }

    let name_val = custom_name.unwrap_or(fn_name);
    let other_args_tokens: Vec<proc_macro2::TokenStream> =
        other_args.iter().map(|arg| quote! { #arg }).collect();

    if let Some((core_name, feature_name)) = selected_core {
        let core_prefixed_name = format!("{}: {}", core_name, name_val);
        let core0_prefixed_name = format!("Core 0: {}", name_val);

        quote! {
            #[cfg_attr(all(feature = "tracing", feature = #feature_name), ::tracing_defmt::instrument(name = #core_prefixed_name, #(#other_args_tokens),*))]
            #[cfg_attr(all(feature = "tracing", not(feature = #feature_name)), ::tracing_defmt::instrument(name = #core0_prefixed_name, #(#other_args_tokens),*))]
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
