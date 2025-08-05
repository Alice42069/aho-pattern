Aho-Corasick but with patterns

```rust
fn main() {
    let patterns = PatternBytes::patterns_from_strs(&[
        "4A 8D 07 ? ? ? ? 4A 8B 03 4A 8D 07 ? ? ? ? 4A 8B 04 4A 8D 07 ? ? ? ? 4B 8B 02",
        "F2 01 07 ? ? ? ? C5 82 3F ? ? ? ? 02",
        "01 85 BA 11 02 02",
        "? ? ? 11 13 4C 12 46 11 2A 0F",
        "? ? ? 01 88 ? ? ? ? 43 11 31 F2",
    ])
    .unwrap();

    let haystack = [...];

    let results = AhoCorasick::find_patterns(&haystack, patterns).unwrap();

    println!("{:?}", results);
}
