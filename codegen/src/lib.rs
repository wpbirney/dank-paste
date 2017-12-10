#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::MetaItem;
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

fn impl_dank_id(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    if ast.attrs[0].name() != "Path" {
        panic!("Path attribute needed");
    }

    let root = match ast.attrs[0].value {
        MetaItem::NameValue(_,ref r) => r,
        MetaItem::List(_,_) => panic!(""),
        MetaItem::Word(_) => panic!("")
    };

    quote! {
        impl DankId for #name {
            fn generate() -> #name {
                #name { id: generate_unused(#root) }
            }

            fn from_id(id: &str) -> Option<#name>    {
                match check_for_id(#root, id) {
                    true => Some(#name { id: id.to_string() }),
                    false => None
                }
            }
            fn id(&self) -> String { self.id.clone() }
            fn filename(&self) -> String { format!("{}/{}", #root, &self.id) }
            fn json(&self) -> String { format!("{}/{}.json", #root, &self.id) }
            fn del(&self) -> String { format!("{}/{}.del", #root, &self.id) }
            fn delete_all(&self)    {
                fs::remove_file(self.filename()).unwrap_or(());
                fs::remove_file(self.json()).unwrap_or(());
                fs::remove_file(self.del()).unwrap_or(());
            }
        }
    }
}

#[proc_macro_derive(DankId, attributes(Path))]
pub fn dank_id(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_dank_id(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
