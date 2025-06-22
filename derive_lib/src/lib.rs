extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let mut struct_name = None;
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Ident(ident) => {
                if ident.to_string() == "struct" {
                    if let Some(TokenTree::Ident(name)) = tokens.next() {
                        struct_name = Some(name.to_string());
                        break;
                    }
                }
            }
            _ => continue,
        }
    }
    let name = match struct_name {
        Some(name) => name,
        None => {
            return "compile_error!(\"Expected a struct definition\")"
                .parse()
                .unwrap()
        }
    };
    let output = format!("impl Component for {name} {{}}");
    output.parse().unwrap()
}
