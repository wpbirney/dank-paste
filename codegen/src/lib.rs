#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;
use syn::Meta;

use syn::Lit;

#[proc_macro_derive(DankInfo)]
pub fn dank_info(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;

    let expanded = quote! {
        impl DankInfo for #name {
            fn load(path: &str) -> #name {
                let f = File::open(path).unwrap();
                serde_json::from_reader(f).unwrap()
            }

            fn write_to_file(&self, path: &str)   {
                let f = File::create(path).unwrap();
                serde_json::to_writer(f, &self).unwrap();
            }

            fn expire(&self) -> u64 {
                self.expire
            }
        }
    };

    expanded.into()
}

fn impl_dank_id(ast: syn::DeriveInput) -> quote::Tokens {
    let name = ast.ident;

    let meta = match ast.attrs[0].interpret_meta().unwrap() {
        Meta::NameValue(r) => r,
        _ => panic!("")
    };

    let root = match meta.lit {
        Lit::Str(ref s) => s,
        _ => panic!("")
    };

    let root = root.value();

    println!("{}: {}", meta.ident, root);

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

    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let expanded = impl_dank_id(ast);

    expanded.into()
}
