# Happi

## Macro
- [x] Add a macro to `impl Table` by using a procedural macro

## Search
- [x] Get popular gallery
- [x] Add way to set a maximun of page research
- [ ] Add ability to set default value (*ex: Set `Sort` to `PopularWeek`. `correct([Page(2)]) -> [Page(2), Sort::PopularWeek]`*)
- [ ] Add time args
- [ ] Manage args

## Gallery
- [x] Cast `media_id` into `u32`

## Table
- [x] Add new method to access to the entries of a table

## Rustianisation (en franglais)
*Tout ce qui est écrit ici n'est que proposition et aucunement un plan pour le future de cette API*
- [x] créer une `enum` pour les différents type de tag (tag, artist, parody...)
- [ ] ASYNC

# GITHUB
- [ ] Add how to use napi in the README file

# CRATES.IO
- [ ] Publish napi on [crates.io](crates.io)

# 2.0

## Gallery
- [ ] Create new gallery with similar gallery
    - impl manually Deserialize