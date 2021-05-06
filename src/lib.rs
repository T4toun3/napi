
mod string_utils;
pub mod search;
pub mod doujin;
pub mod table;
pub mod tag;
pub mod search_args;
pub mod serde_utils;

#[cfg(test)]
mod tests {


    use std::time::SystemTime;

    use crate::search::Search;
    use crate::search_args::{SearchArgs, Sort};
    use crate::table::*;
    use crate::{
        doujin::Doujin,
        tag::{Tag, TagType},
    };

    #[test]
    fn new_table_json() {
        let mut table1 = LanguageTable::new().unwrap();
        let table2 = LanguageTable::new_by_popularity().unwrap();
        assert_ne!(table1, table2);
        table1.sort_by_popularity();
        assert_eq!(table1, table2);
    }

    #[test]
    fn new_table_html() {
        let table1 = TagTable::new().unwrap();
        let table2 = TagTable::new_by_popularity().unwrap();
        assert_ne!(table1, table2);
        assert_eq!(table1.len(), table2.len());
    }

    #[test]
    fn table_trait_some() {
        let mut table = CategoryTable::new().unwrap();
        assert_eq!(table.len(), 7);
        assert_eq!(table.get(0).unwrap().name, "artistcg");
        assert_eq!(table.get_mut(2).unwrap()._type, TagType::Category);
        assert_eq!(table.get_by_name("manga").unwrap().url, "/category/manga/");
        assert_eq!(table.max().unwrap().name, "doujinshi");
        assert_eq!(table.min().unwrap().id, 36320);
    }

    #[test]
    fn table_trait_none() {
        let mut table = CategoryTable::new().unwrap();
        assert!(table.get(8).is_none());
        assert!(table.get_mut(32).is_none());
        assert!(table.get_by_name("no name").is_none());
    }

    #[test]
    #[ignore = "Take too much time"]
    fn all_table() {
        TagTable::new().unwrap();
        ArtistTable::new().unwrap();
        CharacterTable::new().unwrap();
        ParodieTable::new().unwrap();
        GroupTable::new().unwrap();
    }

    #[test]
    fn args_corretor_default() {
        let args = vec![];
        assert_eq!(
            SearchArgs::correct(args),
            vec![
                SearchArgs::Page(1),
                SearchArgs::Sort(Sort::Recent),
                SearchArgs::Text("\n\n".to_string(), true)
            ]
        );
    }

    #[test]
    fn args_corretor_concat_text() {
        let args = vec![
            SearchArgs::Text("loli".to_string(), true),
            SearchArgs::Text("con".to_string(), true),
        ];
        assert_eq!(
            SearchArgs::correct(args),
            vec![
                SearchArgs::Page(1),
                SearchArgs::Sort(Sort::Recent),
                SearchArgs::Text("loli+con".to_string(), true)
            ]
        );
    }

    #[test]
    fn args_corretor_concat_text_minus() {
        let args = vec![
            SearchArgs::Text("loli".to_string(), true),
            SearchArgs::Text("con".to_string(), false),
        ];
        assert_eq!(
            SearchArgs::correct(args),
            vec![
                SearchArgs::Page(1),
                SearchArgs::Sort(Sort::Recent),
                SearchArgs::Text("loli+-con".to_string(), true)
            ]
        );
    }

    #[test]
    fn args_corretor_dedup() {
        let args = vec![
            SearchArgs::Page(5),
            SearchArgs::Page(9),
            SearchArgs::Sort(Sort::PopularToday),
            SearchArgs::Sort(Sort::PopularAllTime),
            SearchArgs::Sort(Sort::Recent),
        ];
        assert_eq!(
            SearchArgs::correct(args),
            vec![
                SearchArgs::Page(5),
                SearchArgs::Sort(Sort::PopularToday),
                SearchArgs::Text("\"\"".to_string(), true)
            ]
        );
    }

