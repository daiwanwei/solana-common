#[allow(unused_extern_crates)]
extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Instructions, attributes(instruction))]
pub fn instructions_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data_enum) = input.data else {
        panic!("Instructions only works on enums");
    };

    let mut output = proc_macro2::TokenStream::new();

    for variant in data_enum.variants {
        let variant_ident = variant.ident;

        let name_in_snake_case = if let Some(rename) = find_rename_attr(&variant.attrs) {
            rename
        } else {
            variant_ident.to_string().to_case(Case::Snake)
        };

        let discriminator = generate_discriminator(SIGHASH_GLOBAL_NAMESPACE, &name_in_snake_case);

        let impls = quote! {
            impl ::anchor_trait::Discriminator for #variant_ident {
                const DISCRIMINATOR: [u8; 8] = #discriminator;
            }

            impl ::anchor_trait::InstructionData for #variant_ident {}
        };

        let expanded = match variant.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let fields = named.iter().map(|f| {
                    let field_ident = &f.ident;
                    let field_type = &f.ty;
                    quote! {
                        pub #field_ident: #field_type
                    }
                });

                quote! {
                    #[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
                    pub struct #variant_ident {
                        #(#fields,)*
                    }

                    #impls
                }
            }
            Fields::Unit => {
                quote! {
                    #[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
                    pub struct #variant_ident {}

                    #impls
                }
            }
            Fields::Unnamed(_) => {
                panic!("Unnamed fields are not supported");
            }
        };

        output.extend(expanded);
    }

    output.into()
}

fn find_rename_attr(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident(INSTRUCTION) {
            continue;
        }

        if let syn::Meta::List(meta) = &attr.meta {
            if meta.tokens.is_empty() {
                continue;
            }
        }

        let mut result = None;
        if let Err(err) = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                if meta.input.peek(syn::Token![=]) {
                    let value: syn::LitStr = meta.value()?.parse()?;
                    result = Some(value.value());
                }
            }
            Ok(())
        }) {
            println!("Error parsing rename attribute: {}", err);
        }
        if result.is_some() {
            return result;
        }
    }
    None
}

fn generate_discriminator(namespace: &str, name: &str) -> proc_macro2::TokenStream {
    let mut hasher = Sha256::new();
    hasher.update(format!("{namespace}:{name}").as_bytes());
    let discriminator = hasher.finalize();
    format!("{:?}", &discriminator[..8]).parse().unwrap()
}

const SIGHASH_GLOBAL_NAMESPACE: &str = "global";

const INSTRUCTION: &str = "instruction";
