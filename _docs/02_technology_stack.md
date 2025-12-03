# Technology Stack

## Document Metadata
- Scope: MVP
- Version: 1.0
- Status: Active

## Stack Summary

| Layer | Technology | Version |
|-------|------------|---------|
| Frontend Framework | Vanilla TypeScript | 5.x |
| 3D Rendering | three.js | Latest stable |
| Build Tool | Vite | 5.x |
| Backend Language | Rust | 1.70+ |
| Roblox DOM | rbx-dom ecosystem | Latest |
| HTTP Framework | Axum or Actix-web | Latest |
| Frontend Hosting | Vercel or Cloudflare Pages | - |
| Backend Hosting | Railway or Render | - |

## Frontend

### TypeScript + three.js

**Selection Rationale:**
- three.js is the standard for browser-based 3D rendering
- TypeScript provides type safety without React overhead
- Vanilla approach minimizes bundle size and complexity
- Aligns with minecraft-threejs reference implementation

**Rejected Alternatives:**

| Alternative | Rejection Reason |
|-------------|------------------|
| React Three Fiber | Adds React overhead; unnecessary for MVP scope |
| Babylon.js | Heavier than three.js; less community tooling for voxel editors |
| PlayCanvas | Commercial focus; overkill for level editor |

### Vite

**Selection Rationale:**
- Fast development server with HMR
- Native TypeScript support
- Simple configuration
- Modern ES module handling
- Industry standard for three.js projects

### Reference Implementation

The `minecraft-threejs` repository (github.com/vyse12138/minecraft-threejs) provides:
- Block placement/removal mechanics
- Collision detection patterns
- Camera control implementation
- Save/load architecture

**Adaptation Notes:**
- Remove gameplay features (survival, terrain generation)
- Simplify to position + color only
- Add export functionality
- Remove infinite world system

## Backend

### Rust with rbx-dom

**Selection Rationale:**
- rbx-dom is the only mature, write-capable Roblox DOM library
- Battle-tested (powers Rojo, the leading Roblox sync tool)
- Comprehensive format support (XML and binary)
- Bundled reflection database for property validation
- Active maintenance by Rojo team

**Key Crates:**

| Crate | Purpose | Required |
|-------|---------|----------|
| `rbx_dom_weak` | DOM implementation | Yes |
| `rbx_types` | Value types (Vector3, Color3, etc.) | Yes |
| `rbx_xml` | .rbxlx serialization | Yes |
| `rbx_reflection_database` | Property/class reflection | Yes |

**Rejected Alternatives:**

| Alternative | Rejection Reason |
|-------------|------------------|
| Node.js + rbxlx | No mature write-capable libraries |
| Go + rbxfile | Less ecosystem support than Rust |
| Python | No Roblox DOM libraries |
| WASM rbx-dom | Non-trivial compilation; research-grade only |

### HTTP Framework

**Recommended:** Axum or Actix-web

Both are production-ready Rust HTTP frameworks. Selection criteria:
- Axum: Simpler API, tower middleware ecosystem
- Actix-web: Higher raw performance, larger community

For MVP, either is suitable. Axum recommended for simpler learning curve.

## Hosting

### Frontend: Vercel or Cloudflare Pages

**Selection Rationale:**
- Static site deployment (compiled JS/HTML/CSS)
- Global CDN distribution
- Automatic HTTPS
- Git-based deployments
- Generous free tiers

| Provider | Pros | Cons |
|----------|------|------|
| Vercel | Excellent DX, automatic previews | Higher cost at scale |
| Cloudflare Pages | Fast edge network, cheaper | Slightly more setup |

**Recommendation:** Cloudflare Pages for cost efficiency.

### Backend: Railway or Render

**Selection Rationale:**
- Native Rust support (no Docker required)
- Simple deployment from Git
- Automatic builds
- Usage-based pricing
- No cold start issues (unlike serverless)

| Provider | Pros | Cons |
|----------|------|------|
| Railway | Great DX, Docker support, usage-based | Newer platform |
| Render | Native Rust, established, free tier | Slower builds |

**Recommendation:** Railway for developer experience and Rust support.

### Architecture

```
[User Browser]
      |
      v
[Cloudflare Pages] --> Static assets (HTML/JS/CSS)
      |
      | API calls
      v
[Railway] --> Rust backend --> .rbxlx generation
```

## Development Tools

### Required Tools

| Tool | Purpose |
|------|---------|
| Node.js 20+ | Frontend toolchain |
| Rust 1.70+ | Backend compilation |
| Roblox Studio | Testing generated files |
| Git | Version control |

### Recommended IDE Setup

| IDE | Extensions |
|-----|------------|
| VS Code | rust-analyzer, TypeScript, Vite |
| Cursor | Same as VS Code (fork) |

## Dependencies

### Frontend (package.json)

```json
{
  "dependencies": {
    "three": "^0.160.0"
  },
  "devDependencies": {
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "@types/three": "^0.160.0"
  }
}
```

### Backend (Cargo.toml)

```toml
[dependencies]
rbx_dom_weak = "2.7"
rbx_types = "1.8"
rbx_xml = "0.13"
rbx_reflection_database = "0.2"
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Version Pinning Strategy

### Frontend
- Pin major versions in package.json
- Use package-lock.json for exact versions
- Update monthly for security patches

### Backend
- Pin minor versions in Cargo.toml
- Use Cargo.lock for reproducible builds
- Monitor rbx-dom releases for Roblox compatibility

## Scaling Considerations (Post-MVP)

| Concern | Solution |
|---------|----------|
| High traffic | Add Railway replicas |
| Large levels | Implement streaming/chunking |
| Global latency | Multi-region deployment |
| File storage | Add S3/R2 for generated files |

## Related Documents

- `01_architecture_overview.md` - System architecture
- `03_space_json_schema.md` - Data format specification
- `04_rbxlx_generation.md` - Roblox file generation
- `05_technical_constraints.md` - Technical limitations
