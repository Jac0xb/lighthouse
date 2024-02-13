extern crate proc_macro;

use proc_macro::TokenStream;
// use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

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

// #[proc_macro_derive(FieldEnum)]
// pub fn field_enum(input: TokenStream) -> TokenStream {
//     // Parse the input tokens into a syntax tree
//     let input = parse_macro_input!(input as DeriveInput);

//     // Extract the struct name and data
//     let name = input.ident;
//     let data = input.data;

//     match data {
//         Data::Struct(DataStruct {
//             fields: Fields::Named(fields),
//             ..
//         }) => {
//             let field_names = fields.named.iter().map(|f| &f.ident);

//             // Generate enum variants from field names
//             let enum_name = quote::format_ident!("{}Fields", name);
//             let enum_tokens = quote! {
//                 pub enum #enum_name {
//                     #( #field_names ),*
//                 }
//             };

//             // Convert generated enum into a TokenStream and return it
//             TokenStream::from(enum_tokens)
//         }
//         _ => panic!("FieldEnum macro only works with structs with named fields"),
//     }
// }

// #[proc_macro_derive(FieldOffset)]
// pub fn field_offset(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let struct_name = input.ident;
//     let field_names: Punctuated<Field, Comma>;

//     let fields = if let syn::Data::Struct(data_struct) = input.data {
//         match data_struct.fields {
//             Fields::Named(fields) => {
//                 field_names = fields.named.clone();
//                 field_names.iter().map(|f| {
//                     let field_name = &f.ident;
//                     let ty = &f.ty;
//                     return quote! {
//                         if field == stringify!(#field_name) {
//                             return Some(std::mem::size_of::<#ty>());
//                         }
//                     };
//                 })
//             }
//             _ => unimplemented!("FieldOffset only supports named fields"),
//         }
//     } else {
//         unimplemented!("FieldOffset only supports structs");
//     };

//     let expanded = quote! {
//         impl #struct_name {
//             pub fn get_field_offset(field: &str) -> Option<usize> {
//                 #(#fields)*
//                 None
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }

// macro_rules! generate_offsets {
//     ($struct_name:ident, $($field_name:ident),+) => {
//         // Generate an enum with variants for each field name
//         pub enum FieldOffset {
//             $( $field_name, )*
//         }

//         // Generate a function that takes the enum and returns the offset of the field
//         pub fn field_offset(field: FieldOffset) -> usize {
//             match field {
//                 $( FieldOffset::$field_name => offset_of!($struct_name, $field_name), )*
//             }
//         }
//     };
// }

#[proc_macro]
pub fn generate_offsets(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Assuming struct parsing, you'll need to extract field names here
    let struct_name = input.ident;
    if let syn::Data::Struct(s) = input.data {
        let fields = s.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

        // Generate the enum and impl
        let enum_name = quote! { #struct_name Field };
        let enum_variants = fields.iter().map(|f| {
            let name = f.as_ref().unwrap(); // assuming named fields, handle Option<Ident> appropriately
            quote! { #name }
        });
        let match_arms = fields.iter().map(|f| {
            let name = f.as_ref().unwrap(); // handle Option<Ident>
            quote! { #enum_name::#name => memoffset::offset_of!(#struct_name, #name), }
        });

        let expanded = quote! {
            pub enum #enum_name {
                #(#enum_variants),*
            }

            impl #struct_name {
                pub fn field_offset(field: #enum_name) -> usize {
                    match field {
                        #(#match_arms)*
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("generate_offsets can only be used with structs");
    }
}
