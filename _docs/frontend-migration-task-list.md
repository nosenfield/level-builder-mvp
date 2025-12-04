# Frontend Migration Task List

## Overview

Tasks for adapting the forked three.js Minecraft experience to the Roblox Level Builder MVP requirements. This document supplements [task-list.md](task-list.md) Phase 2-3.

## Source Codebase Reference

| Component | File Path |
|-----------|-----------|
| Entry Point | `frontend/src/main.ts` |
| Scene Setup | `frontend/src/core/index.ts` |
| Player State | `frontend/src/player/index.ts` |
| Controls | `frontend/src/control/index.ts` |
| Terrain | `frontend/src/terrain/index.ts` |
| Block Materials | `frontend/src/terrain/mesh/materials.ts` |
| UI Manager | `frontend/src/ui/index.ts` |
| Hotbar | `frontend/src/ui/bag/index.ts` |

---

## Phase M1: Camera and Controls

### M1.1: Enable Flying Mode as Default
**File:** `frontend/src/player/index.ts`

- Set `mode` default to `flying` instead of `walking`
- This removes gravity and ground collision by default

### M1.2: Remove Physics and Collisions
**File:** `frontend/src/control/index.ts`

- Disable gravity application in movement loop
- Disable collision raycasting (down, up, front, back, left, right)
- Remove jump mechanics
- Camera moves freely through space without obstruction

### M1.3: Remap Altitude Controls (Q/E)
**File:** `frontend/src/control/index.ts`

- Map `E` key to continuous upward movement (positive Y velocity)
- Map `Q` key to continuous downward movement (negative Y velocity)
- Movement along absolute vertical axis (not camera axis)
- Remove existing `Q` toggle-fly-mode functionality

### M1.4: Update Forward/Backward Movement to Include Vertical Component
**File:** `frontend/src/control/index.ts`

- Replace `control.moveForward()` calls with custom movement calculation
- Calculate camera's forward direction vector (including pitch/vertical angle)
- Forward (`W`) moves along camera's full look direction (including vertical component)
- Backward (`S`) moves opposite to camera's look direction
- Use `camera.getWorldDirection()` or similar to get full 3D forward vector
- Apply movement along this vector instead of horizontal-only movement
- This allows looking up/down and moving forward to move vertically as well

### M1.5: Implement Speed Toggle
**File:** `frontend/src/control/index.ts`

- Base speed: walking speed (4 units/sec)
- `Space` key toggles to flying speed (12 units/sec)
- Speed applies to all directional movement (WASD + Q/E)

### M1.6: Swap Click Controls
**File:** `frontend/src/control/index.ts`

**Current State:**
- Left-click (button 0): Removes block
- Right-click (button 2): Places block

**Target State (per PRD US-1.2, US-1.3):**
- Left-click (button 0): Places block at target position
- Right-click (button 2): Removes block at target position

**Implementation Steps:**

1. **Move placement logic from `case 2` to `case 0`:**
   - Move player overlap check (lines ~399-408) to left-click handler
   - Move block placement code (lines ~410-438) to left-click handler
   - Update comment from "right click to put block" to "left click to place block"

2. **Move removal logic from `case 0` to `case 2`:**
   - Move bedrock protection check (lines ~295-302) to right-click handler
   - Move block removal code (lines ~304-385) to right-click handler
   - Update comment from "left click to remove block" to "right click to remove block"
   - Note: `generateAdjacentBlocks` call (line ~385) will be removed in M2.1, but keep it for now

3. **Verify:**
   - Left-click places block using `this.holdingBlock` (selected color/type)
   - Right-click removes block at clicked position
   - Player overlap check prevents placing blocks inside player
   - Bedrock protection check (lines ~295-302) prevents removing bedrock blocks
   - Note: Bedrock protection is temporary; will be replaced with ground block protection in M2.5

### M1.7: Set Initial Camera Position
**File:** `frontend/src/core/index.ts` or `frontend/src/control/index.ts`

- Position: `(40, 10, 40)` - Above ground plane, offset from center
- Look-at target: `(50, 0, 50)` - Center of ground plane
- Angle: Isometric-style view looking down toward center
- Update `camera.position.set()` and `camera.lookAt()` calls

---

## Phase M2: World Initialization

### M2.1: Disable Procedural Terrain Generation
**File:** `frontend/src/terrain/index.ts`

- Skip noise-based terrain generation on init
- Skip tree generation
- Skip chunk loading/unloading system
- Do not call terrain generation worker

### M2.2: Disable Cloud Generation
**File:** `frontend/src/terrain/index.ts`

- Skip cloud InstancedMesh creation
- Remove cloud regeneration on chunk change

### M2.3: Disable Audio
**File:** `frontend/src/audio/index.ts` or `frontend/src/main.ts`

