# `pipedream-rs`

```rust
use pipedream_rs as pd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Access previous step data using pd::STEPS
    println!("{:?}", &*pd::STEPS);

    // Export data using pd::export
    let mut data = serde_json::Map::new();
    data.insert("name".to_owned(), "Luke".to_owned().into());
    pd::export("data", data.into())
}

```

## Tests

Simulate the env vars present on the Pipedream execution environment when running `cargo test`

```bash
PIPEDREAM_STEPS=./test-step-data.json PIPEDREAM_EXPORTS=./test-exports-data cargo test
```
