# Happi

## Macro
- [x] Add a macro to `impl Table` by using a procedural macro

## Search
- [x] Get popular gallery
- [x] Add way to set a maximun of page research
- [ ] Add ability to set default value (*ex: Set `Sort` to `PopularWeek`. `correct([Page(2)]) -> [Page(2), Sort::PopularWeek]`*)
- [x] Add time args
- [ ] Add tests for TimeRange
- [ ] Manage args

## Gallery
- [x] Cast `media_id` into `u32`
- [ ] use this :
```rust
use chrono::{serde::ts_seconds, DateTime, Utc};
#[serde(with = "ts_seconds")]
pub upload_date: DateTime<Utc>,
```

## Table
- [x] Add new method to access to the entries of a table

## Future
- [ ] ASYNC

# GITHUB
- [ ] Add how to use happi in the README file

# CRATES.IO
- [ ] Publish happi on [crates.io](https://crates.io)

## Gallery
- [x] Create new gallery with similar gallery

## Other
- [ ] look at the [hentai](https://crates.io/crates/hentai) crate