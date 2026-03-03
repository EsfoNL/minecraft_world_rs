use std::fs::File;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Ident, spanned::Spanned};

#[proc_macro_attribute]
pub fn enum_convert(
    tt_args: proc_macro::TokenStream,
    tt: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    enum_convert_impl(tt_args.into(), tt.into()).into()
}

struct Args {}
fn enum_convert_impl(tt_args: TokenStream, tt: TokenStream) -> TokenStream {
    let mut out = File::create("/tmp/proc-out").unwrap();
    writeln!(out, "{tt_args}, {tt}");
    let tt = syn::parse2::<DeriveInput>(tt).unwrap();

    use std::io::Write;

    let id = Ident::new(&format!("{}Type", tt.ident), tt.span());
    let mem = if let Data::Enum(ref e) = tt.data {
        e
    } else {
        panic!("not an enum")
    }
    .variants
    .iter()
    .map(|e| e.ident.clone());

    quote! {
        #tt
        #[doc("marker")]
        pub enum #id {
            #(#mem),*
        }
    }
}
