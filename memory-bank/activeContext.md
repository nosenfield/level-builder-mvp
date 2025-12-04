# Active Context: mvp

**Last Updated**: December 2024

## Current Focus

### What We're Working On Right Now
**Phase M3: Block System Implementation** - ✅ COMPLETE

Successfully implemented all tasks in Phase M3 with performance optimizations:
- M3.1: Defined color-based BlockType enum (10 colors: red, orange, yellow, green, blue, violet, brown, white, gray, black)
- M3.2: Created solid color materials (MeshStandardMaterial with hex colors, removed indigo)
- M3.3: Updated block rendering system (InstancedMesh per color, updated ground plane to use gray/yellow)
- M3.4: Implemented coordinate bounds enforcement (X/Z: -500 to 500, Y: 0 to 500)
- M3.5: Implemented block limit enforcement (10,000 max user-placed blocks, ground excluded)
- Performance: Reduced maxCount from ~28,724 to 20,000, set texture block factors to 0
- Performance: Added cached counter for user-placed blocks (O(1) instead of O(n))
- UI: Updated hotbar with color squares and number labels (buttons 1-0)
- UI: Extracted inline styles to CSS classes for maintainability
- Code cleanup: Removed dead code related to indigo block type

### Current Phase
**MVP Phase 1: Frontend Migration** - Phase M3 Complete, Phase M4 partially started (M4.1 done)

---

## Active Decisions

### Decision 1: BlockType System
**What**: Create new BlockType enum values for colors (red, green, blue, etc.) instead of replacing the system entirely.
**Reasoning**: Keeps existing code structure intact, minimal refactoring needed, maintains compatibility with current rendering system.
**Impact**: `holdingBlock` and `holdingBlocks` arrays will reference new color BlockTypes.

### Decision 2: Ground Plane Marking
**What**: Use `isGround: boolean` flag on Block class for ground protection.
**Reasoning**: Clean separation of concerns, easy to filter for export, aligns with Block class enhancement design.
**Impact**: M2.5 ground protection will check `isGround` flag before allowing removal.

### Decision 3: Camera Controls
**What**: Keep PointerLockControls (FPS-style) but modify forward/backward movement to include vertical component.
**Reasoning**: Maintains familiar FPS controls, only needs enhancement for vertical movement, no major refactor needed.
**Impact**: Forward/backward movement will follow camera look direction including pitch angle.

### Decision 4: Tag System for Future Game Mechanics
**What**: Add `tags?: string[]` and `tagConfig?: Record<string, any>` to Block class for LLM-driven game mechanics.
**Reasoning**: Flexible, LLM-friendly structure that supports dynamic tag assignment and configurable mechanics. No color-tag coupling.
**Impact**: Future LLM can assign tags like "killPlayer", "healPlayer" with configs (damage amount, respawn time, etc.).

---

## Recent Changes

### Last 3 Significant Changes
1. **Completed Phase M3: Block System with Performance Optimizations** (December 3, 2024)
   - Implemented 10 color block types (removed indigo, moved violet to slot 6, added brown to slot 7)
   - Created solid color materials (MeshStandardMaterial with hex colors)
   - Updated InstancedMesh rendering system for color blocks
   - Implemented coordinate bounds enforcement (X/Z: -500 to 500, Y: 0 to 500)
   - Implemented block limit enforcement (10,000 max user-placed blocks)
   - Performance: Reduced maxCount from ~28,724 to 20,000 (procedural generation disabled)
   - Performance: Set texture block allocation factors to 0 (unused blocks)
   - Performance: Added cached counter for user-placed blocks (O(1) instead of O(n) filter)
   - UI: Updated hotbar with color squares and number labels (buttons 1-0)
   - UI: Extracted inline styles to CSS classes for better maintainability
   - Code cleanup: Removed dead code related to indigo block type
   - Updated ground plane to use BlockType.gray and BlockType.yellow

2. **Completed Phase M2: World Initialization** (December 2024)
   - Disabled procedural terrain and cloud generation
   - Disabled audio system (silent operation)
   - Created 100x100 grey ground plane at Y=0 with yellow marker grid (11x11 intersections)
   - Added isGround property to Block class
   - Implemented ground block protection (non-removable)
   - Removed generateAdjacentBlocks calls
   - Performance optimizations: blocksMap for O(1) lookups, InstancedMesh for yellow markers

3. **Refactored Highlight System** (December 2024)
   - Simplified block highlight by raycasting directly against rendered blocks (terrain.blocks[])
   - Removed separate instanceMesh for raycasting (eliminated range mismatch issues)
   - Created shared constants.ts with BLOCK_INTERACTION_RANGE (50 units)
   - Removed dead code (BLOCK_HIGHLIGHT_RANGE)
   - Fixed inconsistent highlighting - now consistent with block placement/removal range
   - Increased block placement distance to 50 units
   - Decreased hold-to-place interval to 0.25s (250ms)

---

## Next Steps

### Immediate (This Session)
- [x] Complete Phase M3: Block System with performance optimizations
- [x] Update hotbar UI with color squares (M4.1 partially done)
- [ ] Complete remaining Phase M4 tasks (block counter, controls help, export button)
- [ ] Begin Phase M5: Data Structure enhancements

### Near-Term (This Week)
- [x] Complete Phase M1: Camera and Controls (7 tasks)
- [x] Complete Phase M2: World Initialization (5 tasks)
- [x] Complete Phase M3: Block System (5 tasks + optimizations)
- [ ] Complete Phase M4: UI Adaptation (M4.1 done, 4 remaining)
- [ ] Begin Phase M5: Data Structure (3 tasks)

---

## Blockers / Open Questions

### Current Blockers
None - All implementation questions resolved, ready to begin coding.

### Questions to Resolve
None - All decisions documented in migration task list.

---

## Key Files Currently Modified

- `_docs/frontend-migration-task-list.md` - Complete migration plan (543 lines)
- `memory-bank/activeContext.md` - This file (being updated)
- `memory-bank/progress.md` - Progress tracking (to be updated)
- `memory-bank/systemPatterns.md` - Architecture patterns (to be updated)
- `memory-bank/techContext.md` - Tech stack details (to be updated)

## Implementation Readiness

**Status**: ✅ Ready to begin implementation

**Migration Task List**: Complete with 23 tasks across 5 phases
- Phase M1: Camera/Controls (7 tasks)
- Phase M2: World Init (5 tasks)
- Phase M3: Block System (5 tasks)
- Phase M4: UI Adaptation (5 tasks)
- Phase M5: Data Structure (3 tasks)

**Key Decisions Documented**:
- Camera position: (40, 10, 40) looking at (50, 0, 50)
- BlockType: New color enum values
- Ground marking: isGround flag
- Export: Escape menu button
- API URL: import.meta.env.VITE_API_URL
- Level name: "Untitled Level" default
