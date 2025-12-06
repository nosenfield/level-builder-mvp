# Technical Context: mvp

**Last Updated**: December 2024

## Tech Stack

### Frontend
- **Framework**: Vanilla TypeScript (no framework)
- **Language**: TypeScript 4.5.5
- **Build Tool**: Vite 2.8.0
- **3D Library**: three.js 0.137.0
- **Type Definitions**: @types/three 0.137.0
- **Styling**: CSS (no framework)

### Backend
- **Language**: Rust
- **Framework**: Axum 0.7
- **Roblox Library**: rbx-dom ecosystem (rbx_dom_weak, rbx_xml, rbx_types)
- **Serialization**: rbx-xml for `.rbxlx` generation
- **HTTP**: Axum HTTP server

### Infrastructure
- **Frontend Hosting**: Cloudflare Pages (✅ deployed and tested)
- **Backend Hosting**: Railway (✅ deployed and tested)
- **Backend URL**: `https://level-builder-mvp-production.up.railway.app`
- **Health Endpoint**: `/health` and `/api/health` (verified working)
- **CI/CD**: Manual deployment via Railway and Cloudflare Pages (auto-deploy on git push)
- **Monitoring**: Railway logs + Cloudflare Pages analytics

### Testing
- **Backend Tests**: 46 tests passing (26 validation + 9 RBXLX + 6 Phase 8 integration + 1 integration + 4 baseplate/spawn)
- **Frontend Tests**: TBD
- **E2E Tests**: TBD

---

## Development Setup

### Prerequisites
```bash
- Node.js 18+ (for frontend)
- npm or yarn (package manager)
- Rust 1.70+ (for backend - install from rustup.rs)
- Modern browser with WebGL support
```

### Installation
```bash
# Frontend
cd frontend
npm install  # or yarn install

# Start development server
npm run dev  # Runs on Vite default port (usually 5173)

# Build for production
npm run build

# Preview production build
npm run preview

# Backend
cd backend
cargo build  # Builds dependencies (first time may take a while)

# Start development server
cargo run  # Runs on port 3000 by default
```

### Environment Variables

**Frontend** (`.env` or `import.meta.env`):
```bash
# Backend API URL (optional, defaults to /api/export)
VITE_API_URL=https://api.example.com/api/export
```

**Note**: Optional - if not set, export uses relative `/api/export` path (for local development).

**Backend** (`.env` or environment variables):
```bash
# Port (default: 4000, Railway provides automatically)
PORT=4000

# CORS is configured to allow all origins for MVP development
# Production CORS configuration documented in deployment guide
```

---

## Dependencies

### Core Dependencies (Frontend)
- `three@^0.137.0` - 3D graphics library
- `@types/three@^0.137.0` - TypeScript definitions

### Development Dependencies (Frontend)
- `typescript@^4.5.5` - TypeScript compiler
- `vite@^2.8.0` - Build tool and dev server

### Backend Dependencies (All Complete)
- `axum = "0.7"` - HTTP web framework ✅
- `tokio = { version = "1", features = ["full"] }` - Async runtime ✅
- `serde = { version = "1", features = ["derive"] }` - JSON serialization ✅
- `serde_json = "1"` - JSON parsing ✅
- `tower-http = { version = "0.5", features = ["cors"] }` - CORS middleware ✅
- `rbx_dom_weak = "4.0"` - Roblox DataModel manipulation ✅
- `rbx_xml = "2.0"` - XML serialization for `.rbxlx` ✅
- `rbx_types = "3.0"` - Roblox type definitions (Color3, Vector3, CFrame, etc.) ✅

### Why We Chose These

**three.js**: Industry standard for WebGL, excellent performance, well-documented, large community.

**Vite**: Fast dev server, excellent TypeScript support, simple configuration, modern build tooling.

**Rust + rbx-dom**: 
- Rust: Performance, memory safety, excellent for server-side processing
- rbx-dom: Official Rust ecosystem for Roblox file manipulation, actively maintained

**Cloudflare Pages**: Free tier, excellent CDN, easy deployment, good for static sites.

