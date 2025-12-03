# Technical Constraints

## Document Metadata
- Scope: MVP
- Version: 1.0
- Status: Active

## Purpose

This document catalogs technical constraints, known issues, and required mitigations for the MVP implementation.

## Constraint Categories

| Category | Priority |
|----------|----------|
| Critical | Must address before MVP launch |
| High | Should address before MVP launch |
| Medium | Can defer to post-MVP |
| Low | Document for awareness |

## Critical Constraints

### C1: Schema Versioning Required

**Issue:** Without schema versioning, breaking changes to Space JSON will cause silent failures or data corruption.

**Impact:** Frontend/backend version mismatches will produce broken exports.

**Mitigation:**
- Include `schemaVersion: 1` in all Space JSON payloads
- Backend rejects payloads with unknown versions
- Version check is first validation step

**Implementation:**
```rust
if space_json.schema_version != 1 {
    return Err(Error::UnsupportedVersion(space_json.schema_version));
}
```

### C2: Roblox Studio Configuration Drift

**Issue:** Roblox Studio modifies places when opened:
- Inserts missing services automatically
- Updates Lighting defaults
- Adjusts property type mismatches
- Removes deprecated properties

**Impact:** Generated files may differ after save in Studio.

**Mitigation:**
- Include all standard services in generated files
- Use Roblox default values where possible
- Accept that Studio modifications are normal
- Do not rely on exact file reproduction for testing

**Affected Services:**
| Service | Studio Behavior |
|---------|-----------------|
| Lighting | May update Technology property |
| Workspace | May add Terrain if missing |
| Players | May adjust MaxPlayers |
| Chat | May update ChatVersion |

### C3: SpawnLocation Must Be in Workspace

**Issue:** SpawnLocations reparented to Workspace after initial creation may not function as spawn points.

**Impact:** Players may spawn at origin or fall into void.

**Mitigation:**
- Create SpawnLocation as direct child of Workspace during DOM construction
- Never reparent SpawnLocation after creation
- Verify SpawnLocation.Parent == Workspace in validation

**Verification:**
```rust
fn validate_spawn(dom: &WeakDom) -> Result<(), Error> {
    let workspace = find_service(dom, "Workspace")?;
    let spawn = find_instance(dom, "SpawnLocation")?;
    if spawn.parent != workspace {
        return Err(Error::InvalidSpawnParent);
    }
    Ok(())
}
```

### C4: Coordinate Bounds Enforcement

**Issue:** Unbounded coordinates can cause:
- Roblox Studio crashes
- Rendering failures
- Physics instability
- Memory exhaustion

**Impact:** Malformed payloads could DOS the backend or corrupt output.

**Mitigation:**
- Enforce strict bounds on all coordinates
- Validate before processing
- Return clear error messages

**Bounds:**
| Axis | Min | Max | Validation |
|------|-----|-----|------------|
| X | -500 | 500 | Reject if outside |
| Y | 0 | 500 | Reject if outside |
| Z | -500 | 500 | Reject if outside |

### C5: Block Count Limits

**Issue:** Large block counts cause:
- Backend memory exhaustion
- Slow XML generation
- Large file downloads
- Studio performance degradation

**Impact:** Uncapped payloads could exhaust server resources.

**Mitigation:**
- Hard limit: 10,000 blocks per level
- Reject payloads exceeding limit with 413 error
- Document limit in UI

**Response:**
```json
{
  "error": "PAYLOAD_TOO_LARGE",
  "message": "Level exceeds maximum block count",
  "details": {
    "max_blocks": 10000,
    "received_blocks": 15234
  }
}
```

## High Priority Constraints

### H1: Duplicate Block Positions

**Issue:** Multiple blocks at same (x, y, z) coordinates create overlapping geometry.

**Impact:** Visual glitches, Z-fighting, wasted file size.

**Mitigation:**
- Validate uniqueness of positions in backend
- Use Set/HashMap keyed by position for O(1) lookup
- Reject duplicates with specific error

**Detection:**
```rust
let mut positions: HashSet<(i32, i32, i32)> = HashSet::new();
for block in &space_json.blocks {
    let pos = (block.x, block.y, block.z);
    if !positions.insert(pos) {
        return Err(Error::DuplicatePosition(pos));
    }
}
```

### H2: Color Format Validation

**Issue:** Invalid color strings cause parsing failures or incorrect rendering.

**Impact:** Backend crashes or Parts with wrong colors.

**Mitigation:**
- Validate hex pattern: `^#[0-9A-Fa-f]{6}$`
- Normalize to uppercase before processing
- Provide default color for invalid values (or reject)

**Regex Validation:**
```rust
let color_regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
if !color_regex.is_match(&block.color) {
    return Err(Error::InvalidColor(block.color.clone()));
}
```

### H3: Referent Uniqueness

**Issue:** Duplicate referent values in `.rbxlx` cause Studio load failures.

