# RBXLX Generation

## Document Metadata
- Scope: MVP
- Version: 1.0
- Status: Active

## Purpose

This document specifies how the Rust backend transforms Space JSON into valid `.rbxlx` files that open correctly in Roblox Studio.

## RBXLX Format Overview

### File Structure

The `.rbxlx` format is XML-based with the following structure:

```xml
<roblox version="4">
  <Meta name="ExplicitAutoJoints">true</Meta>
  <External>null</External>
  <External>nil</External>
  <Item class="ServiceClassName" referent="UniqueID">
    <Properties>
      <!-- property elements -->
    </Properties>
    <!-- child Items -->
  </Item>
  <!-- more Items -->
</roblox>
```

### Key Elements

| Element | Purpose |
|---------|---------|
| `<roblox>` | Root element, version must be "4" |
| `<Meta>` | File metadata key-value pairs |
| `<External>` | Legacy elements (include for compatibility) |
| `<Item>` | Represents a Roblox Instance |
| `<Properties>` | Container for property values |

### Referent System

Every `<Item>` requires a unique `referent` attribute for internal references.

Format: `RBX` followed by UUID or sequential ID.

Example: `RBX0001`, `RBXA1B2C3D4`

## Required DataModel Structure

### Minimal Playable Place

```
DataModel
├── Workspace
│   ├── Terrain
│   ├── Camera
│   ├── SpawnLocation
│   └── [Level Blocks as Parts]
├── Players
├── Lighting
├── ReplicatedStorage
├── ServerScriptService
├── ServerStorage
├── StarterGui
├── StarterPack
├── StarterPlayer
│   ├── StarterPlayerScripts
│   └── StarterCharacterScripts
├── SoundService
└── Chat
```

### Service Configuration

Services are top-level singletons. They cannot be created via `Instance.new()` but must exist in the DataModel.

| Service | Required | Notes |
|---------|----------|-------|
| Workspace | Yes | Contains all visible geometry |
| Players | Yes | Player management |
| Lighting | Yes | Rendering settings |
| ReplicatedStorage | Yes | Shared storage |
| ServerScriptService | No | Server scripts (empty for MVP) |
| ServerStorage | No | Server-only storage |
| StarterGui | Yes | UI containers |
| StarterPack | Yes | Tool containers |
| StarterPlayer | Yes | Player configuration |
| SoundService | No | Audio settings |
| Chat | No | Chat system |

## Part Generation

### Block to Part Mapping

Each block in Space JSON becomes a `Part` in Workspace.

| Space JSON | Roblox Part |
|------------|-------------|
| x | Position.X |
| y | Position.Y + 0.5 |
| z | Position.Z |
| color | Color3 property |

### Part Properties

```xml
<Item class="Part" referent="RBXBlockNNNN">
  <Properties>
    <string name="Name">Block</string>
    <bool name="Anchored">true</bool>
    <bool name="CanCollide">true</bool>
    <token name="shape">1</token>
    <Vector3 name="size">
      <X>1</X>
      <Y>1</Y>
      <Z>1</Z>
    </Vector3>
    <CoordinateFrame name="CFrame">
      <X>0</X>
      <Y>0.5</Y>
      <Z>0</Z>
      <R00>1</R00>
      <R01>0</R01>
      <R02>0</R02>
      <R10>0</R10>
      <R11>1</R11>
      <R12>0</R12>
      <R20>0</R20>
      <R21>0</R21>
      <R22>1</R22>
    </CoordinateFrame>
    <Color3 name="Color">
      <R>0.5</R>
      <G>0.5</G>
      <B>0.5</B>
    </Color3>
    <token name="Material">256</token>
  </Properties>
</Item>
```

### Required Part Properties

| Property | Type | Value | Purpose |
|----------|------|-------|---------|
| Name | string | "Block" | Identifier |
| Anchored | bool | true | Prevents physics movement |
| CanCollide | bool | true | Player collision |
| shape | token | 1 | Block shape (1 = Block) |
| size | Vector3 | (1,1,1) | 1x1x1 studs |
| CFrame | CoordinateFrame | Position + Identity rotation | Location |
| Color | Color3 | RGB 0-1 | Block color |
| Material | token | 256 | Plastic material |

### Color Conversion

Hex to Color3 (0-1 range):

```rust
fn hex_to_color3(hex: &str) -> (f32, f32, f32) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;
    (r, g, b)
}
```

## SpawnLocation

### Purpose

SpawnLocation determines where players appear when joining or respawning. Must be in Workspace for player spawning to function.

### Configuration

```xml
<Item class="SpawnLocation" referent="RBXSpawn0001">
  <Properties>
    <string name="Name">SpawnLocation</string>
    <bool name="Anchored">true</bool>
    <bool name="CanCollide">true</bool>
    <Vector3 name="size">
      <X>6</X>
      <Y>1</Y>
      <Z>6</Z>
    </Vector3>
    <CoordinateFrame name="CFrame">
      <X>0</X>
      <Y>0.5</Y>
      <Z>0</Z>
      <!-- Identity rotation matrix -->
      <R00>1</R00><R01>0</R01><R02>0</R02>
      <R10>0</R10><R11>1</R11><R12>0</R12>
      <R20>0</R20><R21>0</R21><R22>1</R22>
    </CoordinateFrame>
    <bool name="Neutral">true</bool>
    <float name="Duration">0</float>
    <bool name="Enabled">true</bool>
    <bool name="AllowTeamChangeOnTouch">false</bool>
  </Properties>
</Item>
```

