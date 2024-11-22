# Restaurant API

This project implements a RESTful API in RUST for a restaurant that allows adding, removing, and querying menu items.

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
|   ├── integration_tests.rs
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

## Running the Integration Tests

To run the integration tests, use the following command in client/ directory after running the restaurant API:

```bash
cargo test
```

### NOTES + ASSUMPTIONS

The main data structure is very simple, it's just a map of table numbers to a list of orders. The backend API is simple too,
there are only: add, remove, and two query methods. One query is for querying the table for all items and one API is for querying for a specific food item.

Also, there's background job to check if enough time elapsed between `created_at` + `cooking_time` to remove an order from a table.

I checked that the background job works by adding some items, waiting a few seconds (I have it configured to run every 5), to see they were removed.
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

If you want to make it minutes elapsed, you can convert the `cooking_time` field to be compatible with `created_at` which is in UNIX time and then add that to it in the backgound task filter.

I wrote an extra main method in the client library to simulate sending multiple requests using the client library (I have 20, but the project specified 10) randomly chosen.
Given that there's input returned after every call and the server doesn't crash it should show that this is sufficient.
However in the real world you would write performance tests to automate this.

Here's one sample run:

```
Queried items for table 1: []
Remaining items for table 1: []
Queried items for table 1: [MenuItem { item_name: "Ramen", table_number: 1, cooking_time: 13, created_at: 1732169559 }]
Added items: [MenuItem { item_name: "Ramen", table_number: 1, cooking_time: 13, created_at: 1732169559 }, MenuItem { item_name: "Pizza", table_number: 2, cooking_time: 11, created_at: 1732169559 }]
Remaining items for table 1: [MenuItem { item_name: "Ramen", table_number: 1, cooking_time: 13, created_at: 1732169559 }]
Remaining items for table 1: [MenuItem { item_name: "Ramen", table_number: 1, cooking_time: 13, created_at: 1732169559 }]
Added items: [MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }, MenuItem { item_name: "Salad", table_number: 2, cooking_time: 5, created_at: 1732169559 }]
Queried items for table 1: [MenuItem { item_name: "Pizza", table_number: 2, cooking_time: 11, created_at: 1732169559 }, MenuItem { item_name: "Salad", table_number: 2, cooking_time: 5, created_at: 1732169559 }]
Queried items for table 1: [MenuItem { item_name: "Ramen", table_number: 1, cooking_time: 13, created_at: 1732169559 }, MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }]
Added items: [MenuItem { item_name: "German Sausage", table_number: 1, cooking_time: 15, created_at: 1732169559 }, MenuItem { item_name: "Kung Pao Chicken", table_number: 2, cooking_time: 12, created_at: 1732169559 }]
Remaining items for table 1: [MenuItem { item_name: "Ramen", table_number: 1, cooking_time: 13, created_at: 1732169559 }, MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }]
Queried items for table 1: [MenuItem { item_name: "Ramen", table_number: 1, cooking_time: 13, created_at: 1732169559 }, MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }, MenuItem { item_name: "German Sausage", table_number: 1, cooking_time: 15, created_at: 1732169559 }, MenuItem { item_name: "Taco", table_number: 1, cooking_time: 15, created_at: 1732169559 }]
Added items: [MenuItem { item_name: "Taco", table_number: 1, cooking_time: 15, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 2, cooking_time: 5, created_at: 1732169559 }]
Added items: [MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }, MenuItem { item_name: "German Sausage", table_number: 2, cooking_time: 10, created_at: 1732169559 }]
Added items: [MenuItem { item_name: "Salad", table_number: 1, cooking_time: 9, created_at: 1732169559 }, MenuItem { item_name: "Ramen", table_number: 2, cooking_time: 10, created_at: 1732169559 }]
Remaining items for table 1: [MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }, MenuItem { item_name: "Taco", table_number: 1, cooking_time: 15, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }, MenuItem { item_name: "Salad", table_number: 1, cooking_time: 9, created_at: 1732169559 }]
Remaining items for table 1: [MenuItem { item_name: "Ramen", table_number: 1, cooking_time: 13, created_at: 1732169559 }, MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }, MenuItem { item_name: "Taco", table_number: 1, cooking_time: 15, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }, MenuItem { item_name: "Salad", table_number: 1, cooking_time: 9, created_at: 1732169559 }]
Remaining items for table 1: [MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }, MenuItem { item_name: "Taco", table_number: 1, cooking_time: 15, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }]
Added items: [MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }, MenuItem { item_name: "German Sausage", table_number: 2, cooking_time: 12, created_at: 1732169559 }]
Queried items for table 1: [MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }, MenuItem { item_name: "Taco", table_number: 1, cooking_time: 15, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }]
Queried items for table 1 after all operations: [MenuItem { item_name: "Kung Pao Chicken", table_number: 1, cooking_time: 10, created_at: 1732169559 }, MenuItem { item_name: "Taco", table_number: 1, cooking_time: 15, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }, MenuItem { item_name: "Burger", table_number: 1, cooking_time: 6, created_at: 1732169559 }]
```