**Impact:** Generated files won't open in Studio.

**Mitigation:**
- Use sequential counter for referent generation
- Prefix with "RBX" per Roblox convention
- Verify uniqueness in validation pass

**Generation:**
```rust
struct ReferentGenerator {
    counter: u64,
}

impl ReferentGenerator {
    fn next(&mut self) -> String {
        self.counter += 1;
        format!("RBX{:08X}", self.counter)
    }
}
```

### H4: JSON Payload Size Limit

**Issue:** Extremely large JSON payloads can exhaust server memory during parsing.

**Impact:** Server instability, potential DOS vector.

**Mitigation:**
- Set Content-Length limit on HTTP endpoint
- Limit: 2 MB (sufficient for 10k blocks)
- Streaming JSON parser if needed

**HTTP Configuration (Axum):**
```rust
.layer(DefaultBodyLimit::max(2 * 1024 * 1024)) // 2 MB
```

## Medium Priority Constraints

### M1: Reflection Database Drift

**Issue:** Roblox updates add/remove/modify classes and properties. The rbx_reflection_database crate may lag behind.

**Impact:** New properties may not serialize correctly.

**Mitigation:**
- Monitor rbx-dom releases
- Update dependencies monthly
- Use only stable, well-established properties
- Avoid newly-introduced properties

### M2: XML Special Characters

**Issue:** User-provided level names may contain XML special characters.

**Impact:** Malformed XML output.

**Mitigation:**
- Sanitize level names before insertion
- Escape: `<`, `>`, `&`, `"`, `'`
- Or use CDATA sections for string content

**Sanitization:**
```rust
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}
```

### M3: Browser Compatibility

**Issue:** three.js WebGL requirements vary across browsers.

**Impact:** Editor may not function on older browsers.

**Mitigation:**
- Target modern browsers only (Chrome 90+, Firefox 90+, Safari 15+, Edge 90+)
- Display clear error for unsupported browsers
- Test on major browsers before launch

**Detection:**
```typescript
if (!window.WebGLRenderingContext) {
    showError("WebGL is required but not supported by your browser.");
}
```

### M4: Export Timeout

**Issue:** Very large levels may exceed reasonable response times.

**Impact:** User sees timeout error, no file delivered.

**Mitigation:**
- Target < 5 second generation time
- Set 30 second server timeout
- Show progress indicator in UI
- Consider async generation for post-MVP

## Low Priority Constraints

### L1: File Naming

**Issue:** Level names may contain invalid filename characters.

**Impact:** Download may fail or produce oddly-named files.

**Mitigation:**
- Sanitize filename for Content-Disposition header
- Replace invalid characters with underscores
- Truncate to reasonable length

**Invalid Characters:** `< > : " / \ | ? *`

### L2: Mobile Support

**Issue:** three.js editor may not work well on mobile devices.

**Impact:** Mobile users cannot create levels.

**Mitigation:**
- MVP targets desktop only
- Display "desktop recommended" message on mobile
- Consider mobile support post-MVP

### L3: Accessibility

**Issue:** 3D editor inherently difficult for screen readers.

**Impact:** Reduced accessibility.

**Mitigation:**
- Ensure keyboard navigation works
- Provide text-based fallback where possible
- Document as known limitation for MVP

## Risk Matrix

| Constraint | Likelihood | Impact | Priority |
|------------|------------|--------|----------|
| C1: Schema Version | High | High | Critical |
| C2: Config Drift | Certain | Medium | Critical |
| C3: SpawnLocation | Medium | High | Critical |
| C4: Coord Bounds | Medium | High | Critical |
| C5: Block Count | Medium | High | Critical |
| H1: Duplicates | Medium | Medium | High |
| H2: Color Format | Medium | Medium | High |
| H3: Referents | Low | High | High |
| H4: Payload Size | Low | High | High |
| M1: Reflection | Low | Medium | Medium |
| M2: XML Escape | Low | Medium | Medium |
| M3: Browser | Low | Medium | Medium |
| M4: Timeout | Low | Medium | Medium |
| L1: Filename | Low | Low | Low |
| L2: Mobile | Low | Low | Low |
| L3: Accessibility | Low | Low | Low |

## Testing Requirements

### Unit Tests

- Schema version rejection
- Coordinate bounds validation
- Color format validation
- Duplicate position detection
- Referent uniqueness

### Integration Tests

- Full Space JSON to .rbxlx pipeline
- Generated file opens in Roblox Studio
- SpawnLocation functions correctly
- Parts appear at correct positions
- Colors render correctly

### Load Tests

- 10,000 block generation time
- Concurrent export requests
- Memory usage under load

## Related Documents

- `01_architecture_overview.md` - System architecture
- `02_technology_stack.md` - Technology details
- `03_space_json_schema.md` - Input validation
- `04_rbxlx_generation.md` - Output generation
