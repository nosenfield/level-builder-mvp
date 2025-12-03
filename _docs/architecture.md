# Architecture Overview

## Document Metadata
- Scope: MVP
- Version: 1.0
- Status: Active

## System Purpose

Web-based voxel level builder that exports playable Roblox places. Users create 3D block structures in browser, download `.rbxlx` files, and open them in Roblox Studio for immediate play testing.

## MVP User Flow

```
[Browser] --> [Rust Backend] --> [.rbxlx File] --> [Roblox Studio]
   |               |                                      |
   v               v                                      v
Level Builder   Space JSON     Download File         Play Test
(three.js)      Generation     Generation           (Auto-spawn)
```

1. User opens web app, views empty 3D grid
2. User places/removes 1x1x1 colored blocks
3. User clicks "Export"
4. Frontend sends Space JSON to backend
5. Backend generates `.rbxlx` file
6. User downloads file, opens in Roblox Studio
7. User plays level with default spawn and movement

## Component Architecture

### Frontend (Browser)

**Purpose:** 3D voxel editing interface

**Responsibilities:**
- Render editable 3D grid space
- Handle block placement/removal via mouse input
- Manage camera controls (orbit, pan, zoom)
- Serialize level state to Space JSON
- POST Space JSON to backend API
- Handle file download response

**Technology:** TypeScript, three.js, Vite

### Backend (Server)

**Purpose:** Convert Space JSON to valid Roblox place files

**Responsibilities:**
- Accept Space JSON via HTTP POST
- Validate schema and constraints
- Generate Roblox DataModel structure
- Serialize to `.rbxlx` XML format
- Return file as downloadable response

**Technology:** Rust, rbx-dom ecosystem, Actix-web or Axum

### Output (Roblox Studio)

**Purpose:** Playable game environment

**Contents:**
- Workspace with level geometry (Parts)
- SpawnLocation for player spawn
- Default Roblox services (Lighting, Players, etc.)
- No custom scripts required for MVP

## Data Flow

### Export Request

```
Frontend                    Backend
   |                          |
   |  POST /api/export        |
   |  Body: Space JSON        |
   |------------------------->|
   |                          |  Validate JSON
   |                          |  Build DataModel
   |                          |  Serialize .rbxlx
   |  200 OK                  |
   |  Content-Disposition:    |
   |  attachment; filename=   |
   |  level.rbxlx             |
   |<-------------------------|
   |                          |
   v                          |
Download                      |
Triggered                     |
```

### Space JSON Structure (Summary)

```json
{
  "schemaVersion": 1,
  "name": "My Level",
  "blocks": [
    { "x": 0, "y": 0, "z": 0, "color": "#FF0000" }
  ]
}
```

Full schema defined in `03_space_json_schema.md`.

## Service Boundaries

| Component | Input | Output | State |
|-----------|-------|--------|-------|
| Frontend | User input | Space JSON | In-memory only |
| Backend | Space JSON | .rbxlx bytes | Stateless |
| Roblox Studio | .rbxlx file | Playable place | Local file |

## Key Constraints

### MVP Scope Limits

- No user accounts or persistence
- No cloud save functionality
- No multiplayer or collaboration
- No custom game mechanics
- No LLM integration
- Single block size (1x1x1 studs)
- Position and color properties only

### Technical Constraints

- Frontend must work in modern browsers (Chrome, Firefox, Safari, Edge)
- Backend must handle reasonable level sizes (target: 10,000 blocks max)
- Generated `.rbxlx` must open without errors in Roblox Studio
- Export time should be under 5 seconds for typical levels

## Post-MVP Considerations (Out of Scope)

The following features are documented for future reference but excluded from MVP:

- LLM-assisted level design
- Custom game mechanics (kill zones, checkpoints)
- Roblox Studio plugin for debugging
- Cloud-based error patching
- Block attributes beyond position/color
- Variable block sizes
- Terrain generation

## Related Documents

- `02_technology_stack.md` - Technology selections and rationale
- `03_space_json_schema.md` - Space JSON specification
- `04_rbxlx_generation.md` - Roblox file generation details
- `05_technical_constraints.md` - Limitations and mitigations
