//! `wnewtype` 实现。 你不应直接使用此 crate.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

mod zdcz;

/// 为结构体实现 `newtype` 模式。
///
/// 这将为内含值实现 `From`, `Into`, `Deref` 和 `DerefMut` 特型。
#[proc_macro_derive(NewType, attributes(inner))]
pub fn newtype(input: TokenStream) -> TokenStream {
    let input = syn::parse::<syn::DeriveInput>(input).expect("syn parse derive input");

    match gen_impl(input) {
        Ok(out) => out,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

fn gen_impl(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident;

    let st = match input.data {
        Data::Struct(st) => st,
        Data::Enum(data_enum) => Err(syn::Error::new_spanned(
            data_enum.enum_token,
            "NewType can only be derived for structs",
        ))?,
        Data::Union(data_union) => Err(syn::Error::new_spanned(
            data_union.union_token,
            "NewType can only be derived for structs",
        ))?,
    };
    let (th, needed_field) = zdcz::find_inner_type_field(&st.fields)?;
    let th_token = th.to_string().parse::<proc_macro2::TokenStream>().unwrap();
    let field_ty = &needed_field.ty;
    let from = zdcz::fill_default_fields(&st.fields, (th, needed_field), &"other".parse().unwrap());
    let from = quote! {
        #name
        #from
    };
    let (deref, deref_mut, into_inner) = if let Some(field_name) = &needed_field.ident {
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
            #th_token
        };
        let deref_mut = quote! {
            &mut self.
            #th_token
        };
        let into_inner = quote! {
            self.
            #th_token
        };
        (deref, deref_mut, into_inner)
    };

    let from = quote! {
        impl #impl_generics ::core::convert::From<#field_ty> for #name #ty_generics #where_clause {
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

    Ok(quote! {
        #from #deref #deref_mut #into_inner
    })
}
