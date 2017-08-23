# rust-backend-example

This is an extremely simple, dependency free example of how Rust can be used as a backend in web development.

A simple HTTP server is spawned that logs the number of requests from different clients, and returns the number of requests as a JSON object `{"requests": 2}`.

That's all it does.

To test it out:
```
cargo run --release
```
And then open index.html in your favorite web browser