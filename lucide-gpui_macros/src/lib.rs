use std::collections::HashMap;
use include_dir::{Dir, include_dir};
use quote::quote;

const ICONS: Dir = include_dir!("icons");

fn get_icons() -> HashMap<String, String> {
    let mut icons: HashMap<String, String> = HashMap::new();
    for entry in ICONS.entries() {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_str().unwrap();
        icons.insert(name.to_string(), name.to_string());
    }
    icons
}

fn create_function_name(s: &str) -> String {
    let snake_case = s.replace("-", "_");

    return "icon_".to_string() + &snake_case;
}

#[proc_macro]
pub fn lucide_init(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let param = syn::parse_macro_input!(input as syn::Ident);

    let icons = get_icons();

    let mut expanded = proc_macro2::TokenStream::new();
    for (name, path) in icons {
        let name = create_function_name(&name);
        let quoted = format!("lucide:{}", path);

        let name_as_ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
        expanded.extend(quote! {
            pub fn #name_as_ident() -> Self {
                Self {
                    source: #quoted,
                    size: IconSize::Md,
                    color: None,
                }
            }
        });
    }

    let impl_block = quote! {
        impl #param {
            #expanded
        }
    };

    impl_block.into()
}

/// Match statement to go from path to data
#[proc_macro]
pub fn match_icons(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // take a variable as input
    let input = syn::parse_macro_input!(input as syn::Ident);

    let icons = get_icons();
    let mut arms = proc_macro2::TokenStream::new();
    for (_, path) in icons {
        let bytes = ICONS.get_file(format!("{}.svg", path)).unwrap().contents();
        let bytes = bytes.iter().map(|b| *b).collect::<Vec<u8>>();
        let bytes = syn::LitByteStr::new(&bytes, proc_macro2::Span::call_site());
        arms.extend(quote! {
            #path => Ok(Cow::Borrowed(#bytes)),
        });
    }

    let expanded = quote! {
        match #input {
            #arms
            _ => Err(anyhow::anyhow!("Asset not found: {}", #input)),
        }
    };

    expanded.into()
}