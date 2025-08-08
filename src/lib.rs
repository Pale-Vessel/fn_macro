//! Macro to derive the three `Fn` traits
//!
//! Provides a way to implement `FnOnce`, `FnMut`, and `Fn` with the same
//! arguments, output type, and body.
//! 
//! ## Macro for function traits
//! This crate adds a macro to derive the three function traits `FnOnce`, `FnMut`, and `Fn`. These traits are often implemented three times with the same signature and body (excluding the different borrow type on `self`), which can lead to unnecessary boilerplate - this macro hopes to overcome this.
//! 
//! ## Usage
//! This macro is comprised of four parts - the initial `derive`, and three attributes for the input and output types, and the function body.
//! 
//! ```toml
//! [dependencies]
//! fn_macro = "0.1.0"
//! ```
//! 
//! ```rust
//! #![feature(unboxed_closures)]
//! #![feature(fn_traits)]
//! 
//! use fn_macro::{Fn, fn_args, fn_body, fn_output};
//! 
//! #[derive(Fn)]
//! #[fn_args(f64, f64, String)]
//! #[fn_body{
//!     let k = self.0 + args.0;
//!     format!("{} {}", args.2, k + args.1)
//! }]
//! #[fn_output(String)]
//! struct Test(f64);
//! 
//! fn main() {
//!     let object = Test(9.5);
//!     println!("{}", object(1.0, 2.5, String::from("Hello"))) //Hello 13.0
//! }
//! ```
//! 
//! ## Known issues
//! Due to the use of `expect` in the macro code, VSCode will highlight the macro's use as incorrect code, claiming it will always panic. This is wrong - the macro will only panic if one of the necessary fields is not provided.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, MetaList, parse};

/// The main macro, to begin the derivation process
/// 
/// See the top level documentation for more detail
#[proc_macro_derive(Fn)]
pub fn derive_fn_mut(input: TokenStream) -> TokenStream {
    let ast = parse(input).unwrap();
    impl_fn(&ast)
}

fn find_attr(attributes: &[Attribute], wanted_attribute: &str) -> Attribute {
    attributes
        .iter()
        .find(|attr| attr.path().is_ident(wanted_attribute))
        .expect("No attribute of type {wanted_attribute} given")
        .clone()
}

fn impl_fn(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let args_attr = find_attr(&ast.attrs, "fn_args");
    let MetaList {
        tokens: arg_tokens, ..
    } = args_attr.meta.require_list().unwrap();
    let arg_size = arg_tokens.clone().into_iter().count().div_ceil(2);
    let args: proc_macro2::TokenStream = (0..arg_size)
        .map(|index| format!("args.{index}"))
        .collect::<Vec<_>>()
        .join(",")
        .parse()
        .unwrap();

    let body_attr = find_attr(&ast.attrs, "fn_body");
    let MetaList {
        tokens: body_tokens,
        ..
    } = body_attr.meta.require_list().unwrap();

    let output_attr = find_attr(&ast.attrs, "fn_output");
    let MetaList {
        tokens: output_tokens,
        ..
    } = output_attr.meta.require_list().unwrap();

    let generated = quote! {
        impl FnOnce<(#arg_tokens)> for #name {
            type Output = #output_tokens;
            extern "rust-call" fn call_once(self, args: (#arg_tokens)) -> Self::Output {
                self(#args)
            }
        }
        impl FnMut<(#arg_tokens)> for #name {
            extern "rust-call" fn call_mut(&mut self, args: (#arg_tokens)) -> Self::Output {
                self(#args)
            }
        }
        impl Fn<(#arg_tokens)> for #name {
            extern "rust-call" fn call(&self, args: (#arg_tokens)) -> Self::Output {
                #body_tokens
            }
        }

    };
    generated.into()
}

/// The macro to hold the input arguments to the function
///  
/// See the top-level documentation for more detail
#[proc_macro_attribute]
pub fn fn_args(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// The macro to hold the body of the function
/// 
/// See the top-level documentation for more detail
#[proc_macro_attribute]
pub fn fn_body(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// The macro to hold the output type of the function
/// 
/// See the top-level documentation for more detail
#[proc_macro_attribute]
pub fn fn_output(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}
