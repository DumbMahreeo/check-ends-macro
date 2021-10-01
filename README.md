# Use
It's like a match, but it returns an Option

```rust
let to_match = "hello world";

let opt = match_start!{to_match,
    "hello" | "something" => "something else"
};

if let Some(value) = opt {
    println!("{}", value);
}
```
