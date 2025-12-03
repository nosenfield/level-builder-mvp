# Space JSON Schema

## Document Metadata
- Scope: MVP
- Version: 1.0
- Status: Active

## Purpose

Space JSON is the intermediate data format between the web-based level builder and the Rust backend. It represents the complete state of a user's level in a serializable format.

## Schema Definition

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SpaceJSON",
  "type": "object",
  "required": ["schemaVersion", "blocks"],
  "properties": {
    "schemaVersion": {
      "type": "integer",
      "const": 1,
      "description": "Schema version for forward compatibility"
    },
    "name": {
      "type": "string",
      "maxLength": 50,
      "default": "Untitled Level",
      "description": "Human-readable level name"
    },
    "blocks": {
      "type": "array",
      "items": { "$ref": "#/definitions/Block" },
      "maxItems": 10000,
      "description": "Array of all blocks in the level"
    }
  },
  "definitions": {
    "Block": {
      "type": "object",
      "required": ["x", "y", "z", "color"],
      "properties": {
        "x": {
          "type": "integer",
          "minimum": -500,
          "maximum": 500,
          "description": "X coordinate in grid units"
        },
        "y": {
          "type": "integer",
          "minimum": 0,
          "maximum": 500,
          "description": "Y coordinate (height) in grid units"
        },
        "z": {
          "type": "integer",
          "minimum": -500,
          "maximum": 500,
          "description": "Z coordinate in grid units"
        },
        "color": {
          "type": "string",
          "pattern": "^#[0-9A-Fa-f]{6}$",
          "description": "Hex color code (e.g., #FF0000)"
        }
      }
    }
  }
}
```

## Example Payload

### Minimal Valid Payload

```json
{
  "schemaVersion": 1,
  "blocks": []
}
```

### Typical Payload

```json
{
  "schemaVersion": 1,
  "name": "My First Level",
  "blocks": [
    { "x": 0, "y": 0, "z": 0, "color": "#808080" },
    { "x": 1, "y": 0, "z": 0, "color": "#808080" },
    { "x": 0, "y": 1, "z": 0, "color": "#FF0000" },
    { "x": 0, "y": 0, "z": 1, "color": "#00FF00" }
  ]
}
```

## Field Specifications

### schemaVersion

| Property | Value |
|----------|-------|
| Type | Integer |
| Required | Yes |
| MVP Value | 1 |

Purpose: Enables backward-compatible schema evolution. Backend must reject unknown versions.

### name

| Property | Value |
|----------|-------|
| Type | String |
| Required | No |
| Default | "Untitled Level" |
| Max Length | 50 characters |

Purpose: Used for Roblox place naming and user reference.

Validation:
- Strip leading/trailing whitespace
- Replace invalid filename characters
- Truncate to 50 characters

### blocks

| Property | Value |
|----------|-------|
| Type | Array of Block objects |
| Required | Yes |
| Max Items | 10,000 |

Purpose: Contains all voxel data for the level.

## Block Object

### Coordinate System

```
        +Y (up)
         |
         |
         +------ +X (right)
        /
       /
      +Z (forward)
```

Origin (0, 0, 0) is the center of the level grid.

### Position Constraints

| Axis | Minimum | Maximum | Unit |
|------|---------|---------|------|
| X | -500 | 500 | Grid cells |
| Y | 0 | 500 | Grid cells |
| Z | -500 | 500 | Grid cells |

Notes:
- Y minimum is 0 (no underground blocks in MVP)
- Each grid cell equals 1 Roblox stud
- Total possible blocks: 1001 x 501 x 1001 = ~502 million (theoretical)
- Practical limit: 10,000 blocks per level

### Color Format

| Property | Value |
|----------|-------|
| Format | Hexadecimal RGB |
| Pattern | `#RRGGBB` |
| Case | Insensitive (normalize to uppercase) |

Examples:
- `#FF0000` - Red
- `#00FF00` - Green
- `#0000FF` - Blue
- `#808080` - Gray

## Validation Rules

### Backend Must Validate

1. **Schema Version**
   - Must equal 1 for MVP
   - Reject unknown versions with 400 error

2. **Block Count**
   - Maximum 10,000 blocks
   - Reject oversized payloads with 413 error

3. **Coordinate Bounds**
   - All coordinates within specified ranges
   - Reject out-of-bounds with 400 error

4. **Color Format**
   - Must match hex pattern
   - Reject invalid colors with 400 error

5. **Duplicate Positions**
   - No two blocks at same (x, y, z)
   - Reject duplicates with 400 error

6. **JSON Structure**
   - Valid JSON syntax
   - Required fields present
   - Correct types for all fields

### Error Response Format

```json
{
  "error": "VALIDATION_ERROR",
  "message": "Human-readable description",
  "details": {
    "field": "blocks[42].color",
    "reason": "Invalid hex color format"
  }
}
```

## Coordinate Mapping

### Space JSON to Roblox Part

| Space JSON | Roblox Part Property |
|------------|---------------------|
| x | Position.X |
| y | Position.Y + 0.5 |
| z | Position.Z |
| color | Color3.fromHex() |

Note: Y offset of 0.5 places block bottom at grid level (Roblox Parts are centered).

### Size

All MVP blocks are 1x1x1 studs in Roblox.

```
Part.Size = Vector3.new(1, 1, 1)
```

## Serialization Notes

### Frontend Serialization

```typescript
interface Block {
  x: number;
  y: number;
  z: number;
  color: string;
}

interface SpaceJSON {
  schemaVersion: 1;
  name?: string;
  blocks: Block[];
}

function serialize(blocks: Map<string, Block>): string {
  const payload: SpaceJSON = {
    schemaVersion: 1,
    name: levelName,
    blocks: Array.from(blocks.values())
  };
  return JSON.stringify(payload);
}
```

### Backend Deserialization (Rust)

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SpaceJSON {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    pub name: Option<String>,
    pub blocks: Vec<Block>,
}

#[derive(Deserialize)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub color: String,
}
```

## Future Schema Evolution (Post-MVP)

Reserved fields for future versions:

| Field | Purpose | Version |
|-------|---------|---------|
| `blocks[].material` | Roblox material type | 2 |
| `blocks[].size` | Non-uniform block sizes | 2 |
| `blocks[].tags` | Gameplay tags (kill, spawn) | 2 |
| `metadata` | Level metadata object | 2 |
| `spawnPoint` | Custom spawn location | 2 |

Backward compatibility strategy:
- Always include schemaVersion
- New fields are optional in later versions
- Backend supports all versions up to current

## Related Documents

- `01_architecture_overview.md` - System architecture
- `02_technology_stack.md` - Technology selections
- `04_rbxlx_generation.md` - How Space JSON becomes .rbxlx
- `05_technical_constraints.md` - Size limits and validation
