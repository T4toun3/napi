
mod string_utils;
pub mod search;
pub mod doujin;
pub mod table;
pub mod tag;
pub mod search_args;
pub mod serde_utils;

#[cfg(test)]
mod tests {

    use chrono::NaiveDateTime;

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
    fn search_and_merge_all() {
        let args = vec![
            SearchArgs::Page(2),
            SearchArgs::Text("hello".to_string(), true),
        ];
        let mut search = Search::new(args).unwrap();
        search.merge_search_all_pages(); // On récupère toutes les autres pages dans la recherche et les ajoute dans entries en plus de la page précédente
        assert!(search.entries.len() > 80);
    }

    #[test]
    fn search_and_merge_range() {
        let args = vec![
            SearchArgs::Page(2),
            SearchArgs::Text("hello".to_string(), true),
        ];
        let mut search1 = Search::new(args).unwrap();
        let mut search2 =  search1.clone();
        
        search1.merge_search_all_pages();
        search2.merge_search_pages(1..10);
        assert_eq!(search1.entries.len(), search2.entries.len());
    }

    #[test]
    fn search_and_merge_range_limited() {
        let args = vec![
            SearchArgs::Page(2),
            SearchArgs::Text("hello".to_string(), true),
        ];

        let mut search1 = Search::new(args).unwrap();
        let mut search2 =  search1.clone();
        let mut search3 =  search1.clone();
        
        search1.merge_search_all_pages();
        search2.merge_search_pages(2..search2.pages + 1);
        search3.merge_search_pages(1..3);
        assert!(search1.entries.len() > search2.entries.len());
        assert!(search1.entries.len() > search3.entries.len());
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
        Doujin::random().unwrap();
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
    fn similar() {
        let doujin = Doujin::new(316932).unwrap();
        assert_eq!(doujin.similars.len(), 5);
    }

    #[test]
    fn doujin_image_and_page_url() {
        let doujin = Doujin::new(327341).unwrap();

        // assert_eq!(doujin.upload_date.to_string(), "2020-09-02 13:29:34");
        assert_eq!(doujin.get_image_url_small(1).unwrap(), doujin.get_images_urls_small()[0], "https://t.nhentai.net/galleries/1723824/1t.jpg");
        assert_eq!(doujin.get_image_url(10).unwrap(), "https://i.nhentai.net/galleries/1723824/10.jpg");
        assert_eq!(doujin.get_images_urls()[3], "https://i.nhentai.net/galleries/1723824/4.jpg");
        assert_eq!(doujin.get_image_cover(), "https://t.nhentai.net/galleries/1723824/cover.jpg");
        assert_eq!(doujin.get_image_thumbnail(), "https://t.nhentai.net/galleries/1723824/thumb.jpg");
        assert_eq!(doujin.get_page_url(5), "https://nhentai.net/g/327341/5/");
        assert_eq!(doujin.get_pages_urls().len(), 126);
    }

    #[test]
    fn doujin_get_all() {
        let doujin = Doujin::new(141506).unwrap();

        assert_eq!(doujin.get_parodies().iter().map(|t| t.name.clone()).collect::<Vec<_>>(), vec!["kantai collection"]);
        assert_eq!(doujin.get_characters().iter().map(|t| t.name.clone()).collect::<Vec<_>>(), vec!["akizuki", "teitoku"]);
        assert_eq!(doujin.get_tags().iter().map(|t| t.name.clone()).collect::<Vec<_>>(), vec!["schoolgirl uniform", "nakadashi", "x-ray", "blowjob", "sole female", "sole male"]);
        assert_eq!(doujin.get_artists().iter().map(|t| t.name.clone()).collect::<Vec<_>>(), vec!["nakano sora"]);
        assert_eq!(doujin.get_groups().iter().map(|t| t.name.clone()).collect::<Vec<_>>(), vec!["in the sky"]);
        assert_eq!(doujin.get_languages().iter().map(|t| t.name.clone()).collect::<Vec<_>>(), vec!["translated", "chinese"]);
        assert_eq!(doujin.get_category().iter().map(|t| t.name.clone()).collect::<Vec<_>>(), vec!["doujinshi"]);
    }

    #[test]
    fn doujin_all_fields() {
        let doujin = Doujin::new(141506).unwrap();

        assert_eq!(doujin.id, 141506);
        assert_eq!(doujin.media_id, 844469);
        assert_eq!(doujin.title.english, "(C88) [In The Sky (Nakano Sora)] Shuuya ni Omou (Kantai Collection -KanColle-) [Chinese] [\\u5c4f\\u5e55\\u9ad2\\u4e86\\u6f22\\u5316\\u7d44]");
        assert_eq!(doujin.upload_date, NaiveDateTime::parse_from_str("2015-08-21 13:13:16", "%Y-%m-%d %H:%M:%S").unwrap());
        assert_eq!(doujin.tags.len(), 14);
        assert_eq!(doujin.num_pages, 24);
        assert_eq!(doujin.images.pages.len(), 24);
        assert!(doujin.num_favorites > 400);
        assert_eq!(doujin.similars.len(), 5);
    }

    #[test]
    fn doujin_title() {
        let doujin = Doujin::new(304826).unwrap();

        assert_eq!(doujin.title.english, "(C97) [DOLL PLAY (Kurosu Gatari)] Galar no Yoru no Sugata | Galar\\u2019s Night view (Pok\\u00e9mon Sword and Shield) [English] [Coffedrug]");
        assert_eq!(doujin.title.japanese, "(C97) [DOLL PLAY (\\u9ed2\\u5de3\\u30ac\\u30bf\\u30ea)] \\u30ac\\u30e9\\u30eb\\u306e\\u591c\\u306e\\u3059\\u304c\\u305f (\\u30dd\\u30b1\\u30c3\\u30c8\\u30e2\\u30f3\\u30b9\\u30bf\\u30fc \\u30bd\\u30fc\\u30c9\\u30fb\\u30b7\\u30fc\\u30eb\\u30c9) [\\u82f1\\u8a33]");
        assert_eq!(doujin.title.pretty, "Galar no Yoru no Sugata | Galar\\u2019s Night view");
    }
}
