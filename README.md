# Rust TCP Echo Server

A multithreaded TCP Echo Server written in Rust.

This project demonstrates the fundamentals of network programming in Rust, including TCP socket management, concurrent client handling, error management, and safe memory ownership.

## Features

* TCP server implementation using Rust standard library
* Concurrent client handling with threads
* Echo protocol support
* Graceful client disconnection handling
* Connection reset detection
* Configurable listening address and port
* No external dependencies

## Architecture

The server follows a simple thread-per-connection architecture:

1. A `TcpListener` binds to a specified address.
2. The server waits for incoming connections.
3. Each new client connection is handled in a dedicated thread.
4. The server continuously reads incoming data.
5. Received bytes are immediately sent back to the client.
6. The connection remains active until the client disconnects or an error occurs.

```text
Client A ──┐
           ├── Thread A
Client B ──┤
           ├── Thread B
Client C ──┘
                │
                ▼
          TCP Echo Server
```

## Getting Started

### Requirements

* Rust 1.70+
* Cargo

### Clone the repository

```bash
git clone https://github.com/your-username/rust-tcp-echo-server.git
cd rust-tcp-echo-server
```

### Build

```bash
cargo build --release
```

### Run

Default address:

```bash
cargo run
```

Server starts on:

```text
127.0.0.1:9000
```

Custom address:

```bash
cargo run -- 0.0.0.0:8080
```

## Testing

Using Netcat:

```bash
nc 127.0.0.1 9000
```

Send:

```text
Hello World
```

Receive:

```text
Hello World
```

## Example Logs

```text
server listening on 127.0.0.1:9000

handling connection from: 127.0.0.1:53214

client 127.0.0.1:53214 close connection

connection finished 127.0.0.1:53214
```

## Technical Concepts Demonstrated

* TCP Networking
* Socket Programming
* Ownership and Borrowing
* Error Handling
* Thread Management
* Concurrent Programming
* Rust Standard Library Networking APIs

## Current Limitations

This implementation intentionally keeps the design simple.

Current limitations include:

* One operating system thread per client
* No connection timeout
* No TLS encryption
* No structured logging
* No thread pool
* No async runtime

## Future Improvements

### Networking

* [ ] IPv6 support
* [ ] Configurable buffer size
* [ ] Connection timeout support
* [ ] Graceful shutdown mechanism

### Performance

* [ ] Migrate to Tokio
* [ ] Async I/O support
* [ ] Thread pool implementation
* [ ] Benchmarking

### Observability

* [ ] Structured logging with tracing
* [ ] Metrics collection
* [ ] Prometheus integration

### Security

* [ ] TLS support
* [ ] Rate limiting
* [ ] Client authentication

### Testing

* [ ] Unit tests
* [ ] Integration tests
* [ ] Load testing

## Learning Goals

This project was built to learn:

* Rust networking APIs
* Concurrent server design
* TCP communication
* Error handling patterns
* Ownership in real-world applications

## License

This project is licensed under the MIT License. See the LICENSE file for details.


## Author

Nahounou Gosse

Passionate about systems programming, backend development, cybersecurity, artificial intelligence, and Rust.
