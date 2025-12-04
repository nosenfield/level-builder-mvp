# Active Context: mvp

**Last Updated**: December 2024

## Current Focus

### What We're Working On Right Now
**Phase M4: UI Adaptation** - ✅ COMPLETE

Successfully completed all Phase M4 tasks:
- M4.1: Updated hotbar to color palette (color squares with number labels)
- M4.2: Added block counter display (N / 10,000) positioned under FPS counter
- M4.3: Updated controls help text (renamed Guide to Controls, updated all control instructions)
- M4.4: Removed unused UI elements (footer, GitHub link; preserved save/load code hidden for production)
- M4.5: Added Export button to Escape menu (placeholder implementation ready for Phase 4)

Additional improvements:
- Implemented 10-second interval auto-save system
- Auto-save on exit and page unload
- Camera quaternion save/restore for complete view restoration
- Fixed load game to properly render saved blocks (renderCustomBlocks method)
- Restored Load Game button with enable/disable based on saved data
- UI cleanup (removed footer, GitHub link, improved menu structure)

### Current Phase
**MVP Phase 1: Frontend Migration** - Phase M4 Complete, Phase M5 next

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
1. **Completed Phase M4: UI Adaptation with Auto-Save System** (December 4, 2024)
   - Implemented 10-second interval auto-save and auto-save on exit/page unload
   - Added camera quaternion save/restore for complete view restoration
   - Fixed load game functionality (renderCustomBlocks method to properly render saved blocks)
   - Added block counter display (N / 10,000) positioned under FPS counter with warning state
   - Updated controls help text (renamed Guide to Controls, updated all instructions)
   - Restored Load Game button with smart enable/disable based on saved data
   - Added Export button to Escape menu (placeholder for Phase 4)
   - UI cleanup: Removed footer and GitHub link, improved menu structure
   - Preserved save/load code (hidden for MVP, ready for production)

2. **Completed Phase M3: Block System with Performance Optimizations** (December 3, 2024)
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

3. **Completed Phase M2: World Initialization** (December 2024)
   - Disabled procedural terrain and cloud generation
   - Disabled audio system (silent operation)
   - Created 100x100 grey ground plane at Y=0 with yellow marker grid (11x11 intersections)
   - Added isGround property to Block class
   - Implemented ground block protection (non-removable)
   - Removed generateAdjacentBlocks calls
   - Performance optimizations: blocksMap for O(1) lookups, InstancedMesh for yellow markers

---

## Next Steps

### Immediate (This Session)
- [x] Complete Phase M4: UI Adaptation (all 5 tasks)
- [x] Implement auto-save system (10s interval + on exit)
- [x] Implement camera view restoration (quaternion save/restore)
- [x] Fix load game functionality
- [ ] Begin Phase M5: Data Structure enhancements

### Near-Term (This Week)
- [x] Complete Phase M1: Camera and Controls (7 tasks)
- [x] Complete Phase M2: World Initialization (5 tasks)
- [x] Complete Phase M3: Block System (5 tasks + optimizations)
- [x] Complete Phase M4: UI Adaptation (5 tasks)
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
- Camera position: Initial (40, 5, 40), new level start (40, 5, 40), both looking at (50, 0, 50)
- BlockType: New color enum values
- Ground marking: isGround flag
- Export: Escape menu button
- API URL: import.meta.env.VITE_API_URL
- Level name: "Untitled Level" default
- Auto-save: 10-second interval (recommended over per-block save for performance)
- Camera restore: Save quaternion for complete view restoration (not just position)
- Save/load: Preserved code hidden for MVP, ready for production backend integration
