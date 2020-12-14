extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Block)]
pub fn block_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_block_macro(&ast)
}

fn impl_block_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for i in 1..self.bytes.len() + 1 {
                    let _ = write!(
                        f,
                        "{:02x}{}",
                        self.bytes[i - 1],
                        if i % 4 == 0 { '\n' } else { ' ' }
                    );
                }
                Ok(())
            }
        }

        impl From<[u8; 16]> for #name {
            fn from(bytes: [u8; 16]) -> Self {
                Self { bytes }
            }
        }
    };
    gen.into()
}
