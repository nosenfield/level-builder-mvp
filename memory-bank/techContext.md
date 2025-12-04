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
- **Framework**: Axum or Actix-web (TBD)
- **Roblox Library**: rbx-dom ecosystem
- **Serialization**: rbx-xml for `.rbxlx` generation
- **HTTP**: Axum/Actix-web HTTP server

### Infrastructure
- **Frontend Hosting**: Cloudflare Pages
- **Backend Hosting**: Railway
- **CI/CD**: TBD (GitHub Actions likely)
- **Monitoring**: TBD

### Testing
- **Unit Tests**: TBD
- **Integration Tests**: TBD
- **E2E Tests**: TBD

---

## Development Setup

### Prerequisites
```bash
- Node.js 18+ (for frontend)
- npm or yarn (package manager)
- Rust toolchain (for backend - TBD)
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
```

### Environment Variables

**Frontend** (`.env` or `import.meta.env`):
```bash
# Backend API URL (optional, defaults to /api/export)
VITE_API_URL=https://api.example.com/api/export
```

**Backend** (TBD):
```bash
# Port (default: 3000)
PORT=3000

# CORS origins (comma-separated)
CORS_ORIGINS=https://example.com,https://www.example.com
```

---

## Dependencies

### Core Dependencies (Frontend)
- `three@^0.137.0` - 3D graphics library
- `@types/three@^0.137.0` - TypeScript definitions

### Development Dependencies (Frontend)
- `typescript@^4.5.5` - TypeScript compiler
- `vite@^2.8.0` - Build tool and dev server

### Backend Dependencies (TBD)
- `rbx-dom-weak` - Roblox DataModel manipulation
- `rbx-xml` - XML serialization for `.rbxlx`
- `rbx-types` - Roblox type definitions
- `rbx-reflection-database` - Roblox reflection data
- `axum` or `actix-web` - HTTP framework
- `tokio` - Async runtime
- `serde` + `serde_json` - JSON serialization

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
- **Input validation**: Backend validates Space JSON schema
- **Rate limiting**: 10 exports per minute per IP (backend)
- **CORS**: Backend must allow frontend origin
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

### Deployment (Backend - TBD)
```bash
# Railway
# Connect GitHub repo
# Build command: cargo build --release
# Start command: ./target/release/backend
# Auto-deploy on push to main
```

### Environments
- **Development**: `http://localhost:5173` (frontend), `http://localhost:3000` (backend)
- **Staging**: TBD
- **Production**: TBD (Cloudflare Pages + Railway)

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
**Solution**: Check backend validation, verify Space JSON schema, check rbx-dom version compatibility

#### Issue 5: Performance issues with many blocks
**Symptoms**: Low FPS, laggy interactions
**Solution**: Reduce block count, optimize InstancedMesh usage, check browser performance tools

---

## Code Quality Standards

- **TypeScript**: Strict mode enabled
- **No `any` types**: Use proper types or `unknown`
- **Components**: Keep under 200 lines where possible
- **Comments**: Document complex logic
- **Error handling**: Always handle errors gracefully
- **Logging**: Use console.log for debugging (structured logging TBD)
