# Active Context: mvp

**Last Updated**: December 2024

## Current Focus

### What We're Working On Right Now
**Phase 10: Deployment** - 🔄 IN PROGRESS

**Current Phase**: Phase 10: Deployment (Code Complete, Manual Deployment Pending)
**Next**: Complete manual deployment steps (Railway + Cloudflare Pages), then Phase 9: Error Handling

**Test Status**: 46 tests passing (26 validation + 9 RBXLX + 6 Phase 8 integration + 1 integration + 4 baseplate/spawn platform)

---

## Active Decisions

### Tag System for Future Game Mechanics
**What**: Add `tags?: string[]` and `tagConfig?: Record<string, any>` to Block class for LLM-driven game mechanics.
**Reasoning**: Flexible, LLM-friendly structure that supports dynamic tag assignment and configurable mechanics. No color-tag coupling.
**Impact**: Future LLM can assign tags like "killPlayer", "healPlayer" with configs (damage amount, respawn time, etc.).
**Status**: Implemented in Block class, ready for future LLM integration

---

## Recent Changes

### Last 3 Significant Changes
1. **Phase 10: Deployment Code Complete** (December 2024)
   - Added `/health` and `/api/health` endpoints to backend for Railway health checks
   - Created comprehensive deployment documentation (`_docs/deployment.md`)
   - Backend ready for Railway deployment (health endpoint, PORT env var support)
   - Frontend ready for Cloudflare Pages deployment (VITE_API_URL env var support)
   - Manual deployment steps documented (Railway + Cloudflare Pages)

2. **Added Baseplate and Spawn Platform** (December 2024)
   - Baseplate Part (200x16x200 studs) at (0, -8, 0) matching ground plane scaled 2x
   - SpawnLocation fixed at (0, 0.5, 0) with Decal child
   - All 46 tests passing

3. **Completed Phase 8: Integration Testing** (December 2024)
   - Fixed Parts visibility: Changed XML serialization to pass `dom.root().children()` instead of DataModel root
   - Fixed Lighting Technology warning: Added Technology=3 (ShadowMap)
   - Fixed duplicate Camera: Removed custom Camera (Roblox Studio creates its own)
   - Simplified Part properties to minimal set (Name, CFrame, Size, Color, Anchored)
   - All 11 Phase 8 tasks complete (6 automated + 5 manual tests)

---

## Next Steps

### Immediate (This Session)
- [ ] Complete Phase 10: Manual Deployment Steps
  - Deploy backend to Railway (follow `_docs/deployment.md`)
  - Deploy frontend to Cloudflare Pages (follow `_docs/deployment.md`)
  - Test production deployment end-to-end
  - Verify health endpoint, export flow, CORS

### Near-Term (This Week)
- [ ] Complete Phase 10: Deployment (manual steps)
- [ ] Begin Phase 9: Error Handling (7 tasks)
  - Test invalid schema version, block limit, out-of-bounds coordinates
  - Test invalid color format, duplicate positions, malformed JSON
  - Verify frontend displays error messages correctly

---

## Blockers / Open Questions

### Current Blockers
None - All implementation questions resolved, ready to begin coding.

### Questions to Resolve
None - All decisions documented in migration task list.

---

## Key Files

**Backend**:
- `backend/src/main.rs` - Axum HTTP server with `/api/export` and `/health` endpoints
- `backend/src/rbxlx.rs` - RBXLX generation (includes Baseplate and SpawnLocation)
- `backend/src/validation.rs` - Validation module with tests
- `backend/src/models.rs` - Space JSON structs (SpaceJSON, Block)

**Deployment**:
- `_docs/deployment.md` - Comprehensive deployment guide (Railway + Cloudflare Pages)

**Frontend**:
- `frontend/src/ui/index.ts` - Export handler, loading overlay, error messages
- `frontend/src/export/serialize.ts` - Space JSON serialization

## Implementation Status

**Current**: 🔄 Phase 10: Deployment (Code Complete, Manual Deployment Pending)

**Completed Phases**: M1-M5 (frontend migration), Phases 4-8 (backend development), Phase 10 code (deployment prep)
**All core functionality working**: Exported `.rbxlx` files open correctly in Roblox Studio
**Deployment Ready**: Backend and frontend code ready for production deployment (Railway + Cloudflare Pages)
