// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
#![warn(missing_docs, unreachable_pub)]

//! This crate contains the `SilentDebug` and `SilentDisplay` derive macros.
//! which help to avoid accidentally printing sensitive data.
//! Imported from diem-crypto-derive@0.0.3
//! https://github.com/diem/diem/blob/release-1.4.3/crypto/crypto-derive/src/lib.rs#L113

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Derive the `SilentDisplay` trait, which is an implementation of `Display` that does not print the contents of the struct.
/// This is useful for structs that contain sensitive data, such as private keys.
#[proc_macro_derive(SilentDisplay)]
pub fn silent_display(source: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(source).expect("Incorrect macro input");
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {
        // In order to ensure that secrets are never leaked, Display is elided
        impl #impl_generics ::std::fmt::Display for #name #type_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "<elided secret for {}>", stringify!(#name))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SilentDebug)]
/// Derive the `SilentDebug` trait, which is an implementation of `Debug` that does not print the contents of the struct.
/// This is useful for structs that contain sensitive data, such as private keys.
pub fn silent_debug(source: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(source).expect("Incorrect macro input");
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {
        // In order to ensure that secrets are never leaked, Debug is elided
        impl #impl_generics ::std::fmt::Debug for #name #type_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "<elided secret for {}>", stringify!(#name))
            }
        }
    };
    gen.into()
}

/// Overload group operations for a struct implementing [AdditiveGroupElement].
#[proc_macro_derive(GroupOps)]
pub fn group_ops(source: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(source).expect("Incorrect macro input");
    let name = &ast.ident;
    let gen = quote! {
        impl_op_ex!(+ |a: &#name, b: &#name| -> #name { <#name as AdditiveGroupElement>::Group::add(a, b) });
        impl_op_ex!(+= |a: &mut #name, b: &#name| { *a = <#name as AdditiveGroupElement>::Group::add(a, b) });
        impl_op_ex!(-= |a: &mut #name, b: &#name| { *a = <#name as AdditiveGroupElement>::Group::sub(a, b) });
        impl_op_ex!(*= |a: &mut #name, b: &<<#name as AdditiveGroupElement>::Group as AdditiveGroup>::Scalar| { *a = <#name as AdditiveGroupElement>::Group::mul(b, a) });
        impl_op_ex!(- |a: &#name, b: &#name| -> #name { <#name as AdditiveGroupElement>::Group::sub(a, b) });
        impl_op_ex_commutative!(* |a: &<<#name as AdditiveGroupElement>::Group as AdditiveGroup>::Scalar, b: &#name| -> #name { <#name as AdditiveGroupElement>::Group::mul(a, b) });
        impl_op_ex!(- |a: &#name| -> #name { <#name as AdditiveGroupElement>::Group::neg(a) });
        impl_op_ex_commutative!(* |a: u64, b: &#name| -> #name { <#name as AdditiveGroupElement>::Group::mul(&<<#name as AdditiveGroupElement>::Group as AdditiveGroup>::Scalar::from(a), b) });
        impl_op_ex!(*= |a: &mut #name, b: u64| { *a = <#name as AdditiveGroupElement>::Group::mul(&<<#name as AdditiveGroupElement>::Group as AdditiveGroup>::Scalar::from(b), a) });
    };
    gen.into()
}
