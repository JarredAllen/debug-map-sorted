debug-map-sorted
----
[![crates.io](https://img.shields.io/crates/v/debug-map-sorted.svg)](https://crates.io/crates/debug-map-sorted)

An implementation of Debug on a wrapper for [`HashMap`] which displays the output in sorted order.

See [`SortedHashMapDebugOutput`] for more info.

## Example usage

```rust
# use std::collections::HashMap;
use debug_map_sorted::SortedOutputExt;

let data: HashMap<usize, &'static str> = [(0, "zero"), (1, "one"), (2, "two")]
                                                .into_iter().collect();
assert_eq!(
    format!("{:?}", data.sorted_debug()),
    "{0: \"zero\", 1: \"one\", 2: \"two\"}"
);
```