- Skip audio initialization
- Remove sound effect calls from block placement/removal

### M2.4: Create Ground Plane
**File:** `frontend/src/terrain/index.ts` (new function)

- Generate 100x100 block plane at Y=0
- Blocks span X: 0-99, Z: 0-99
- All blocks use grey color (#808080)
- Blocks are 1x1x1 units
- Mark as special "ground" type (non-removable)

### M2.5: Implement Ground Block Protection
**File:** `frontend/src/control/index.ts`

- Check block type before removal
- Prevent removal of ground-type blocks
- Allow removal of user-placed blocks only

---

## Phase M3: Block System

### M3.1: Define Color-Based Block Types
**File:** `frontend/src/terrain/index.ts` (modify BlockType enum)

**Decision:** Create new BlockType enum values for colors, replacing/repurposing existing types.

**Update BlockType enum:**
```typescript
export enum BlockType {
  red = 0,
  green = 1,
  blue = 2,
  yellow = 3,
  orange = 4,
  purple = 5,
  gray = 6,
  white = 7,
  black = 8,
  brown = 9,
  // Keep bedrock for ground plane (if needed)
  bedrock = 10
}
```

**Color mapping per PRD Appendix B:**

| BlockType | Name | Hex |
|-----------|------|-----|
| red | Red | #FF0000 |
| green | Green | #00FF00 |
| blue | Blue | #0000FF |
| yellow | Yellow | #FFFF00 |
| orange | Orange | #FFA500 |
| purple | Purple | #800080 |
| gray | Gray | #808080 |
| white | White | #FFFFFF |
| black | Black | #000000 |
| brown | Brown | #8B4513 |

**Note:** Keep existing `holdingBlock` and `holdingBlocks` structure, but update to use new color BlockTypes.

### M3.2: Create Solid Color Materials
**File:** `frontend/src/terrain/mesh/materials.ts`

- Use `MeshStandardMaterial` with solid color (no texture)
- Set `color` property from hex value
- Maintain material array structure for compatibility
- Allow future texture support via optional texture property

### M3.3: Update Block Rendering
**File:** `frontend/src/terrain/index.ts`

- Create InstancedMesh for each color type
- Replace texture-based materials with color materials
- Maintain existing matrix/transform system

### M3.4: Implement Coordinate Bounds Enforcement
**File:** `frontend/src/control/index.ts`

- X bounds: -500 to 500
- Z bounds: -500 to 500
- Y bounds: 0 to 500
- Prevent block placement outside bounds
- Visual feedback when placement blocked (optional)
- **Note:** Keep adjacent placement logic (Minecraft-style) but ensure grid snapping to integer coordinates

### M3.5: Implement Block Limit Enforcement
**File:** `frontend/src/control/index.ts` or `frontend/src/terrain/index.ts`

- Track user-placed block count (ground plane excluded)
- Count only blocks where `placed === true` AND `isGround !== true`
- Maximum: 10,000 blocks
- Prevent placement when limit reached
- Decrement count on block removal
- Add helper method: `getUserPlacedBlockCount(): number`

---

## Phase M4: UI Adaptation

### M4.1: Update Hotbar to Color Palette
**File:** `frontend/src/ui/bag/index.ts`

- Replace texture icons with solid color squares
- Display 10 color options (matching M3.1 BlockType colors)
- Maintain selection highlight functionality
- Keep number key selection (1-9 for colors, 0 for 10th color)
- Update `holdingBlocks` array to reference new color BlockTypes (red, green, blue, etc.)
- Map color hex values to BlockType enum values

### M4.2: Add Block Counter Display
**File:** `frontend/src/ui/index.ts` (new component)

- Display format: `N / 10,000`
- Position: Footer area per PRD UI layout
- Update in real-time on place/remove
- Warning state when approaching limit (e.g., > 8000)

### M4.3: Update Controls Help Text
**File:** `frontend/src/ui/index.ts`

Update displayed controls:
- WASD: Move
- Q/E: Down/Up
- Space: Fast mode
- Left-click: Place block
- Right-click: Remove block
- Mouse: Look around
- ESC: Release cursor

### M4.4: Remove Unused UI Elements
**File:** `frontend/src/ui/index.ts`

- Remove or hide save/load menu options
- Remove or hide flying mode toggle display
- Remove or hide mobile joystick controls
- Keep menu system for future settings

### M4.5: Add Export Button to Escape Menu
**File:** `frontend/src/ui/index.ts`

- Add "Export" button to the Escape/pause menu (shown when pointer is unlocked)
- Button triggers export flow (see M5.2 and Phase 4 of task-list.md)
- Position: In menu area, visible when menu is shown
- Style: Prominent, matches PRD UI requirements

---

## Phase M5: Data Structure

### M5.1: Enhance Block Class for Export and Game Mechanics
**File:** `frontend/src/terrain/mesh/block.ts`

**Purpose:** Support MVP export (color) and future LLM-driven game mechanics (tags with configs)

**Changes:**

1. **Add `color: string` property** (required for MVP)
   - Hex color format: `"#FF0000"`
   - Stored when block is placed
   - Used for Space JSON export

2. **Add `isGround?: boolean` property** (for ground protection)
   - Marks ground plane blocks as non-removable
   - Used in M2.5 ground protection

3. **Add `tags?: BlockTag[]` property** (optional, for game mechanics)
   - Array of tag strings (e.g., `["killPlayer", "destructable"]`)
   - Supports multiple tags per block
   - Tags are arbitrary strings (no predefined enum)
   - Examples: `"static"`, `"moveable"`, `"destructable"`, `"killPlayer"`, `"damagePlayer"`, `"healPlayer"`, etc.

4. **Add `tagConfig?: Record<string, any>` property** (optional, for tag-specific configs)
   - Key-value pairs for tag configurations
   - Key: tag name, Value: config object (flexible structure)
   - Example: `{ "healPlayer": { "amount": 5, "respawnTime": 60 } }`
   - Allows same tag with different configs on different blocks

5. **Update constructor:**
   ```typescript
   constructor(
     x: number,
     y: number,
     z: number,
     type: BlockType,
     placed: boolean,
     color: string,                    // NEW: Required
     isGround: boolean = false,        // NEW: Optional
     tags?: string[],                  // NEW: Optional tags
     tagConfig?: Record<string, any>   // NEW: Optional configs
   )
   ```

6. **Add helper methods:**
   ```typescript
   hasTag(tag: string): boolean
   addTag(tag: string, config?: any): void
   removeTag(tag: string): void
   getTagConfig(tag: string): any | undefined
   setTagConfig(tag: string, config: any): void
   ```

**Design Rationale:**
- **Flexible tags**: No enum restriction - LLM can assign any tag name
- **Multiple tags**: Single block can have `["killPlayer", "destructable"]`
- **Configurable**: Each tag can have its own config (e.g., respawn time, damage amount)
- **LLM-friendly**: Simple structure for LLM to parse and assign
- **Backward compatible**: Tags/configs optional for MVP

**Example Usage:**
```typescript
// MVP: Basic block with color
const block1 = new Block(0, 1, 0, BlockType.red, true, '#FF0000')

// Future: Block with tags (assigned by LLM)
const block2 = new Block(0, 2, 0, BlockType.blue, true, '#0000FF', false, 
  ['healPlayer'], 
  { healPlayer: { amount: 5, respawnTime: 60 } }
)

// Future: Block with multiple tags
const block3 = new Block(0, 3, 0, BlockType.purple, true, '#800080', false,
  ['healPlayer', 'destructable'],
  { 
    healPlayer: { amount: 10, respawnTime: 120 },
    destructable: { health: 100 }
  }
)
```

### M5.2: Implement Space JSON Serialization
**File:** New file `frontend/src/export/serialize.ts`

**Configuration:**
- Backend API URL: Use `import.meta.env.VITE_API_URL` (default to relative `/api/export` if not set)
- Level name: Default to `"Untitled Level"` for MVP (no user input required)

**MVP Format (schemaVersion: 1):**
```json
{
  "schemaVersion": 1,
  "name": "Untitled Level",
  "blocks": [
    { "x": 0, "y": 1, "z": 0, "color": "#FF0000" }
  ]
}
```

**Future Format (schemaVersion: 2+):**
```json
{
  "schemaVersion": 2,
  "name": "My Level",
  "blocks": [
    {
      "x": 0,
      "y": 1,
      "z": 0,
      "color": "#0000FF",
      "tags": ["healPlayer"],
      "tagConfig": {
        "healPlayer": { "amount": 5, "respawnTime": 60 }
      }
    }
  ]
}
```

**Implementation:**
- Filter `customBlocks` to only `placed: true` and `!isGround`
- Map to export format based on `schemaVersion`
- Include tags/configs only if `schemaVersion >= 2` and tags exist
- Serialize `tagConfig` as JSON object (flexible structure)
- Use default level name: `"Untitled Level"` (no user input for MVP)
- POST to backend: `import.meta.env.VITE_API_URL || '/api/export'`
- Handle response: Trigger browser download of `.rbxlx` file
- Error handling: Display user-friendly error messages

**LLM Processing Flow:**
1. User creates level structure (blocks with colors)
2. User requests: "blue blocks heal 5 HP and respawn every 60s"
3. LLM processes request and assigns tags:
   - Finds all blocks with `color: "#0000FF"`
   - Adds `tags: ["healPlayer"]` to each
   - Sets `tagConfig: { healPlayer: { amount: 5, respawnTime: 60 } }`
4. Export includes tags/configs in Space JSON v2
5. Backend/LLM generates Roblox scripts that:
   - Look for blocks with `"healPlayer"` tag
   - Read config to get amount/respawn time
   - Generate appropriate script system

**Note:** Export API integration handled in [task-list.md](task-list.md) Phase 4.

### M5.3: Tag System Design for LLM Integration
**File:** Documentation (add to `_docs/` or code comments)

**Tag System Architecture:**

1. **Tag Assignment (LLM-driven):**
   - LLM receives user request: "blue blocks heal 5 HP and respawn every 60s"
   - LLM parses request and identifies:
     - Target blocks: `color === "#0000FF"`
     - Tag to assign: `"healPlayer"`
     - Config values: `{ amount: 5, respawnTime: 60 }`
   - LLM updates blocks in `customBlocks` array:
     ```typescript
     blocks.forEach(block => {
       if (block.color === '#0000FF') {
         block.addTag('healPlayer', { amount: 5, respawnTime: 60 })
       }
     })
     ```

2. **Tag Querying (Script Generation):**
   - LLM generates Roblox scripts that query blocks by tag
   - Example script pattern:
     ```lua
     -- Find all blocks with "healPlayer" tag
     for _, block in pairs(workspace:GetDescendants()) do
       if block:GetAttribute("Tag") == "healPlayer" then
         local config = block:GetAttribute("TagConfig")
         -- Use config.amount and config.respawnTime
       end
     end
     ```

3. **Tag Config Structure:**
   - Flexible JSON structure (no fixed schema)
   - Each tag can have different config shape
   - Examples:
     ```typescript
     // healPlayer config
     { amount: 5, respawnTime: 60 }
     
     // killPlayer config  
     { damage: 100 }
     
     // destructable config
     { health: 100, explosionRadius: 5 }
     ```

4. **Common Tag Patterns:**
   - `"static"` - Block cannot be moved (no config needed)
   - `"moveable"` - Block can be pushed/moved (config: `{ mass: number }`)
   - `"destructable"` - Block can be destroyed (config: `{ health: number }`)
   - `"killPlayer"` - Kills player on touch (config: `{ damage?: number }`)
   - `"damagePlayer"` - Damages player on touch (config: `{ damage: number }`)
   - `"healPlayer"` - Heals player on touch (config: `{ amount: number, respawnTime?: number }`)
   - `"checkpoint"` - Respawn point (config: `{ name?: string }`)
   - `"spawn"` - Player spawn location (no config)
   - `"finish"` - Level completion trigger (no config)

5. **Multiple Tags Example:**
   ```typescript
   // Block that heals AND can be destroyed
   block.tags = ['healPlayer', 'destructable']
   block.tagConfig = {
     healPlayer: { amount: 10, respawnTime: 120 },
     destructable: { health: 50 }
   }
   ```

**Key Design Principles:**
- **No color-tag coupling**: Any color can have any tag(s)
- **Dynamic assignment**: LLM assigns tags based on user request, not predefined rules
- **Flexible configs**: Each tag defines its own config structure
- **Multiple tags**: Blocks can have multiple independent mechanics
- **LLM-friendly**: Simple structure easy for LLM to parse and generate

---

## Task Summary

| Phase | Focus | Tasks |
|-------|-------|-------|
| M1 | Camera/Controls | 7 |
| M2 | World Init | 5 |
| M3 | Block System | 5 |
| M4 | UI Adaptation | 4 |
| M5 | Data Structure | 2 |
| **Total** | | **23** |

---

## Dependencies

| This Task | Depends On |
|-----------|------------|
| M2.4 (Ground Plane) | M3.1, M3.2 (Color Materials) |
| M2.5 (Ground Protection) | M2.4 (Ground Plane) |
| M4.1 (Color Hotbar) | M3.1 (Color Types) |
| M4.2 (Block Counter) | M3.5 (Block Limit) |
| M5.1 (Block Storage) | M3.1 (Color Types) |
| M5.2 (Serialization) | M5.1 (Block Storage) |

---

## Implementation Order

Recommended sequence accounting for dependencies:

1. M3.1, M3.2 (Color materials foundation)
2. M1.1 through M1.7 (Camera/controls)
3. M2.1, M2.2, M2.3 (Disable unwanted features)
4. M2.4, M3.3 (Ground plane with colors)
5. M2.5, M3.4, M3.5 (Protections and limits)
6. M4.1, M4.2, M4.3, M4.4 (UI updates)
7. M5.1, M5.2 (Export data structure)

---

## Related Documents

- [prd.md](prd.md) - Product requirements
- [architecture.md](architecture.md) - System architecture
- [task-list.md](task-list.md) - Full MVP task list
- [03_space_json_schema.md](03_space_json_schema.md) - Export format specification
