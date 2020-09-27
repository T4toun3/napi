# NAPI

## Macro
- Add a macro to `impl Table` by using a procedural macro

## Doujin
- Get similare doujinshis from a doujinshi
    - impl manually Deserialize

## Search
- Get popular doujinshi
- Add way to set a maximun of page research

## Other
- Optimize mutlithreading for `merge_search_all_pages` and `new()` (Table)

## Rustianisation (en franglais)
*Tout ce qui est écrit ici n'est que proposition et aucunement un plan pour le future de cette API*
- créer une `enum` pour les différents type de tag (tag, artist, parody...)
- merger toutes les tables ensembles pour n'avoir plus que une seule `struct` avec un champs `tag` qui contient les tags et un champs `type` qui contient une `enum` des différent type

# GITHUB
- Add how to use napi in the README file

# CRATE.IO
- Publish napi on [crate.io](crate.io)
