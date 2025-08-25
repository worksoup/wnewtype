//! ZDCZ(字段操作)。

use quote::quote;
use syn::{Attribute, Field, Fields};
/// 获取需要的属性。
///
/// - `iter`: 字段拥有的属性。一般为 `field.attrs.iter()`.
/// - `needed`: 所需的属性。
pub fn get_field_attr<'a>(
    iter: impl Iterator<Item = &'a Attribute>,
    needed: &str,
) -> Option<Attribute> {
    let mut b = None;
    for a in iter {
        if let Some(ident) = a.path().get_ident() {
            if ident == needed {
                b = Some(a.clone());
                break;
            }
        }
    }
    b
}
/// 填充字段。
pub fn fill_default_fields(
    fields: &Fields,
    (th, inner): (usize, &Field),
    value_name: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    for (th_, field) in fields.iter().enumerate() {
        if if let Some(inner_ident) = inner.ident.as_ref() {
            if let Some(this_field_ident) = field.ident.as_ref() {
                inner_ident == this_field_ident
            } else {
                false
            }
        } else {
            th_ == th
        } {
            if let Some(field_name) = inner.ident.as_ref() {
                tokens.extend(quote!(#field_name: #value_name,));
            } else {
                tokens.extend(quote!(#value_name,));
            }
        } else if let Some(field_name) = field.ident.as_ref() {
            tokens.extend(quote!(#field_name: ::core::default::Default::default(),))
        } else {
            tokens.extend(quote!(::core::default::Default::default(),));
        }
    }

    if inner.ident.is_some() {
        quote! {
            {#tokens}
        }
    } else {
        quote! {
            (#tokens)
        }
    }
}
pub fn find_inner_type_field(fields: &Fields) -> syn::Result<(usize, &syn::Field)> {
    let mut len = 0;
    let mut th = 0;
    let mut name = None;
    let mut iter = fields.iter();
    let first = iter
        .next()
        .ok_or_else(|| syn::Error::new_spanned(fields, "该类型没有字段，无法视为 NewType."))?;
    let is_inner_type = get_field_attr(first.attrs.iter(), "inner").is_some();
    if is_inner_type {
        len += 1;
        th = 0;
        name = Some(first)
    }

    let mut field_count = 1;
    for (th_, field) in iter.enumerate() {
        field_count = th_;
        let is_inner_type = get_field_attr(field.attrs.iter(), "inner").is_some();
        if is_inner_type {
            len += 1;
            th = th_;
            name = Some(field);
        }
    }
    if len != 1 {
        if len == 0 && field_count == 1 {
            Ok((0, first))
        } else {
            Err(syn::Error::new_spanned(
                first,
                "NewType can only be derived for single-valued-field structs, consider add `#[inner]` to the inner type field.",
            ))
        }
    } else {
        Ok((th, name.unwrap()))
    }
}
