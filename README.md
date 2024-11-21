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

- `src/main.rs`: Contains the main server code that is the REST API logic and an internal loop to check when orders are finished.
- `src/tests.rs`: Contains unit tests for the API logic.
- `models/`: A library crate that contains shared data structures.
- `models/src/lib.rs`: Library that is shared data structures used by both the server and client.
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

### NOTES + ASSUMPTIONS

I checked that the background job works by adding some items, waiting a few seconds (I have it configured as 5), to see they were removed.
I decided to simplify the requirements by making the background job for getting rid of finished orders to be in seconds from now.
This is just for testing and getting something working fast.

```
➜  Restaurant API git:(master) ✗ curl -X POST http://localhost:3000/add -H "Content-Type: application/json" -d '[
{"item_name": "Pizza", "table_number": 1, "cooking_time": 10},
{"item_name": "Burger", "table_number": 2, "cooking_time": 12}
]'
[{"item_name":"Pizza","table_number":1,"cooking_time":10,"created_at":1732167098},{"item_name":"Burger","table_number":2,"cooking_time":12,"created_at":1732167098}]%
➜  Restaurant API git:(master) ✗ curl -X GET http://localhost:3000/query/2
[]%
➜  Restaurant API git:(master) ✗ curl -X GET http://localhost:3000/query/1
[]%
```

If you want to make it minutes elapsed, I believe you can convert the `cooking_time` field to be compatible with `created_at` which is in UNIX time and then add that to it in the backgound task filter.