use proc_macro2::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields};


#[proc_macro_derive(NewTable)]
pub fn impl_new_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;

    let new = fn_new(&ast.data);
    let new_popularity = fn_new_by_popularity(&ast.data);
    let search_tag = fn_search_tag(&ast.data);

    let expanded = quote!{
        impl #name {
            pub fn new() -> Option<Self> {
                #new
            }
            pub fn new_by_popularity() -> Option<Self> {
                #new_popularity
            }

            fn search_tag(html: &str) -> Option<Vec<Tag>> {
                #search_tag
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
                    let name = &fields.named[0].ident;
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
                                Some(reqwest::blocking::get(&format!("https://nhentai.net/{}/?page={}", stringify!(#name), x)).ok()?.text().ok()?)
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
                    let name = &fields.named[0].ident;
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
                                Some(reqwest::blocking::get(&format!("https://nhentai.net/{}/popular?page={}", stringify!(#name), x)).ok()?.text().ok()?)
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
                }
                _ => unimplemented!()
            }
        }
        _ => unimplemented!()
    }
}

fn fn_search_tag(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    let name = &fields.named[0].ident;
                    quote!{
                        Some(
                            html
                            .after(r#"<div class="container" id="tag-container">"#)
                            .before(r#"</div>"#)?
                            .split(r#"<section"#)
                            .map(|x| x.split(r#"<a href="#).collect::<Vec<&str>>())
                            .flatten()
                            .collect::<Vec<&str>>()
                                    .into_iter()
                                    .flat_map(|x| {
                                        let mut name_string = stringify!(#name).to_owned();
                                        name_string.pop();
                                        Some(Tag {
                                            id: x[x.find("tag tag-")? + 8..x.find(r#" "><span"#)?]
                                                .parse::<u32>()
                                                .ok()?,
                                            _type: name_string,
                                            name: x[x.find(r#""name">"#)? + 7..x.find("</span><span")?].to_owned(),
                                            url: x[x.find("\"")? + 1..x.find(r#"" class="tag"#)?].to_owned(),
                                            count: x[x.find(r#""count">"#)? + 8..x.find("</span></a>")?]
                                                .replace("K", "000")
                                                .parse::<u32>()
                                                .ok()?,
                                        })
                                    })
                                    .collect::<Vec<Tag>>(),
                        )
                    }
                }
                _ => unimplemented!()
            }
        }
        _ => unimplemented!()
    }
}


#[proc_macro_derive(Table)]
pub fn impl_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;

    let impl_table_content = fn_impl_table_content(&ast.data);

    let expanded = quote!{
        impl Table for #name {
            #impl_table_content
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn fn_impl_table_content(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    let name = &fields.named[0].ident;
                    quote!{
                        fn get(&self, index: usize) -> Option<&Tag> {
                            self.#name.get(index)
                        }
                    
                        fn get_mut(&mut self, index: usize) -> Option<&mut Tag> {
                            self.#name.get_mut(index)
                        }
                        
                        fn get_by_id(&self, id: u32) -> Option<&Tag> {
                            self.#name.iter().find(|&t| t.id == id)
                        }
                    
                        fn get_by_name(&self, name: &str) -> Option<&Tag> {
                            self.#name.iter().find(|&t| t.name == name)
                        }
                    
                        fn max(&self) -> Option<&Tag> {
                            self.#name.iter().max_by_key(|t| t.count)
                        }
                    
                        fn min(&self) -> Option<&Tag> {
                            self.#name.iter().min_by_key(|t| t.count)
                        }
                        fn sort_by_popularity(&mut self) {
                            self.#name.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
                        }
                    
                        fn sort_by_alphabetical(&mut self) {
                            self.#name.sort_unstable_by_key(|x| x.name.to_owned())
                        }
                    }
                }
                _ => unimplemented!()
            }
        }
        _ => unimplemented!()
    }
}