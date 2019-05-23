//! Proc-macro implementation for `newtype`. You probably shouldn't use this
//! crate directly.

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{
    Data,
    Fields,
};

use quote::quote;

/// Treat a single-field tuple struct as a "newtype"
///
/// This will implement `From`, `Into`, `Deref`, and `DerefMut` for the inner
/// type.
#[proc_macro_derive(NewType)]
pub fn newtype(input: TokenStream) -> TokenStream {
    let input = syn::parse::<syn::DeriveInput>(input).expect("syn parse derive input");

    gen_impl(input).into()
}

fn gen_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident;

    let st = match input.data {
        Data::Struct(st) => st,
        _ => panic!("NewType can only be derived for single-field tuple structs"),
    };

    let fields = match st.fields {
        Fields::Unnamed(fields) => fields,
        _ => panic!("NewType can only be derived for single-field tuple structs"),
    };

    if fields.unnamed.len() != 1 {
        panic!("NewType can only be derived for single-field tuple structs")
    }

    let field_ty = fields.unnamed.into_iter().nth(0).unwrap().ty;

    let from = quote! {
        impl #impl_generics From<#field_ty> for #name #ty_generics #where_clause {
            fn from(other: #field_ty) -> #name #ty_generics {
                #name (other)
            }
        }
    };

    let other_from = quote! {
        impl #impl_generics From<#name #ty_generics> for #field_ty #where_clause {
            fn from(other: #name #ty_generics) -> #field_ty {
                other.0
            }
        }
    };

    let deref = quote! {
        impl #impl_generics ::core::ops::Deref for #name #ty_generics #where_clause {
            type Target = #field_ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    let deref_mut = quote! {
        impl #impl_generics ::core::ops::DerefMut for #name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    let into_inner = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn into_inner(self) -> #field_ty {
                self.0
            }
        }
    };

    quote! {
        #from #other_from #deref #deref_mut #into_inner
    }
}
