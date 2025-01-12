# gotrue-rs-redux

> :exclamation: This is a fork of [Fubinator's gotrue-rs](https://github.com/Fubinator/gotrue-rs). While functionally similar, it has breaking API changes and a more expansive documentation. Credit is given to Fubinator for a base of the development.


[![Rust](https://github.com/j7nw4r/gotrue-rs-redux/actions/workflows/ci.yml/badge.svg)](https://github.com/j7nw4r/gotrue-rs-redux/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/go_true_redux.svg)](https://crates.io/crates/go_true_redux)
[![License: MIT](https://img.shields.io/crates/l/go_true_redux.svg)](#license)

This is a [GoTrue](https://github.com/supabase/gotrue) client implementation in Rust. The library is currently under development.

## Usage
Add the following line to your `Cargo.toml`:

```toml
go_true_redux = "0.2.2"
```

## Examples

To create an User, create a new client and execute the `sign_up` function with email and password:

```rust
use go_true_redux::Client;

#[tokio::main]
async fn main() {
    let url = "http://localhost:9998";
    let mut client = Client::new(url);

    let email = "email@example.com";
    let password = "Abcd1234!";

    let session = client.sign_up(email, password).await?;

    println!("{:?}", session);
}
```

For more information, check out the [API docs](https://docs.rs/go_true_redux/0.2.2/go_true_redux/)!

## Testing

The first thing to do is to start the supabase server in docker:

```sh
cd infra
docker compose up
```

Once the server has been started, the tests can be run:

```sh
cargo test --tests
```

## Contributing

Contributions, issues and feature requests are welcome. Feel free to check out the [issues page](https://github.com/j7nw4r/gotrue-rs-redux) if you want to contribute.
