# Product Context: mvp

**Last Updated**: December 2024

## Why This Project Exists

### Problem Statement

Creating game environments in Roblox requires significant technical knowledge and setup:
- Installing Roblox Studio (1+ GB download)
- Learning Studio's complex interface
- Understanding Roblox's part system, properties, and hierarchy
- Manually configuring spawn points and game settings

This creates barriers for young creators, educators, hobbyists, and teams prototyping level layouts.

### User Pain Points
1. **Installation barrier**: Roblox Studio is large and requires installation
2. **Learning curve**: Studio interface is complex and intimidating
3. **Time investment**: Setting up basic levels takes too long
4. **Technical knowledge**: Requires understanding Roblox-specific concepts
5. **Commitment**: Must install software just to experiment

### Our Solution

A browser-based voxel editor that:
- Works entirely in the browser (no installation)
- Uses familiar Minecraft-style building interface
- Exports directly to Roblox Studio format
- Requires zero Roblox knowledge to use
- Enables rapid prototyping and experimentation

---

## Target Users

### Primary User Persona: Aspiring Creator
- **Age**: 10-16 years
- **Role**: Young game creator
- **Goals**: Create and share game levels with friends
- **Frustrations**: Roblox Studio is overwhelming and complex
- **Tech Savviness**: Low to moderate (familiar with Minecraft-style building)
- **Behavior**: Wants to build quickly, familiar with voxel editors

### Secondary User Persona: Educator
- **Age**: 25-45 years
- **Role**: Teacher/instructor
- **Goals**: Teach game design fundamentals without setup overhead
- **Frustrations**: Setup time for student projects, need accessible tools
- **Tech Savviness**: Moderate
- **Behavior**: Needs quick, accessible tools for classroom use

### Tertiary User Persona: Prototyper
- **Age**: 18-30 years
- **Role**: Game developer/designer
- **Goals**: Rapidly prototype level layouts before full implementation
- **Frustrations**: Studio is slow for simple layouts
- **Tech Savviness**: High
- **Behavior**: Values speed over features, wants to iterate quickly

---

## Key User Flows

### Flow 1: Create and Export Level
1. User opens web app in browser
2. User sees empty 3D grid space
3. User selects color from palette (1-9 keys or click)
4. User places blocks by left-clicking in 3D space
5. User removes blocks by right-clicking
6. User navigates camera (WASD + Q/E + mouse)
7. User clicks "Export" button in Escape menu
8. System generates Space JSON and sends to backend
9. Backend returns `.rbxlx` file
10. Browser downloads file
11. User opens file in Roblox Studio
12. Result: Playable level with working spawn and movement

### Flow 2: Quick Level Prototype
1. User places blocks rapidly using keyboard shortcuts
2. User adjusts camera to view from different angles
3. User exports and tests in Roblox Studio
4. User iterates based on playtest feedback
5. Result: Rapid iteration cycle without Studio overhead

---

## Product Goals

### Short-term (MVP)
- Validate core value proposition: browser-based Roblox level creation
- Enable non-technical users to create playable levels
- Establish foundation for future features (game mechanics, LLM assistance)
- Achieve > 95% export success rate
- Enable < 5 minute time-to-first-export

### Long-term (Future)
- LLM-assisted level design (v3)
- Custom game mechanics (kill zones, checkpoints) (v2)
- Collaborative editing (v3)
- Direct Roblox publishing (v3)
- Mobile support (v2)
- Cloud save/load (v2)

---

## Success Metrics

### User Engagement
- Daily Active Users: 100+ (post-launch target)
- Average blocks per level: 50+
- Return users: 20%+ (repeat visits within 7 days)

### Business Impact
- Export success rate: > 95%
- Time to first export: < 5 minutes
- User satisfaction: Positive feedback (qualitative)

### Technical Performance
- Output validity: 100% (files open without Studio errors)
- Editor frame rate: >= 30 FPS with 5,000 blocks
- Export time: < 5 seconds for 10,000 blocks
