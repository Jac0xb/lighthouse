extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput};

#[proc_macro_derive(Optionize)]
pub fn optionize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let optional_name = syn::Ident::new(&format!("Optional{}", name), name.span());
    let attrs = &input.attrs;

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => panic!("Optionize macro only works with structs"),
    };

    let optional_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: Option<#ty>, }
    });

    let derive_attrs: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("derive"))
        .collect();

    let expanded = quote! {
        // Original struct with its attributes
        // #( #attrs )*
        // pub struct #name {
        //     #( #fields, )*
        // }

        // Optional variant of the struct with the same derive attributes
        #( #derive_attrs )*
        pub struct #optional_name {
            #( #optional_fields )*
        }
    };

    TokenStream::from(expanded)
}
