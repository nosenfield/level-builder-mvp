# Active Context: mvp

**Last Updated**: December 2024

## Current Focus

### What We're Working On Right Now
**Phase 7: Backend RBXLX Generation** - ✅ COMPLETE

Successfully completed all Phase 7 tasks:
- 7.1-7.10: Created complete DataModel structure with all required services
- 7.11: Implemented hex-to-Color3 conversion (supports #RRGGBB and #RGB)
- 7.12-7.15: Implemented block-to-Part generation with all required properties
- 7.16-7.19: Implemented spawn location calculation and creation
- 7.20-7.21: Implemented XML serialization using rbx_xml

Additional improvements:
- Created RBXLX generation module (backend/src/rbxlx.rs) with comprehensive functions
- Added rbx_dom_weak, rbx_xml, and rbx_types dependencies
- Implemented spawn position calculation (center X/Z, above highest block)
- All Parts have correct properties (Anchored, CanCollide, Size, CFrame, Color, Material)
- All required services created (Workspace, Players, Lighting, ReplicatedStorage, StarterGui, StarterPack, StarterPlayer)
- Workspace contains Terrain, Camera, SpawnLocation, and all level Parts
- 9 unit tests covering all RBXLX generation scenarios (all passing)
- Total: 36 tests passing (26 validation + 9 RBXLX + 1 integration)

### Current Phase
**MVP Backend Development** - Phase 7 Complete ✅
**Next**: Phase 8: Integration Testing (from task-list.md)

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
1. **Completed Phase 7: Backend RBXLX Generation** (December 4, 2024)
   - Created RBXLX generation module (backend/src/rbxlx.rs) with complete DataModel generation
   - Added rbx_dom_weak v4.0, rbx_xml v2.0, and rbx_types v3.0 dependencies
   - Implemented hex-to-Color3 conversion (supports #RRGGBB and #RGB formats)
   - Implemented block-to-Part generation with all required properties
   - Implemented spawn location calculation (center X/Z, above highest block)
   - Created complete DataModel structure with all required services
   - Workspace contains Terrain, Camera, SpawnLocation, and all level Parts
   - Updated export_handler to use generate_rbxlx() instead of placeholder
   - Added 9 unit tests covering RBXLX generation (all passing)
   - Total test count: 36 tests (all passing)

2. **Completed Phase 6: Backend Validation** (December 4, 2024)
   - Created validation module (backend/src/validation.rs) with comprehensive validation functions
   - Implemented schema version validation (must be 1)
   - Implemented block count validation (<= 10,000)
   - Implemented coordinate bounds validation (X/Z: -500 to 500, Y: 0 to 500)
   - Implemented color format validation (hex pattern: #RRGGBB or #RGB)
   - Implemented duplicate position detection using HashSet
   - Updated export_handler to use validate_space_json() function
   - Added 26 unit tests covering all validation scenarios (all passing)
   - Error messages are user-friendly with specific details (block index, coordinates)

2. **Completed Phase 5: Backend API Setup** (December 4, 2024)
   - Initialized Rust backend project with Cargo
   - Created Axum HTTP server with CORS middleware (allows all origins for MVP)
   - Implemented `/api/export` POST endpoint
   - Defined Space JSON structs with serde (SpaceJSON, Block)
   - Implemented JSON deserialization with automatic error handling
   - Added schema version validation (only version 1 supported)
   - Returns placeholder `.rbxlx` file with correct headers (application/octet-stream, Content-Disposition)
   - Error handling returns structured JSON errors (error/message format)
   - Server configurable via PORT environment variable (default: 3000)
   - Created backend documentation (README.md, .gitignore)

2. **Completed Phase 4: Frontend Export Integration** (December 4, 2024)
   - Implemented export button click handler with full export flow
   - Created loading overlay component (shows "Exporting level..." during export)
   - Created error message component with close button (red styling for errors)
   - Implemented POST request to backend `/api/export` endpoint
   - Added blob download handling (triggers `.rbxlx` file download on success)
   - Added comprehensive error handling (network errors, HTTP errors, parsing errors)
   - Integrated with existing serializeToSpaceJSON function (schemaVersion 1)
   - Loading overlay shows/hides correctly on success and error
   - Error messages are user-friendly and dismissible

3. **Completed Phase M5: Data Structure Enhancements** (December 4, 2024)
   - Enhanced Block class with color property (required, hex format) for Space JSON export
   - Added tags and tagConfig properties for future LLM-driven game mechanics
   - Added 5 helper methods (hasTag, addTag, removeTag, getTagConfig, setTagConfig)
   - Created blockTypeToHex() utility function to convert BlockType enum to hex color
   - Updated all Block instantiations (4 locations) to include color parameter
   - Implemented serializeToSpaceJSON() function with schemaVersion 1 and 2+ support
   - Created export module structure (frontend/src/export/serialize.ts, index.ts)
   - Added backward compatibility for loading old saves without color property
   - Created tag system design documentation (_docs/tag-system-design.md)

3. **Completed Phase M4: UI Adaptation with Auto-Save System** (December 4, 2024)
   - Implemented 10-second interval auto-save and auto-save on exit/page unload
   - Added camera quaternion save/restore for complete view restoration
   - Fixed load game functionality (renderCustomBlocks method to properly render saved blocks)
   - Added block counter display (N / 10,000) positioned under FPS counter with warning state
   - Updated controls help text (renamed Guide to Controls, updated all instructions)
   - Restored Load Game button with smart enable/disable based on saved data
   - Added Export button to Escape menu (placeholder for Phase 4)
   - UI cleanup: Removed footer and GitHub link, improved menu structure
   - Preserved save/load code (hidden for MVP, ready for production)

4. **Completed Phase M3: Block System with Performance Optimizations** (December 3, 2024)
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
- [x] Complete Phase M5: Data Structure enhancements (all 3 tasks)
- [x] Complete Phase 4: Frontend Export Integration (all 6 tasks)
- [x] Complete Phase 5: Backend API Setup (all 6 tasks)
- [x] Complete Phase 6: Backend Validation (all 6 tasks)
- [x] Complete Phase 7: Backend RBXLX Generation (all 21 tasks)
  - 7.1-7.10: DataModel structure with all services ✅
  - 7.11: Hex-to-Color3 conversion ✅
  - 7.12-7.15: Block-to-Part generation ✅
  - 7.16-7.19: Spawn location creation ✅
  - 7.20-7.21: XML serialization ✅

### Near-Term (This Week)
- [x] Complete Phase M1: Camera and Controls (7 tasks)
- [x] Complete Phase M2: World Initialization (5 tasks)
- [x] Complete Phase M3: Block System (5 tasks + optimizations)
- [x] Complete Phase M4: UI Adaptation (5 tasks)
- [x] Complete Phase M5: Data Structure (3 tasks)
- [x] Complete Phase 4: Frontend Export Integration (6 tasks)
- [x] Complete Phase 5: Backend API Setup (6 tasks)
- [x] Complete Phase 6: Backend Validation (6 tasks)
- [x] Complete Phase 7: Backend RBXLX Generation (21 tasks)

---

## Blockers / Open Questions

### Current Blockers
None - All implementation questions resolved, ready to begin coding.

### Questions to Resolve
None - All decisions documented in migration task list.

---

## Key Files Currently Modified

- `backend/src/main.rs` - Axum HTTP server with `/api/export` endpoint (updated to use RBXLX generation)
- `backend/src/rbxlx.rs` - RBXLX generation module with DataModel creation and XML serialization
- `backend/src/validation.rs` - Validation module with all validation functions and tests
- `backend/src/models.rs` - Space JSON structs (SpaceJSON, Block) with serde
- `backend/Cargo.toml` - Rust dependencies (axum, tokio, serde, tower-http, rbx_dom_weak, rbx_xml, rbx_types)
- `backend/README.md` - Backend setup and API documentation
- `backend/.gitignore` - Rust-specific gitignore patterns
- `frontend/src/ui/index.ts` - Export handler, loading overlay, error message methods
- `frontend/src/export/serialize.ts` - Space JSON serialization function

## Implementation Readiness

**Status**: ✅ Ready for Phase 8: Integration Testing

**Migration Task List**: ✅ COMPLETE - All 23 tasks across 5 phases done
- Phase M1: Camera/Controls (7 tasks) ✅
- Phase M2: World Init (5 tasks) ✅
- Phase M3: Block System (5 tasks) ✅
- Phase M4: UI Adaptation (5 tasks) ✅
- Phase M5: Data Structure (3 tasks) ✅

**Phase 4 Status**: ✅ COMPLETE
- 4.1 ✅ Complete (via M5.2)
- 4.2 ✅ Complete (via M5.2)
- 4.3-4.8 ✅ Complete (all 6 tasks implemented)

**Phase 5 Status**: ✅ COMPLETE
- 5.1 ✅ Complete (Axum server with CORS)
- 5.2 ✅ Complete (`/api/export` POST endpoint)
- 5.3 ✅ Complete (Space JSON struct with serde)
- 5.4 ✅ Complete (Block struct with serde)
- 5.5 ✅ Complete (JSON deserialization)
- 5.6 ✅ Complete (Returns `.rbxlx` with correct headers)

**Phase 6 Status**: ✅ COMPLETE
- 6.1 ✅ Complete (Schema version validation)
- 6.2 ✅ Complete (Block count validation <= 10,000)
- 6.3 ✅ Complete (Coordinate bounds validation)
- 6.4 ✅ Complete (Color format validation)
- 6.5 ✅ Complete (Duplicate position detection)
- 6.6 ✅ Complete (Structured error JSON responses)

**Phase 7 Status**: ✅ COMPLETE
- 7.1-7.10 ✅ Complete (DataModel structure with all services)
- 7.11 ✅ Complete (Hex-to-Color3 conversion)
- 7.12-7.15 ✅ Complete (Block-to-Part generation)
- 7.16-7.19 ✅ Complete (Spawn location creation)
- 7.20-7.21 ✅ Complete (XML serialization)

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
