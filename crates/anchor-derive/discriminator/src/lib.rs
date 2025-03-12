#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn discriminator(args: TokenStream, input: TokenStream) -> TokenStream {
    let account_strct = parse_macro_input!(input as syn::ItemStruct);
    let name = &account_strct.ident;
    let name_str = name.to_string();

    let namespace = args.to_string();

    let discriminator: proc_macro2::TokenStream = generate_discriminator(&namespace, &name_str);

    let expanded = quote! {
        #account_strct

        impl ::anchor_trait::Discriminator for #name {
            const DISCRIMINATOR: [u8; 8] = #discriminator;
        }
    };

    TokenStream::from(expanded)
}

fn generate_discriminator(namespace: &str, name: &str) -> proc_macro2::TokenStream {
    let mut hasher = Sha256::new();
    hasher.update(format!("{namespace}:{name}").as_bytes());
    let discriminator = hasher.finalize();
    format!("{:?}", &discriminator[..8]).parse().unwrap()
}
