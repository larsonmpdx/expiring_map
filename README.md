# Expiring Map

The expiring map is a wrapper around a hash map, where each entry expires some time after being inserted. The intended use is for caching in places where you want to ensure the data does not become stale. 

## Usage

```rust
use expiring_map::ExpiringMap;
use std::time::Duration;

let time_to_live = Duration::from_secs(60);
let mut map = ExpiringMap::new(time_to_live);

map.insert("keyA".to_owned(), "valA".to_owned());

assert_eq!(Some(&"valA".to_owned()), map.get("keyA"));

// after 60 seconds has passed since "keyA" was inserted
// map.remove_expired_entries();
```

Even if map entries have expired, they are not removed from the map until `remove_expired_entries` is called. 

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
