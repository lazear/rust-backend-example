# rust-backend-example
This is an extremely simple, dependency free example of how Rust can be used as a backend/API in web development.

The server listens to requests on port 8080 and simply logs the number of requests from each client IP address, and then returns the number of requests as a JSON object `{"requests": 2}`. 

That's all it does.

To test it out:
```
cargo run --release
```
And then open index.html in your favorite web browser

Or, you can use docker to spawn a container running Nginx that will proxy requests to the rust backend
```
docker build -t rust-backend .
docker run -d -p 80:80 rust-backend
```

At then go to http://localhost in your browser.
