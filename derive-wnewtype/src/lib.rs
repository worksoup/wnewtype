//! `wnewtype` 实现。 你不应直接使用此 crate.

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{Data, Field};

use quote::quote;
use zdcz::{fill_default_fields, type_is_phantom};

/// 为结构体实现 `newtype` 模式。
///
/// 这将为内含值实现 `From`, `Into`, `Deref` 和 `DerefMut` 特型。
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
        _ => panic!("NewType can only be derived for structs"),
    };
    let (len, th, field_name) =
        zdcz::find_needed_field_index(&st.fields, |field: &Field| !type_is_phantom(field));
    if len != 1 {
        panic!("NewType can only be derived for single-valued-field structs")
    }
    let th = th.to_string().parse::<proc_macro2::TokenStream>().unwrap();
    let field = st.fields.iter().next().unwrap();
    let field_ty = &field.ty;
    let (from, init) = fill_default_fields(
        &st.fields,
        |f| !type_is_phantom(f),
        &"other".parse().unwrap(),
    );
    let from = quote! {
        #(#init)*
        #name
        #from
    };
    let (deref, deref_mut, into_inner) = if let Some(field_name) = field_name {
        let deref = quote! {
            &self.#field_name
        };
        let deref_mut = quote! {
            &mut self.#field_name
        };
        let into_inner = quote! {
            self.#field_name
        };
        (deref, deref_mut, into_inner)
    } else {
        let deref = quote! {
            &self.
            #th
        };
        let deref_mut = quote! {
            &mut self.
            #th
        };
        let into_inner = quote! {
            self.
            #th
        };
        (deref, deref_mut, into_inner)
    };

    let from = quote! {
        impl #impl_generics From<#field_ty> for #name #ty_generics #where_clause {
            #[inline]
            fn from(other: #field_ty) -> #name #ty_generics {
                #from
            }
        }
    };

    let deref = quote! {
        impl #impl_generics ::core::ops::Deref for #name #ty_generics #where_clause {
            type Target = #field_ty;
            #[inline]
            fn deref(&self) -> &Self::Target {
                #deref
            }
        }
    };

    let deref_mut = quote! {
        impl #impl_generics ::core::ops::DerefMut for #name #ty_generics #where_clause {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                #deref_mut
            }
        }
    };

    let into_inner = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// Unwrap to the inner type
            #[inline]
            pub fn into_inner(self) -> #field_ty {
                #into_inner
            }
        }
    };

    quote! {
        #from #deref #deref_mut #into_inner
    }
}
