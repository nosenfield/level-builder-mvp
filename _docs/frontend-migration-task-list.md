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

### M1.4: Update Forward/Backward Movement
**File:** `frontend/src/control/index.ts`

- Modify `W`/`S` movement to follow camera projection axis
- Forward moves in direction camera is facing (including vertical component)
- Backward moves opposite to camera facing direction

### M1.5: Implement Speed Toggle
**File:** `frontend/src/control/index.ts`

- Base speed: walking speed (4 units/sec)
- `Space` key toggles to flying speed (12 units/sec)
- Speed applies to all directional movement (WASD + Q/E)

### M1.6: Swap Click Controls
**File:** `frontend/src/control/index.ts`

- Left-click (button 0): Place block at target position
- Right-click (button 2): Remove block at target position
- Reverse current `mousedownHandler` logic

### M1.7: Set Initial Camera Position
**File:** `frontend/src/core/index.ts` or `frontend/src/control/index.ts`

- Position: Above center of ground plane (x=50, z=50)
- Height: Elevated to see full plane
- Angle: Isometric-style view looking down toward center

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
**File:** `frontend/src/terrain/mesh/materials.ts` (modify or new file)

Create 10 color blocks per PRD Appendix B:

| Index | Name | Hex |
|-------|------|-----|
| 0 | Red | #FF0000 |
| 1 | Green | #00FF00 |
| 2 | Blue | #0000FF |
| 3 | Yellow | #FFFF00 |
| 4 | Orange | #FFA500 |
| 5 | Purple | #800080 |
| 6 | Gray | #808080 |
| 7 | White | #FFFFFF |
| 8 | Black | #000000 |
| 9 | Brown | #8B4513 |

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

### M3.5: Implement Block Limit Enforcement
**File:** `frontend/src/control/index.ts` or `frontend/src/terrain/index.ts`

- Track user-placed block count (ground plane excluded)
- Maximum: 10,000 blocks
- Prevent placement when limit reached
- Decrement count on block removal

---

## Phase M4: UI Adaptation

### M4.1: Update Hotbar to Color Palette
**File:** `frontend/src/ui/bag/index.ts`

- Replace texture icons with solid color squares
- Display 10 color options
- Maintain selection highlight functionality
- Update `holdingBlocks` array to reference color indices

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

---

## Phase M5: Data Structure

### M5.1: Implement Block Storage for Export
**File:** New file `frontend/src/terrain/blockStore.ts`

- Store user-placed blocks in exportable format
- Structure: `Map<string, {x, y, z, color}>` keyed by `"x,y,z"`
- Exclude ground plane blocks
- Update on place/remove operations

### M5.2: Implement Space JSON Serialization
**File:** New file `frontend/src/export/serialize.ts`

Output format per [03_space_json_schema.md](03_space_json_schema.md):

```json
{
  "schemaVersion": 1,
  "name": "My Level",
  "blocks": [
    { "x": 0, "y": 1, "z": 0, "color": "#FF0000" }
  ]
}
```

Note: Export API integration handled in [task-list.md](task-list.md) Phase 4.

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