    #[test]
    fn args_corretor_tag_to_text() {
        let args = vec![SearchArgs::Tag(
            Tag {
                _type: TagType::Tag,
                name: "big breast".to_owned(),
                ..Default::default()
            },
            true,
        )];
        assert_eq!(
            SearchArgs::correct(args),
            vec![
                SearchArgs::Page(1),
                SearchArgs::Sort(Sort::Recent),
                SearchArgs::Text(r#"tag:"big breast""#.to_string(), true)
            ]
        );
    }

    #[test]
    fn args_corretor_concate_tag_and_text() {
        let args = vec![
            SearchArgs::Tag(
                Tag {
                    _type: TagType::Tag,
                    name: "big breast".to_owned(),
                    ..Default::default()
                },
                true,
            ),
            SearchArgs::Text("one".to_owned(), false),
            SearchArgs::Tag(
                Tag {
                    _type: TagType::Language,
                    name: "english".to_owned(),
                    ..Default::default()
                },
                true,
            ),
        ];
        assert_eq!(
            SearchArgs::correct(args),
            vec![
                SearchArgs::Page(1),
                SearchArgs::Sort(Sort::Recent),
                SearchArgs::Text(
                    r#"tag:"big breast"+-one+language:"english""#.to_string(),
                    true
                )
            ]
        );
    }

    #[test]
    fn search_and_merge() {
        // TODO: test when the max value is greater than the number of page 
        let args = vec![
            SearchArgs::Page(2),
            SearchArgs::Text("hello".to_string(), true),
        ];
        let mut search = Search::new(args).unwrap();
        search.merge_search_all_pages(); // On récupère toutes les autres pages dans la recherche et les ajoute dans entries en plus de la page précédente
        assert!(search.entries.len() > 70);
    }

    #[test]
    fn shearch_popular() {
        let mut search_pop = Search::search_populars(Sort::PopularAllTime).unwrap();
        search_pop.merge_search_pages(1..3);
        assert!(search_pop
            .entries
            .iter()
            .map(|e| e.id)
            .any(|id| id == 297974))
    }

    
    #[test]
    fn random_doujin() {
        let rand = Doujin::random();
        println!("Ramdom doujin : {:?}", rand);
    }

    #[test]
    fn doujin_from_search() {
        let args = vec![
            SearchArgs::Page(2),
            SearchArgs::Text("hello".to_string(), true),
        ];
        let search = Search::new(args).unwrap();
        search.entries[0].fetch().unwrap(); // On prend la première page de la recherche
        // println!("{:?}",Search::new("https://nhentai.net/search/?q=test&page=1&sort=popular-week"));
    }

    #[test]
    fn new_doujin() {
        let doujin = Doujin::new(224001).unwrap();
        let img_urls = doujin.get_images_urls();
        assert_eq!(img_urls.len(), 31);
        assert_eq!(img_urls[0], "https://i.nhentai.net/galleries/1183642/1.jpg");
    }

    #[test]
    fn doujin_media_id() {
        let doujin = Doujin::new(224001).unwrap();
        assert_eq!(doujin.media_id, 1183642);
    }

    #[test]
    fn similar() {
        let doujin = Doujin::new(316932).unwrap();
        assert_eq!(doujin.similars.len(), 5);
    }

    #[test]
    fn all_methods_doujin() {
        let doujin = Doujin::new(334108).unwrap();
        assert_eq!(doujin.get_image_url_small(3).unwrap(), "https://t.nhentai.net/galleries/1764861/3t.png");
        // TODO: complite with all methods
    }
    #[test]
    fn it_works() {
        let time_start = SystemTime::now();

        /*let args = vec![SearchArgs::Page(2), SearchArgs::Page(3), SearchArgs::Text("hello".to_string())];
        let url = Search::build_url_with_args(args);
        println!("url = {}", url);
        let mut search = Search::new(&url).unwrap();
        search.merge_search_all_pages(); // On récupère toutes les autres pages dans la recherche et les ajoute dans entries en plus de la page précédente
        println!("All results:");
        for result in &search.entries {
            println!("[{}] {}",result.id,result.name) // On affiche toutes les entrées trouvées
        }
        println!("{}", search.entries[0].id);
        let page = Doujin::new(search.entries[0].id).unwrap(); // On prend la première page de la recherche
        println!("{:?}",page.get_pages_image_urls()); // On récupère l'url de toutes les images du manga
        println!("{:?}",Search::new("https://nhentai.net/search/?q=test&page=1&sort=popular-week"));*/

        let args = vec![SearchArgs::Page(2)];
        let search = Search::new(args).unwrap();
        for result in search.entries {
            println!("[{}], {}", result.id, result.name)
        }

        /*let doujin = Doujin::new(327341).unwrap();
        println!("{:?}", doujin);
        println!("get_page_image_url_small : {}", doujin.get_page_image_url_small(1).unwrap());
        //println!("get_pages_image_urls_small : {:?}", doujin.get_pages_image_urls_small());
        println!("get_page_image_url : {}", doujin.get_page_image_url(1).unwrap());
        //println!("get_pages_image_urls : {:?}", doujin.get_pages_image_urls());
        println!("get_image_cover : {}", doujin.get_image_cover());
        println!("get_image_thumbnail : {}", doujin.get_image_thumbnail());
        println!("get_page_url : {}", doujin.get_page_url(1));
        //println!("get_pages_urls : {:?}", doujin.get_pages_urls());*/

        println!(
            "\nTime to execute: {:?}\n",
            SystemTime::now()
                .duration_since(time_start)
                .expect("Time went backwards")
        );
    }
}
