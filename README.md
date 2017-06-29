# Rust JSONRPC 1.0 Client

Rudimentary support for sending JSONRPC 1.0 requests and receiving responses.

This library is based on [rust-jsonrpc](https://github.com/apoelstra/rust-jsonrpc).

## JSONRPC

To send a request which should retrieve the above structure, consider the following
example code

```rust
#[macro_use] extern crate jsonrpc;
extern crate serde;

#[derive(Serialize, Deserialize)]
struct MyStruct {
    elem1: bool,
    elem2: String,
    elem3: Vec<usize>
}

fn main() {
    // The two Nones are for user/pass for authentication
    let mut client = jsonrpc::client::Client::new("example.org", None, None);
    let request = client.build_request("getmystruct", vec![]);
    match client.send_request(&request).and_then(|res| res.into_result::<MyStruct>()) {
        Ok(mystruct) => // Ok!
        Err(e) => // Not so much.
    }
}

```