**Railway**: Easy Rust deployment, good developer experience, reasonable pricing.

---

## Technical Constraints

### Performance Requirements
- **Page load**: < 3 seconds on broadband
- **Editor frame rate**: >= 30 FPS with 5,000 blocks
- **Block placement latency**: < 50ms
- **Export generation time**: < 5 seconds for 10,000 blocks
- **API response time**: < 5 seconds (includes file generation)

### Platform Constraints
- **Must support**: Chrome 90+, Firefox 90+, Safari 15+, Edge 90+
- **Must work offline**: No (requires backend for export)
- **Mobile responsive**: No (desktop only for MVP)
- **WebGL required**: Yes (three.js dependency)

### Security Requirements
- **Authentication**: None (MVP)
- **Authorization**: None (MVP)
- **Input validation**: Backend validates Space JSON schema (schema version, block count, coordinate bounds, color format, duplicate positions)
- **CORS**: Backend configured to allow all origins (MVP development)
- **XSS prevention**: Sanitize user input (level names)

---

## Build & Deployment

### Build Process (Frontend)
```bash
cd frontend
npm run build
# Output: dist/ directory with static files
```

### Deployment (Frontend)
```bash
# Cloudflare Pages
# Connect GitHub repo
# Build command: npm run build
# Output directory: dist
# Auto-deploy on push to main
```

### Deployment (Backend)
```bash
# Railway
# Connect GitHub repo
# Build command: cargo build --release
# Start command: ./target/release/backend
# Auto-deploy on push to main
# Set PORT environment variable (Railway provides automatically)
# Health endpoint: GET /health or GET /api/health
# See _docs/deployment.md for detailed instructions
```

### Environments
- **Development**: `http://localhost:5173` (frontend), `http://localhost:4000` (backend)
- **Production**: 
  - Frontend: Cloudflare Pages (deployed and tested)
  - Backend: Railway (`https://level-builder-mvp-production.up.railway.app`)
  - Health endpoint: `https://level-builder-mvp-production.up.railway.app/health`
  - Export endpoint: `https://level-builder-mvp-production.up.railway.app/api/export`

---

## Troubleshooting

### Common Issues

#### Issue 1: WebGL not supported
**Symptoms**: Black screen, console errors about WebGL
**Solution**: Check browser WebGL support, update graphics drivers, try different browser

#### Issue 2: Export fails with CORS error
**Symptoms**: Browser console shows CORS policy error
**Solution**: Ensure backend CORS is configured to allow frontend origin

#### Issue 3: Blocks not rendering
**Symptoms**: Empty scene, no blocks visible
**Solution**: Check camera position, verify blocks array is populated, check InstancedMesh setup

#### Issue 4: Export returns invalid file
**Symptoms**: Roblox Studio shows error opening file
**Solution**: Check backend validation, verify Space JSON schema, check rbx_xml version compatibility

#### Issue 5: Parts not appearing in Roblox Studio
**Symptoms**: File opens but no blocks visible in Workspace
**Solution**: Ensure XML serialization passes `dom.root().children()` instead of DataModel root. Services must be direct children of `<roblox>`, not wrapped in `<Item class="DataModel">`.

#### Issue 6: Lighting Technology migration warning
**Symptoms**: Roblox Studio shows "Compatibility Lighting" migration warning
**Solution**: Set Lighting service Technology property to 3 (ShadowMap) before serialization.

#### Issue 7: Duplicate Camera in scene
**Symptoms**: Multiple Camera instances in Workspace
**Solution**: Don't create custom Camera - Roblox Studio creates its own automatically with proper CurrentCamera reference.

#### Issue 8: Performance issues with many blocks
**Symptoms**: Low FPS, laggy interactions
**Solution**: Reduce block count, optimize InstancedMesh usage, check browser performance tools

---

## Code Quality Standards

- **TypeScript**: Strict mode enabled
- **No `any` types**: Use proper types or `unknown`
- **Components**: Keep under 200 lines where possible
- **Comments**: Document complex logic
- **Error handling**: Always handle errors gracefully
- **Logging**: Use console.log for debugging
