# Project Brief: mvp

**Version**: 1.0
**Last Updated**: December 2024

## Project Overview

### What We're Building

A web-based Roblox Level Builder MVP that enables non-technical users to create 3D block-based game levels in their browser and export them as playable Roblox Studio files (`.rbxlx` format). Users build levels using an intuitive voxel editor (forked from a three.js Minecraft clone), then download a file that opens directly in Roblox Studio with working player spawn and movement.

### Core Problem

Creating game environments in Roblox requires installing Roblox Studio (1+ GB), learning complex interfaces, understanding Roblox's part system, and manually configuring spawn points. This creates barriers for young creators, educators, hobbyists, and teams prototyping level layouts.

### Target Users

**Primary**: Aspiring Creators (ages 10-16, low-to-moderate technical skill, familiar with Minecraft-style building)
**Secondary**: Educators (ages 25-45, moderate technical skill, need quick accessible tools)
**Tertiary**: Prototypers (ages 18-30, high technical skill, value speed over features)

### Success Criteria

- Export success rate > 95%
- Time to first export < 5 minutes for new users
- 100% output validity (files open without Studio errors)
- Daily active users 100+ (post-launch)
- Average 50+ blocks per level

---

## MVP Scope

### Must Have
- Browser-based 3D voxel editor (three.js)
- Block placement/removal (1x1x1 colored blocks)
- Color selection (10 preset colors)
- Camera controls (orbit, pan, zoom via PointerLockControls)
- Export to `.rbxlx` format (via Rust backend)
- Playable output in Roblox Studio (with SpawnLocation)
- Basic UI (viewport, color palette, export button, block counter)
- 10,000 block limit per level
- Coordinate bounds enforcement (-500 to 500 X/Z, 0 to 500 Y)

### Explicitly Out of Scope
- User accounts and authentication
- Cloud save/load functionality
- Undo/redo functionality
- Variable block sizes
- Custom materials
- Game mechanics (kill zones, checkpoints)
- LLM assistance
- Mobile support
- Multiplayer editing
- Direct Roblox publishing

---

## Technical Constraints

### Performance Targets
- Editor frame rate: >= 30 FPS with 5,000 blocks
- Block placement latency: < 50ms
- Export generation time: < 5 seconds for 10,000 blocks
- Page load time: < 3 seconds on broadband

### Platform Requirements
- Desktop browsers only (Chrome 90+, Firefox 90+, Safari 15+, Edge 90+)
- Modern WebGL support required
- Users must have Roblox Studio installed (assumption)

### Dependencies
- **Frontend**: three.js (0.137.0), TypeScript (4.5.5), Vite (2.8.0)
- **Backend**: Rust, rbx-dom ecosystem, Axum/Actix-web
- **Hosting**: Cloudflare Pages (frontend), Railway (backend)
- **Output Platform**: Roblox Studio (current stable release)

---

## Project Timeline

- **MVP Target**: TBD
- **Key Milestones**:
  - Frontend migration complete: TBD
  - Backend API complete: TBD
  - Integration testing: TBD
  - Production deployment: TBD
