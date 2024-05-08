mod test_macro;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);
    test_macro::handle(attr, item_fn).unwrap_or_else(|e| quote!(compile_error!(#e);).into())
}
