# Jupiter Price Stream

Jupiter Price Stream is a Rust-based WebSocket server that provides real-time price information for various tokens using the Jupiter API. It supports both WebSocket connections for real-time updates and a REST API for on-demand price queries.

## Features

- Real-time token price updates via WebSocket
- On-demand token price queries via REST API
- Caching of token prices to reduce API calls
- Support for multiple concurrent WebSocket connections
- Configurable background task for updating prices

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/simplysabir/token-price.git
   cd token-price
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

### Running the Server

To start the server, run:

```
cargo run --release
```

The server will start on `localhost:8080`.

### WebSocket Connection

To connect to the WebSocket for real-time price updates:

```
ws://localhost:8080/ws
```

You can use a WebSocket client like `websocat` to test the connection:

```
websocat ws://localhost:8080/ws
```

### REST API

To query prices for specific tokens, send a POST request to:

```
http://localhost:8080/prices
```

The request body should be a JSON object with an `addresses` array:

```json
{
  "addresses": [
    "So11111111111111111111111111111111111111112",
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  ]
}
```

Example using curl:

```bash
curl -X POST http://localhost:8080/prices \
     -H "Content-Type: application/json" \
     -d '{"addresses": ["So11111111111111111111111111111111111111112", "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"]}'
```

## Project Structure

- `main.rs`: Contains the main application logic, server setup, and API endpoints.
- `websocket.rs`: Handles WebSocket connections and real-time price updates.
- `jupiter_api.rs`: Manages interactions with the Jupiter API for fetching token prices.

## Configuration

To modify the list of tokens for which prices are regularly updated, edit the `token_addresses` vector in the `price_update_task` function in `main.rs`.

## Dependencies

Key dependencies include:

- `actix-web`: Web framework for Rust
- `actix-web-actors`: WebSocket support for Actix
- `tokio`: Asynchronous runtime for Rust
- `serde`: Serialization and deserialization of JSON
- `reqwest`: HTTP client for API requests

For a full list of dependencies, see the `Cargo.toml` file.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Jupiter API for providing token price data
- Actix Web framework for Rust

## Disclaimer

This project is for educational purposes only. Always verify token prices from multiple sources before making financial decisions.