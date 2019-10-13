extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ReadToken)]
pub fn read_token(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = if ast.generics.lifetimes.is_empty() {
        impl_read_token(&ast)
    } else {
        impl_read_token_lifetime(&ast)
    };

    gen.parse().unwrap()
}

fn impl_read_token_lifetime(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl<'t> ReadToken<'t> for #name<'t> {
            type Token = #name<'t>;

            fn read_token_caps(&self, _: &str, _: &[&str]) -> Self::Token {
                self.clone()
            }
        }
    }
}

fn impl_read_token(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ReadToken<'static> for #name {
            type Token = #name;

            fn read_token_caps(&self, _: &str, _: &[&str]) -> Self::Token {
                self.clone()
            }
        }
    }
}
