extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;
use syn::Data::*;
use syn::Fields::*;

const NAMES: [&str; 10] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

#[proc_macro_derive(DefaultConstructor)]
pub fn default_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let constructor_impl = match &input.data {
        Struct(DataStruct { fields: content, .. }) => {
            match content {
                Named(FieldsNamed { named, .. }) => {
                    let parameters = named.iter().map(|field| {
                        let field_ident = &field.ident;
                        let field_type = &field.ty;
                        quote!(#field_ident: #field_type)
                    });

                    let fields = named.iter().map(|field| {
                        let field_ident = &field.ident;
                        quote!(#field_ident: #field_ident)
                    });

                    quote!{
                        impl #ident {
                            pub fn new(#(#parameters),*) -> Self {
                                Self {
                                    #(#fields),*
                                }
                            }
                        }
                    }
                },
                Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let parameters = unnamed.iter().enumerate().map(|(i, field)| {
                        let field_ident = Ident::new(NAMES[i], Span::call_site());
                        let field_type = &field.ty;
                        quote!(#field_ident: #field_type)
                    });

                    let fields = unnamed.iter().enumerate().map(|(i, _field)| {
                        let field_ident = Ident::new(NAMES[i], Span::call_site());
                        quote!(#field_ident)
                    });

                    quote!{
                        impl #ident {
                            pub fn new(#(#parameters),*) -> Self {
                                Self(#(#fields),*)
                            }
                        }
                    }
                },
                Unit => {
                    quote!{
                        impl #ident {
                            pub fn new() -> Self {
                                Self{}
                            }
                        }
                    }
                },
            }
        },
        _ => panic!("A default constructor can only be created for structs")
    };

    TokenStream::from(constructor_impl)
}
