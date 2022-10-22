# js_lib

The `js_lib` crate provides simple 'javascript-like' functions.

## Making a http get request

```rust
use js_lib::fetch;
let result = fetch("https://www.google.com/").await;
```