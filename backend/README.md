# Backend API Server

Rust backend for the Roblox Level Builder MVP. Accepts Space JSON from the frontend and generates `.rbxlx` files for Roblox Studio.

## Setup

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Installation

```bash
# Dependencies are managed by Cargo
# No manual installation needed
```

## Development

### Run Server

```bash
cargo run
```

Server will start on `http://localhost:4000` by default.

### Configure Port

Set the `PORT` environment variable:

```bash
PORT=8080 cargo run
```

## API Endpoints

### POST `/api/export`

Exports Space JSON to `.rbxlx` file.

**Request:**
- Method: `POST`
- Content-Type: `application/json`
- Body: Space JSON (see `_docs/03_space_json_schema.md`)

**Response:**
- Status: `200 OK` on success
- Content-Type: `application/octet-stream`
- Content-Disposition: `attachment; filename="level.rbxlx"`
- Body: `.rbxlx` file content

**Error Response:**
- Status: `400 Bad Request`
- Content-Type: `application/json`
- Body:
  ```json
  {
    "error": "ERROR_TYPE",
    "message": "Human-readable error message"
  }
  ```

**Example Request:**
```bash
curl -X POST http://localhost:4000/api/export \
  -H "Content-Type: application/json" \
  -d '{
    "schemaVersion": 1,
    "name": "My Level",
    "blocks": [
      {"x": 0, "y": 0, "z": 0, "color": "#FF0000"}
    ]
  }'
```

## Project Structure

```
backend/
├── Cargo.toml          # Rust project configuration
├── src/
│   ├── main.rs         # HTTP server and routes
│   └── models.rs       # Space JSON data structures
└── README.md           # This file
```

## Dependencies

- `axum` - HTTP web framework
- `tokio` - Async runtime
- `serde` + `serde_json` - JSON serialization
- `tower-http` - CORS middleware

## Phase Status

- ✅ **Phase 5**: API Setup (complete)
- ⏳ **Phase 6**: Validation (pending)
- ⏳ **Phase 7**: RBXLX Generation (pending)

## Notes

- CORS is configured to allow all origins for MVP development
- Schema version validation is implemented (only version 1 supported)
- RBXLX generation is currently a placeholder (full implementation in Phase 7)

