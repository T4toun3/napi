pub mod string_utils;

pub mod search;

pub mod doujin;

pub mod table;

#[cfg(test)]
mod tests {

    use std::time::{SystemTime};

    use crate::doujin::Doujin;
    use crate::search::{Search, SearchArgs};
    use crate::table::*;

    #[test]
    fn it_works() {
        let time_start = SystemTime::now();

        let artist_table1 = TagTable::new().unwrap();
        let artist_table2 = TagTable::new_by_popularity().unwrap();

        let mut artist_table2 = artist_table2;

        artist_table2.sort_by_alphabetical();

        println!("==== ARTIST_TABLE 1 ====\n{:#?}", artist_table1.tags[0]);
        println!("==== ARTIST_TABLE 2 ====\n{:#?}", artist_table2.tags[0]);

        /*let args = vec![SearchArgs::Page(8), SearchArgs::Page(9)];
        let url = Search::build_url_with_args(args);
        let search = Search::new(&url);*/
        /*let search = Doujin::new(316932).unwrap(); // On fait une recherche sur cette url
        println!("{:#?}", search.similars);*/
        /*let table = TagTable::new().unwrap();
        let table = ArtistTable::new().unwrap();
        let table = CharacterTable::new().unwrap();
        let table = ParodieTable::new().unwrap();
        let table = GroupTable::new().unwrap();*/
        /*println!("{:?}", &table.get_by_id(14520));
        println!("{:?}", &table.get_by_name("ahegao"));
        println!("{:?}", &table.max());
        println!("{:?}", &table.min());*/

        /*search.merge_search_all_pages(); // On récupère toutes les autres pages dans la recherche et les ajoute dans entries en plus de la page précédente
        println!("All results:");
        for result in &search.entries {
            println!("[{}] {}",result.id,result.name) // On affiche toutes les entrées trouvées
        }*/

        /*let rand = Search::random();
        println!("{:#?}", rand);*/

        /*let page = crate::doujin::Doujin::new(search.entries[0].id).unwrap(); // On prend la première page de la recherche
        println!("{:#?}",page.get_pages_image_urls()); // On récupère l'url de toutes les images du manga
        println!("{:#?}",Search::new("https://nhentai.net/search/?q=test&page=1&sort=popular-week"));*/

        /*let doujin = Doujin::new(327341).unwrap();
        println!("{:#?}", doujin);*/

        /*println!("get_page_image_url_small : {}", doujin.get_page_image_url_small(1).unwrap());
        //println!("get_pages_image_urls_small : {:?}", doujin.get_pages_image_urls_small());
        println!("get_page_image_url : {}", doujin.get_page_image_url(1).unwrap());
        //println!("get_pages_image_urls : {:?}", doujin.get_pages_image_urls());
        println!("get_image_cover : {}", doujin.get_image_cover());
        println!("get_image_thumbnail : {}", doujin.get_image_thumbnail());
        println!("get_page_url : {}", doujin.get_page_url(1));
        //println!("get_pages_urls : {:?}", doujin.get_pages_urls());*/
        println!("\nTime to execute: {:?}\n", SystemTime::now().duration_since(time_start).expect("Time went backwards"));
    }
}