### Spawn Placement Strategy

1. Calculate level bounding box from blocks
2. Place spawn at center X/Z
3. Place spawn Y above highest block at that position
4. If no blocks exist, place at (0, 0.5, 0)

## Workspace Configuration

### Terrain

Empty terrain is required:

```xml
<Item class="Terrain" referent="RBXTerrain">
  <Properties>
    <string name="Name">Terrain</string>
  </Properties>
</Item>
```

### Camera

Default camera for edit mode:

```xml
<Item class="Camera" referent="RBXCamera">
  <Properties>
    <string name="Name">Camera</string>
    <token name="CameraType">0</token>
    <CoordinateFrame name="CFrame">
      <X>0</X><Y>10</Y><Z>20</Z>
      <R00>1</R00><R01>0</R01><R02>0</R02>
      <R10>0</R10><R11>1</R11><R12>0</R12>
      <R20>0</R20><R21>0</R21><R22>1</R22>
    </CoordinateFrame>
  </Properties>
</Item>
```

## Generation Algorithm

### Pseudocode

```
function generate_rbxlx(space_json: SpaceJSON) -> String:
    // Initialize DOM
    dom = new WeakDom()
    root = dom.root()
    
    // Create services
    workspace = create_service(dom, "Workspace")
    add_terrain(workspace)
    add_camera(workspace)
    
    // Generate blocks
    for block in space_json.blocks:
        part = create_part(block)
        parent_to(part, workspace)
    
    // Add spawn location
    spawn_pos = calculate_spawn_position(space_json.blocks)
    spawn = create_spawn_location(spawn_pos)
    parent_to(spawn, workspace)
    
    // Create other required services
    create_service(dom, "Players")
    create_service(dom, "Lighting")
    create_service(dom, "ReplicatedStorage")
    create_service(dom, "StarterGui")
    create_service(dom, "StarterPack")
    create_starter_player(dom)
    
    // Serialize
    return rbx_xml::to_string(&dom)
```

### Rust Implementation Pattern

```rust
use rbx_dom_weak::{InstanceBuilder, WeakDom};
use rbx_types::{Color3, Vector3, CFrame};

pub fn generate(space_json: &SpaceJSON) -> Result<Vec<u8>, Error> {
    let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
    let root = dom.root_ref();
    
    // Build Workspace
    let workspace = dom.insert(root, InstanceBuilder::new("Workspace"));
    
    // Add parts for each block
    for block in &space_json.blocks {
        let part = InstanceBuilder::new("Part")
            .with_property("Anchored", true)
            .with_property("Size", Vector3::new(1.0, 1.0, 1.0))
            .with_property("CFrame", CFrame::new(
                Vector3::new(block.x as f32, block.y as f32 + 0.5, block.z as f32),
                Matrix3::identity()
            ))
            .with_property("Color", hex_to_color3(&block.color));
        dom.insert(workspace, part);
    }
    
    // Add SpawnLocation
    let spawn = InstanceBuilder::new("SpawnLocation")
        .with_property("Anchored", true)
        .with_property("Neutral", true);
    dom.insert(workspace, spawn);
    
    // Serialize to XML
    let mut output = Vec::new();
    rbx_xml::to_writer_default(&mut output, &dom, &[root])?;
    Ok(output)
}
```

## Validation Checklist

Before returning `.rbxlx` to user:

- [ ] XML is well-formed
- [ ] Root element is `<roblox version="4">`
- [ ] All referents are unique
- [ ] Workspace exists with Terrain and Camera
- [ ] SpawnLocation is direct child of Workspace
- [ ] All Parts have required properties
- [ ] Color values are in 0-1 range
- [ ] CFrame matrices are valid

## Known Issues

### Configuration Drift

Roblox Studio modifies places on open:
- May insert missing services
- May update Lighting defaults
- May adjust property values

**Mitigation:** Include all standard services with sensible defaults. Accept that Studio may modify the file.

### SpawnLocation Reparenting Bug

SpawnLocations added to DataModel but not initially parented to Workspace may not function as spawn points.

**Mitigation:** Always create SpawnLocation as direct child of Workspace in initial DOM construction.

## Performance Considerations

### Target Metrics

| Metric | Target |
|--------|--------|
| 1,000 blocks | < 1 second |
| 10,000 blocks | < 5 seconds |
| File size (10k blocks) | < 5 MB |

### Optimization Strategies

1. Batch property writes
2. Use string interning for repeated values
3. Pre-allocate DOM capacity
4. Stream XML output (don't build full string in memory)

## Related Documents

- `01_architecture_overview.md` - System architecture
- `02_technology_stack.md` - rbx-dom details
- `03_space_json_schema.md` - Input format
- `05_technical_constraints.md` - Limitations
