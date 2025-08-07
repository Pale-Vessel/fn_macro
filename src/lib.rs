use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, MetaList, parse};

#[proc_macro_derive(Fn, attributes(option))]
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

#[proc_macro_attribute]
pub fn fn_args(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn fn_body(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn fn_output(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}
