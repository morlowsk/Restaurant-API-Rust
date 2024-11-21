# Restaurant API

This project implements a RESTful API for a restaurant that allows adding, removing, and querying menu items. The API is built using Rust and the `poem` framework. The client code is also written in Rust and uses the `reqwest` crate to make HTTP requests to the server.

## Project Structure

The project is structured as follows:

```
restaurant_api/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── tests.rs
├── models/
│ ├── Cargo.toml
│ └── src/
│   ├── lib.rs
└── client/  
├ ├── Cargo.toml
└── src/
│ ├── main.rs
│ ├── client.rs
```

- `src/main.rs`: Contains the main server code and the REST API logic.
- `src/tests.rs`: Contains unit tests for the API logic.
- `models/`: A library crate that contains shared data structures.
- `src/lib.rs`: Contains shared data structures used by both the server and client.
- `client/`: A binary crate that contains the client code for making HTTP requests to the server.
- `src/client.rs`: Contains the client code for making HTTP requests to the server.
- `src/main.rs`: Code that utilizes the client to make some calls to the server, simulating multiple calls at once

## Dependencies

The project uses the following dependencies:

- `poem`: A web framework for building RESTful APIs.
- `reqwest`: A high-level HTTP client.
- `serde`: A framework for serializing and deserializing Rust data structures.
- `tokio`: An asynchronous runtime for Rust.
- `rand`: A library for generating random numbers.

## Running the Servers

To run the API server, use the following command:

```bash
cargo run --bin restaurant_api
```

To run the simulation of the client calling the server, use the following command (after you run the API server)
in the client/ directory:

```bash
cargo run --bin client
```

## API Endpoints

The server exposes the following API endpoints:

- POST /add: Adds one or more menu items to a specific table.

- DELETE /remove/:table_number/:item_name: Removes a specific item from a specific table.

- GET /query/:table_number: Queries all items for a specific table.

- GET /query/:table_number/:item_name: Queries a specific item for a specific table.

## Running the Unit Tests

To run the unit tests, use the following command in src/ directory:

```bash
cargo test
```