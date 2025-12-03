# MVP Implementation Task List

## Overview

Checklist for implementing the Roblox Level Builder MVP. Tasks are ordered by dependency. Complete each section before moving to the next.

---

## Phase 1: Project Setup

- [ ] **1.1** Create GitHub repository with monorepo structure (`/frontend`, `/backend`)
- [ ] **1.2** Initialize frontend: `npm create vite@latest frontend -- --template vanilla-ts`
- [ ] **1.3** Initialize backend: `cargo new backend`
- [ ] **1.4** Install frontend dependencies: `three`, `@types/three`
- [ ] **1.5** Install backend dependencies: `rbx_dom_weak`, `rbx_xml`, `rbx_types`, `rbx_reflection_database`, `axum`, `tokio`, `serde`, `serde_json`
- [ ] **1.6** Configure TypeScript strict mode
- [ ] **1.7** Set up development scripts in `package.json` and `Cargo.toml`
- [ ] **1.8** Create `.gitignore` for both projects

---

## Phase 2: Frontend — Core Editor

### 2A: Scene Setup
- [ ] **2.1** Create three.js scene with WebGL renderer
- [ ] **2.2** Add perspective camera with default position
- [ ] **2.3** Add ambient and directional lighting
- [ ] **2.4** Add grid helper at Y=0
- [ ] **2.5** Implement orbit controls (mouse drag to rotate)
- [ ] **2.6** Implement pan controls (shift+drag or middle mouse)
- [ ] **2.7** Implement zoom controls (scroll wheel)

### 2B: Block System
- [ ] **2.8** Create block data structure: `Map<string, Block>` keyed by `"x,y,z"`
- [ ] **2.9** Implement raycasting for mouse-to-world position
- [ ] **2.10** Implement block placement on left-click
- [ ] **2.11** Implement block removal on right-click
- [ ] **2.12** Implement grid snapping (round to integers)
- [ ] **2.13** Add block preview/highlight on hover
- [ ] **2.14** Enforce coordinate bounds (-500 to 500 X/Z, 0 to 500 Y)
- [ ] **2.15** Enforce 10,000 block limit

### 2C: Color System
- [ ] **2.16** Create color palette component with 10 preset colors
- [ ] **2.17** Implement color selection state
- [ ] **2.18** Apply selected color to new blocks
- [ ] **2.19** Display selected color indicator

---

## Phase 3: Frontend — UI Components

- [ ] **3.1** Create app layout (header, viewport, sidebar, footer)
- [ ] **3.2** Add logo/title in header
- [ ] **3.3** Add Export button in header
- [ ] **3.4** Add color palette in sidebar
- [ ] **3.5** Add block counter in footer (`N / 10,000`)
- [ ] **3.6** Add controls help text or tooltip
- [ ] **3.7** Add loading overlay component (hidden by default)
- [ ] **3.8** Add error message component (hidden by default)
- [ ] **3.9** Add "Desktop required" message for mobile viewports

---

## Phase 4: Frontend — Export

- [ ] **4.1** Implement Space JSON serialization function
- [ ] **4.2** Add schema version field (`schemaVersion: 1`)
- [ ] **4.3** Implement export button click handler
- [ ] **4.4** Show loading overlay on export start
- [ ] **4.5** POST Space JSON to backend `/api/export`
- [ ] **4.6** Handle successful response: trigger `.rbxlx` download
- [ ] **4.7** Handle error response: display error message
- [ ] **4.8** Hide loading overlay on completion

---

## Phase 5: Backend — API Setup

- [ ] **5.1** Create Axum HTTP server with CORS enabled
- [ ] **5.2** Define `/api/export` POST endpoint
- [ ] **5.3** Define Space JSON request struct with serde
- [ ] **5.4** Define Block struct with serde
- [ ] **5.5** Implement JSON deserialization
- [ ] **5.6** Return `.rbxlx` as `application/octet-stream` with `Content-Disposition`

---

## Phase 6: Backend — Validation

- [ ] **6.1** Validate schema version equals 1
- [ ] **6.2** Validate block count <= 10,000
- [ ] **6.3** Validate coordinate bounds for all blocks
- [ ] **6.4** Validate color format (hex pattern)
- [ ] **6.5** Validate no duplicate positions
- [ ] **6.6** Return structured error JSON for validation failures

---

## Phase 7: Backend — RBXLX Generation

