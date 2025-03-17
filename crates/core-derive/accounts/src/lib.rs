#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Token, Type};

const ACCOUNT: &str = "account";

#[proc_macro_derive(ToAccountMetas, attributes(account))]
pub fn derive_to_account_metas(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    impl_accounts(&input).into()
}

#[derive(Debug)]
struct AccountAttr {
    is_signer: bool,
    is_mut: bool,
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    return !args.args.is_empty();
                }
            }
        }
    }
    false
}

fn impl_accounts(input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    let fields = if let syn::Data::Struct(data) = &input.data {
        if let syn::Fields::Named(fields) = &data.fields {
            &fields.named
        } else {
            panic!("Only named fields are supported")
        }
    } else {
        panic!("Only structs are supported")
    };

    let field_metas = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let attr = parse_account_attr(&f.attrs);
        let is_optional = is_option_type(&f.ty);
        (ident, attr.is_signer, attr.is_mut, is_optional)
    });

    let account_pushes = field_metas.map(|(ident, is_signer, is_mut, is_optional)| {
        let is_signer = if is_signer {
            quote! { true }
        } else {
            quote! { false }
        };

        let meta_fn = if is_mut {
            quote! { solana_program::instruction::AccountMeta::new }
        } else {
            quote! { solana_program::instruction::AccountMeta::new_readonly }
        };

        if is_optional {
            quote! {
                accounts.push(#meta_fn(
                    self.#ident.unwrap_or(crate::ID),
                    #is_signer
                ));
            }
        } else {
            quote! {
                accounts.push(#meta_fn(
                    self.#ident,
                    #is_signer
                ));
            }
        }
    });

    let expanded = quote! {
        impl solana_common_core::ToAccountMetas for #struct_name {
            fn to_account_metas(&self) -> Vec<solana_program::instruction::AccountMeta> {
                let mut accounts = Vec::new();
                #(#account_pushes)*
                accounts
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_account_attr(attrs: &[Attribute]) -> AccountAttr {
    let mut account_attr = AccountAttr { is_signer: false, is_mut: false };

    for attr in attrs {
        if !attr.path().is_ident(ACCOUNT) {
            continue;
        }

        if let syn::Meta::List(meta) = &attr.meta {
            if meta.tokens.is_empty() {
                continue;
            }
        }

        if let Err(err) = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("mut") {
                account_attr.is_mut = true;
            } else if meta.path.is_ident("signer") {
                if meta.input.peek(Token![=]) {
                    let value: syn::LitBool = meta.value()?.parse()?;
                    account_attr.is_signer = value.value();
                }
            }
            Ok(())
        }) {
            println!("Error parsing account attribute: {}", err);
        }
    }

    account_attr
}
