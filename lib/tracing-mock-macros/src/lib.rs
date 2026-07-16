extern crate proc_macro;
use proc_macro::TokenStream;

/// A no-op implementation of `#[instrument]` that returns the item unmodified.
#[proc_macro_attribute]
pub fn instrument(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}
