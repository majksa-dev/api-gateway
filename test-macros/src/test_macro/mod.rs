mod attributes;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::ItemFn;

pub fn handle(attr: TokenStream, item: ItemFn) -> Result<TokenStream, String> {
    let attributes = attributes::parse(attr)?;
    let servers = attributes.servers;
    let config = attributes.config;
    let config = Ident::new(&config, Span::call_site());
    let fn_name = &item.sig.ident;

    Ok(quote! {
        #[test]
        fn #fn_name() {
            #item
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    helper::test(
                        #servers,
                        #config,
                        #fn_name,
                    ).await;
                })
        }
    }
    .into())
}
