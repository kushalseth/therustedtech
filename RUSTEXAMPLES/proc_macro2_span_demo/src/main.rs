extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::Span;

#[proc_macro]
pub fn show_source_file(_input: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let source_file = span.source_file();
    let path = source_file.path().to_string_lossy();

    let output = quote! {
        println!("Macro expanded from file: {}", #path);
    };

    output.into()
}
