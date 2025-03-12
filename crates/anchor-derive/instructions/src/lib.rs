#[allow(unused_extern_crates)]
extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Instructions)]
pub fn instructions_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data_enum) = input.data else {
        panic!("Instructions only works on enums");
    };

    let mut output = proc_macro2::TokenStream::new();

    for variant in data_enum.variants {
        let variant_ident = variant.ident;
        let name_in_snake_case =
            normalize_snake_case(variant_ident.to_string().to_case(Case::Snake));

        let discriminator = generate_discriminator(SIGHASH_GLOBAL_NAMESPACE, &name_in_snake_case);

        let impls = quote! {
            impl Discriminator for #variant_ident {
                const DISCRIMINATOR: [u8; 8] = #discriminator;
            }

            impl InstructionData for #variant_ident {}
        };

        let Fields::Named(FieldsNamed { named, .. }) = variant.fields else {
            continue;
        };

        let fields = named.iter().map(|f| {
            let field_ident = &f.ident;
            let field_type = &f.ty;
            quote! {
                pub #field_ident: #field_type
            }
        });

        let expanded = if fields.len() == 0 {
            quote! {
                #[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
                pub struct #variant_ident {}

                #impls
            }
        } else {
            quote! {
                #[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
                pub struct #variant_ident {
                    #(#fields,)*
                }

                #impls
            }
        };

        output.extend(expanded);
    }

    output.into()
}

fn generate_discriminator(namespace: &str, name: &str) -> proc_macro2::TokenStream {
    let mut hasher = Sha256::new();
    hasher.update(format!("{namespace}:{name}").as_bytes());
    let discriminator = hasher.finalize();
    format!("{:?}", &discriminator[..8]).parse().unwrap()
}

// TODO: Remove this once the issue is fixed in convert_case
fn normalize_snake_case(input: String) -> String { input.replace("_v_", "_v") }

const SIGHASH_GLOBAL_NAMESPACE: &str = "global";
