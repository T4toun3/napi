# NAPI

## Macro
- [x] Add a macro to `impl Table` by using a procedural macro

## Search
- [x] Get popular doujinshi
- [x] Add way to set a maximun of page research
- [ ] Add ability to set default value (*ex: Set `Sort` to `PopularWeek`. `correct([Page(2)]) -> [Page(2), Sort::PopularWeek]`*)
- [ ] Add time args
- [ ] Search with url (`from_url`)

## Doujin
- [ ] Cast `media_id` into `u32`

## Rustianisation (en franglais)
*Tout ce qui est écrit ici n'est que proposition et aucunement un plan pour le future de cette API*
- [x] créer une `enum` pour les différents type de tag (tag, artist, parody...)

# GITHUB
- [ ] Add how to use napi in the README file

# CRATES.IO
- [ ] Publish napi on [crates.io](crates.io)

# 2.0

## Doujin
- [ ] Create new doujin with similar doujin
    - impl manually Deserialize