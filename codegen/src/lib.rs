extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn impl_load_write(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LoadWrite for #name {
            fn load(path: &str) -> #name {
                let f = File::open(path).unwrap();
                serde_json::from_reader(f).unwrap()
            }

            fn write_to_file(&self, path: &str)   {
                let f = File::create(path).unwrap();
                serde_json::to_writer(f, &self).unwrap();
            }
        }
    }
}

#[proc_macro_derive(LoadWrite)]
pub fn load_write(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_load_write(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
