use proc_macro2::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident};


#[proc_macro_derive(NewTable)]
pub fn impl_new_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;

    let new = fn_new(&ast.data);
    let new_popularity = fn_new_by_popularity(&ast.data);

    let expanded = quote!{
        impl #name {
            pub fn new() -> Option<Self> {
                #new
            }
            pub fn new_by_popularity() -> Option<Self> {
                #new_popularity
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn fn_new(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    let container = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote!{
                            use rayon::prelude::*;
                            let nbr_page = reqwest::blocking::get(concat!("https://nhentai.net/", stringify!(#name), "/"))
                                .ok()?
                                .text()
                                .ok()?
                                .after(r#"<section class="pagination">"#)
                                .before(r#"</section>"#)
                                .between(r#"class="next""#, r#"" class="last""#)
                                .after("page=")?
                                .parse::<u32>()
                                .ok()?;

                            let vec_html = (1..nbr_page + 1)
                                .flat_map(|x| {
                                    let url: String = format!("https://nhentai.net/{}/?page={}", stringify!(#name), x);
                                    Some(reqwest::blocking::get(&url).ok()?.text().ok()?)
                                })
                                .collect::<Vec<String>>();
                
                            Some(Self {
                                #name: vec_html
                                    .par_iter()
                                    .flat_map(|x| {
                                        if let Some(t) = Self::search_tag(x) {
                                            Some(t)
                                        } else {
                                            println!(stringify!(Error while searching #name page {}), x);
                                            None
                                        }
                                    })
                                    .flatten()
                                    .collect::<Vec<Tag>>()
                            })
                        }
                    });
                    quote!{
                        #(#container)*
                    }
                }
                _ => unimplemented!()
            }
        }
        _ => unimplemented!()
    }
}

fn fn_new_by_popularity(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    let container = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote!{
                            use rayon::prelude::*;
                            let nbr_page = reqwest::blocking::get(concat!("https://nhentai.net/", stringify!(#name), "/popular"))
                                .ok()?
                                .text()
                                .ok()?
                                .after(r#"<section class="pagination">"#)
                                .before(r#"</section>"#)
                                .between(r#"class="next""#, r#"" class="last""#)
                                .after("page=")?
                                .parse::<u32>()
                                .ok()?;

                            let vec_html = (1..nbr_page + 1)
                                .flat_map(|x| {
                                    let url: String = format!("https://nhentai.net/{}/popular?page={}", stringify!(#name), x);
                                    Some(reqwest::blocking::get(&url).ok()?.text().ok()?)
                                })
                                .collect::<Vec<String>>();
                
                            Some(Self {
                                #name: vec_html
                                    .par_iter()
                                    .flat_map(|x| {
                                        if let Some(t) = Self::search_tag(x) {
                                            Some(t)
                                        } else {
                                            println!(stringify!(Error while searching #name page {}), x);
                                            None
                                        }
                                    })
                                    .flatten()
                                    .collect::<Vec<Tag>>()
                            })

                        }
                    });
                    quote!{
                        #(#container)*
                    }
                }
                _ => unimplemented!()
            }
        }
        _ => unimplemented!()
    }
}
