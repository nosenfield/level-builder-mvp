# Active Context: mvp

**Last Updated**: December 2024

## Current Focus

### What We're Working On Right Now
**Phase M1: Camera and Controls Implementation** - ✅ COMPLETE

Successfully implemented all 7 tasks in Phase M1:
- M1.1: Enabled flying mode as default
- M1.2: Removed physics and collisions
- M1.3: Remapped Q/E altitude controls
- M1.4: Updated forward/backward movement to include vertical component
- M1.5: Implemented speed toggle (Space key)
- M1.6: Swapped click controls (left=place, right=remove)
- M1.7: Set initial camera position (40, 10, 40) looking at (50, 0, 50)

### Current Phase
**MVP Phase 1: Frontend Migration** - Phase M1 Complete, ready for Phase M2

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
1. **Completed Phase M1: Camera and Controls** (December 2024)
   - Implemented all 7 camera/control tasks
   - Removed collision detection and physics
   - Enhanced forward/backward movement with vertical component
   - Swapped click controls (left=place, right=remove)
   - Added speed toggle functionality
   - Set initial camera position

2. **Created comprehensive migration task list** (December 2024)
   - Documented 23 tasks across 5 phases (M1-M5)
   - Resolved all outstanding implementation questions
   - Added LLM tag system design for future game mechanics

3. **Clarified implementation decisions** (December 2024)
   - Camera controls: Keep PointerLockControls with vertical movement enhancement
   - BlockType system: Create new color enum values
   - Ground marking: Use isGround flag
   - Export button: Add to Escape menu

---

## Next Steps

### Immediate (This Session)
- [x] Review PRD and architecture documents
- [x] Review frontend codebase structure
- [x] Create migration task list
- [x] Resolve implementation questions
- [x] Update memory bank
- [x] Complete Phase M1 implementation (Camera and Controls)
- [ ] Begin Phase M2 implementation (World Initialization)

### Near-Term (This Week)
- [x] Complete Phase M1: Camera and Controls (7 tasks)
- [ ] Complete Phase M2: World Initialization (5 tasks)
- [ ] Complete Phase M3: Block System (5 tasks)
- [ ] Begin Phase M4: UI Adaptation

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
