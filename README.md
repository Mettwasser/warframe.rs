# warframe.rs

An async crate to wrap the [Worldstate API](https://docs.warframestat.us).

Use this crate if you want to make a Warframe-related rust project that is async.

## Getting started
To install, simply run `cargo add warframe`.

### Example
```rust,no_run
use warframe::worldstate::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let client = Client::new();

    match client.fetch::<Cetus>().await {
        Ok(cetus) => {
            println!(
                "It is currently {} on cetus. It will be {} in {}",
                cetus.state,
                cetus.state.opposite(),
                cetus.eta()
            );
            Ok(())
        }
        Err(why) => Err(why),
    }
}
```

## Contributing
See [CONTRIBUTING](CONTRIBUTING.md)

### Commitlint

Commit messages are linting in the PR