### 7A: DataModel Structure
- [ ] **7.1** Create WeakDom with DataModel root
- [ ] **7.2** Add Workspace service
- [ ] **7.3** Add Terrain instance to Workspace
- [ ] **7.4** Add Camera instance to Workspace
- [ ] **7.5** Add Players service
- [ ] **7.6** Add Lighting service
- [ ] **7.7** Add ReplicatedStorage service
- [ ] **7.8** Add StarterGui service
- [ ] **7.9** Add StarterPack service
- [ ] **7.10** Add StarterPlayer service with child containers

### 7B: Level Geometry
- [ ] **7.11** Implement hex-to-Color3 conversion
- [ ] **7.12** Implement block-to-Part generation
- [ ] **7.13** Set Part properties: Name, Anchored, CanCollide, Size, CFrame, Color, Material
- [ ] **7.14** Generate unique referents for each Part
- [ ] **7.15** Add all Parts as children of Workspace

### 7C: Spawn Location
- [ ] **7.16** Calculate spawn position (center of level, above highest block)
- [ ] **7.17** Create SpawnLocation instance
- [ ] **7.18** Set SpawnLocation properties: Anchored, Neutral, Duration, Size, CFrame
- [ ] **7.19** Add SpawnLocation as child of Workspace

### 7D: Serialization
- [ ] **7.20** Serialize WeakDom to XML using `rbx_xml::to_writer_default`
- [ ] **7.21** Return XML bytes as response body

---

## Phase 8: Integration Testing

- [ ] **8.1** Test empty level export (0 blocks)
- [ ] **8.2** Test single block export
- [ ] **8.3** Test multi-block export (100 blocks)
- [ ] **8.4** Test maximum block export (10,000 blocks)
- [ ] **8.5** Test various colors
- [ ] **8.6** Test coordinate edge cases (bounds limits)
- [ ] **8.7** Open generated files in Roblox Studio — verify no errors
- [ ] **8.8** Play test in Studio — verify spawn works
- [ ] **8.9** Play test in Studio — verify block collision works
- [ ] **8.10** Verify block positions match editor
- [ ] **8.11** Verify block colors match editor

---

## Phase 9: Error Handling

- [ ] **9.1** Test invalid schema version rejection
- [ ] **9.2** Test block limit exceeded rejection
- [ ] **9.3** Test out-of-bounds coordinate rejection
- [ ] **9.4** Test invalid color format rejection
- [ ] **9.5** Test duplicate position rejection
- [ ] **9.6** Test malformed JSON rejection
- [ ] **9.7** Verify frontend displays error messages correctly

---

## Phase 10: Deployment

### 10A: Backend Deployment
- [ ] **10.1** Create Railway account and project
- [ ] **10.2** Configure Railway for Rust deployment
- [ ] **10.3** Set environment variables (if any)
- [ ] **10.4** Deploy backend to Railway
- [ ] **10.5** Verify backend health endpoint responds
- [ ] **10.6** Note backend URL for frontend config

### 10B: Frontend Deployment
- [ ] **10.7** Create Cloudflare Pages account and project
- [ ] **10.8** Update frontend API URL to production backend
- [ ] **10.9** Build frontend: `npm run build`
- [ ] **10.10** Deploy frontend to Cloudflare Pages
- [ ] **10.11** Configure custom domain (optional)

### 10C: Production Testing
- [ ] **10.12** Test full export flow on production
- [ ] **10.13** Test from multiple browsers
- [ ] **10.14** Verify CORS is working
- [ ] **10.15** Monitor for errors

---

## Phase 11: Polish (If Time Permits)

- [ ] **11.1** Add favicon
- [ ] **11.2** Add meta tags for SEO/sharing
- [ ] **11.3** Improve loading states
- [ ] **11.4** Add keyboard shortcut hints
- [ ] **11.5** Performance optimization for large levels
- [ ] **11.6** Add simple analytics (privacy-respecting)

---

## Completion Checklist

- [ ] User can place colored blocks in 3D editor
- [ ] User can remove blocks
- [ ] User can navigate camera (orbit, pan, zoom)
- [ ] User can select colors
- [ ] User can export level
- [ ] Exported file opens in Roblox Studio
- [ ] Player spawns correctly in exported level
- [ ] Player can walk on blocks
- [ ] Block positions and colors are accurate
- [ ] Error states are handled gracefully

---

## Quick Reference

| Phase | Focus | Est. Tasks |
|-------|-------|------------|
| 1 | Setup | 8 |
| 2 | Editor Core | 19 |
| 3 | UI | 9 |
| 4 | Export (FE) | 8 |
| 5 | API Setup | 6 |
| 6 | Validation | 6 |
| 7 | RBXLX Gen | 21 |
| 8 | Integration | 11 |
| 9 | Error Handling | 7 |
| 10 | Deployment | 15 |
| 11 | Polish | 6 |
| **Total** | | **116** |
