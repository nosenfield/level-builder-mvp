# System Patterns: mvp

**Last Updated**: December 2024

## Architecture Overview

### System Design

Three-tier architecture: Browser Frontend → Rust Backend → Roblox Studio

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Web Browser   │────>│  Rust Backend   │────>│  Roblox Studio  │
│   (three.js)    │     │   (rbx-dom)     │     │   (User's PC)   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
        │                       │
        │   Space JSON          │   .rbxlx file
        │   (POST)              │   (Response)
        └───────────────────────┘
```

**Data Flow**:
1. User builds level in browser (three.js voxel editor)
2. Frontend serializes to Space JSON format
3. POST to backend `/api/export`
4. Backend validates and generates `.rbxlx` XML
5. Browser downloads file
6. User opens in Roblox Studio

### Module Structure

**Frontend** (TypeScript + three.js):
```
frontend/src/
├── constants.ts    # Shared constants (BLOCK_INTERACTION_RANGE, etc.)
├── core/           # Scene, camera, renderer setup
├── control/        # Camera controls, block placement/removal
├── player/         # Player state (mode, speed)
├── terrain/        # Block rendering, terrain generation
│   ├── mesh/       # Block geometry, materials
│   ├── highlight/  # Block preview/highlight (raycasts against terrain.blocks[])
│   └── worker/     # Terrain generation workers
├── ui/             # UI components (hotbar, FPS, joystick)
├── audio/          # Sound effects and music
└── utils/          # Utility functions
```

**Backend** (Rust - to be implemented):
```
backend/
├── src/
│   ├── api/        # HTTP handlers (Axum/Actix-web)
│   ├── export/     # Space JSON → .rbxlx conversion
│   ├── validation/ # Input validation
│   └── models/     # Data structures
```

---

## Design Patterns

### Pattern 1: InstancedMesh for Block Rendering
**When to use**: Rendering many blocks of the same type efficiently
**Example**:
```typescript
// Create InstancedMesh for each color type
const blocks: THREE.InstancedMesh[] = []
for (let i = 0; i < colorTypes.length; i++) {
  blocks[i] = new THREE.InstancedMesh(
    geometry,
    materials[i],
    maxCount
  )
}

// Set matrix for each block instance
blocks[BlockType.red].setMatrixAt(index, matrix)
```

**Why**: Performance optimization - single draw call for all blocks of same color.

### Pattern 2: Block Storage with Metadata
**When to use**: Storing block data with position, color, and future tags
**Example**:
```typescript
class Block {
  x: number
  y: number
  z: number
  type: BlockType
  color: string        // Hex color for export
  placed: boolean
  isGround?: boolean   // Ground protection flag
  tags?: string[]      // Future: game mechanics tags
  tagConfig?: Record<string, any>  // Future: tag configs
}
```

**Why**: Single source of truth, extensible for future features, easy to serialize.

### Pattern 3: Reduced Memory Allocation for Unused Types
**When to use**: Block types that exist in enum but aren't actively used
**Example**:
```typescript
// Set allocation factor to 0 for unused texture blocks
blocksFactor = [
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 12 texture blocks (unused)
  0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5  // 10 color blocks (used)
]
```

**Why**: Prevents unnecessary memory allocation for InstancedMesh instances that will never be used.

### Pattern 4: Color-to-BlockType Mapping
**When to use**: Converting color selection to BlockType enum for rendering
**Example**:
```typescript
const COLOR_TO_BLOCKTYPE: Map<string, BlockType> = new Map([
  ['#FF0000', BlockType.red],
  ['#00FF00', BlockType.green],
  // ... etc
])

function getBlockTypeForColor(color: string): BlockType {
  return COLOR_TO_BLOCKTYPE.get(color) || BlockType.red
}
```

**Why**: Decouples color selection from rendering system, allows flexible color assignment.

### Pattern 5: CSS Class-Based Styling (UI)
**When to use**: Styling UI elements instead of inline styles
**Example**:
```typescript
// Create element with CSS classes
let colorSquare = document.createElement('div')
colorSquare.className = 'color-square'
if (isWhite) {
  colorSquare.classList.add('color-white')
}
```

**Why**: Better separation of concerns, easier maintenance, consistent styling approach.

### Pattern 6: Map-Based Block Lookups (Performance)
**When to use**: Fast O(1) block lookups by position instead of O(n) array searches
**Example**:
```typescript
// In Terrain class
blocksMap = new Map<string, Block>()  // Key: `${x}_${y}_${z}`

// When adding block
const blockKey = `${x}_${y}_${z}`
this.blocksMap.set(blockKey, block)

// When looking up block
const block = this.blocksMap.get(`${x}_${y}_${z}`)
```

**Why**: With 10,000+ blocks, linear search becomes slow. Map provides O(1) lookups.

### Pattern 7: Cached Performance Counters
**When to use**: Tracking counts that are checked frequently (e.g., block limits)
**Example**:
```typescript
// In Terrain class
userPlacedBlockCount = 0

// On block placement (non-ground blocks)
incrementUserPlacedCount() {
  this.userPlacedBlockCount++
}

// On block removal (non-ground blocks)
decrementUserPlacedCount() {
  if (this.userPlacedBlockCount > 0) {
    this.userPlacedBlockCount--
  }
}

// Get count (O(1) instead of O(n) filter)
getUserPlacedBlockCount(): number {
  return this.userPlacedBlockCount
}
```

**Why**: With 10,000+ blocks, filtering array on every placement check is O(n). Cached counter is O(1).

### Pattern 8: Direct Raycasting Against Rendered Blocks
**When to use**: Highlight/raycast systems that need to detect blocks under crosshair
**Example**:
```typescript
// Raycast directly against rendered InstancedMesh array
this.raycaster.setFromCamera({ x: 0, y: 0 }, this.camera)
this.raycaster.far = BLOCK_INTERACTION_RANGE // 50 units

const intersects = this.raycaster.intersectObjects(this.terrain.blocks)
if (intersects.length > 0) {
  const hit = intersects[0]
  // Extract position from InstancedMesh instance
  const matrix = new THREE.Matrix4()
  hit.object.getMatrixAt(hit.instanceId, matrix)
  const position = new THREE.Vector3().setFromMatrixPosition(matrix)
}
```

**Why**: 
- Ensures consistency with block placement/removal (uses same meshes)
- Eliminates range mismatches (no separate instanceMesh to maintain)
- Three.js handles bounding box culling automatically
- Simpler code, fewer edge cases

---

## Key Invariants

### Invariant 1: Block Position Integrity
**Description**: All blocks must have integer coordinates (x, y, z are whole numbers)
**Enforcement**: Grid snapping on placement, coordinate validation before export
**Why**: Ensures consistent rendering and valid Space JSON export

### Invariant 2: Block Limit Enforcement
**Description**: Maximum 10,000 user-placed blocks per level (ground plane excluded)
**Enforcement**: Count check before placement, filter on export
**Why**: Performance and file size constraints

### Invariant 3: Coordinate Bounds
**Description**: All blocks must be within bounds: X/Z: -500 to 500, Y: 0 to 500
**Enforcement**: Validation on placement, validation on export
**Why**: Prevents invalid Space JSON and ensures reasonable level sizes

### Invariant 4: Ground Plane Protection
**Description**: Ground plane blocks (isGround: true) cannot be removed
**Enforcement**: Check before removal, exclude from export
**Why**: Maintains level foundation, prevents user errors

---

## Data Flow

### Block Placement Flow
1. User clicks in 3D space
2. Raycaster detects block face
3. Calculate adjacent position (face normal + position)
4. Check bounds and block limit
5. Create Block instance with color
6. Add to `customBlocks` array
7. Update InstancedMesh matrix
8. Update block counter UI

### Export Flow
1. User clicks Export button
2. Filter `customBlocks` (placed: true, !isGround)
3. Map blocks to Space JSON format
4. Serialize to JSON string
5. POST to backend API
6. Backend validates and generates `.rbxlx`
7. Return file with Content-Disposition header
8. Browser triggers download

### State Management
- **Frontend**: In-memory only (no persistence)
- **Block state**: Stored in `terrain.customBlocks` array
- **Selected color**: Stored in `control.holdingBlock` (BlockType)
- **Camera state**: Managed by PointerLockControls
- **UI state**: Managed by UI class (menu visibility, etc.)

---

## Integration Points

### Backend API (`/api/export`)
- **Purpose**: Convert Space JSON to `.rbxlx` file
- **How we use it**: POST Space JSON, receive binary file response
- **Failure handling**: Display error message to user, log error details
- **Expected response**: 200 OK with `.rbxlx` file, or 400/500 with error JSON

### Roblox Studio (User's Local)
- **Purpose**: Play testing exported levels
- **How we use it**: Generate valid `.rbxlx` files that open without errors
- **Failure handling**: Validation ensures files are always valid
- **Dependencies**: User must have Roblox Studio installed

---

## Performance Considerations

### Optimization Strategy
- **InstancedMesh**: Batch rendering of same-color blocks (reduces draw calls)
- **Map-based lookups**: O(1) block position lookups instead of O(n) array searches
- **Direct raycasting**: Raycast against rendered blocks (terrain.blocks[]) - Three.js handles culling automatically
- **Shared constants**: Extract magic numbers to shared constants.ts for consistency
- **Reduced allocations**: Set unused block type factors to 0, reduce maxCount for manual placement
- **Cached counters**: O(1) user-placed block count instead of O(n) array filtering
- **Chunk-based generation**: (Currently disabled for MVP, but structure exists)
- **Worker threads**: Terrain generation in background (disabled for MVP)
- **Lazy loading**: Only render visible blocks (future optimization)

### Caching Strategy
- **Materials**: Created once, reused for all blocks of same color
- **Geometry**: Single BoxGeometry shared across all InstancedMeshes
- **Block counters**: Cached user-placed block count (incremented/decremented on place/remove)
- **No network caching**: Stateless backend, no caching needed

### Scaling Approach
- **Frontend**: Client-side rendering, scales with user's hardware
- **Backend**: Stateless API, can scale horizontally
- **Block limit**: 10,000 blocks per level (hard limit for MVP)
- **Concurrent users**: Backend must handle 100+ simultaneous exports
