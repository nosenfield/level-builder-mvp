//! RBXLX file generation.
//!
//! Converts Space JSON to valid `.rbxlx` files using `rbx-dom-weak` and `rbx-xml`.
//! Generates a complete Roblox DataModel with Workspace, services, and level geometry.

use crate::models::{Block, SpaceJSON};
use rbx_dom_weak::{InstanceBuilder, WeakDom};
use rbx_types::{CFrame, Color3, Vector3, Vector2, Matrix3, Content};
use std::io::Cursor;

/// Converts a hex color string to Roblox `Color3` (RGB values in 0.0-1.0 range).
///
/// Supports `#RRGGBB` and `#RGB` formats.
pub fn hex_to_color3(hex: &str) -> Result<Color3, String> {
    let hex = hex.trim_start_matches('#');
    
    let (r, g, b) = if hex.len() == 6 {
        // #RRGGBB format
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))?;
        (r, g, b)
    } else if hex.len() == 3 {
        // #RGB format (expand to RRGGBB)
        let r = u8::from_str_radix(&hex[0..1], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))? * 17;
        let g = u8::from_str_radix(&hex[1..2], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))? * 17;
        let b = u8::from_str_radix(&hex[2..3], 16)
            .map_err(|_| format!("Invalid hex color: {}", hex))? * 17;
        (r, g, b)
    } else {
        return Err(format!("Invalid hex color length: {}", hex));
    };
    
    Ok(Color3::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    ))
}

/// Calculates a dynamic spawn position based on block positions.
///
/// Returns `(x, y, z)` where:
/// - `x`, `z`: Center of level (average of all block positions)
/// - `y`: Highest block Y + 1 (spawn above highest block)
///
/// Returns `(0.0, 1.0, 0.0)` if no blocks exist.
///
/// NOTE: Currently unused. SpawnLocation uses fixed position `(0, 0.5, 0)`.
/// Preserved for future use where dynamic spawn positioning may be needed.
#[allow(dead_code)]
pub fn calculate_spawn_position(blocks: &[Block]) -> (f32, f32, f32) {
    if blocks.is_empty() {
        return (0.0, 1.0, 0.0);
    }
    
    let mut sum_x = 0.0;
    let mut sum_z = 0.0;
    let mut max_y = blocks[0].y as f32;
    
    for block in blocks {
        sum_x += block.x as f32;
        sum_z += block.z as f32;
        if block.y as f32 > max_y {
            max_y = block.y as f32;
        }
    }
    
    let center_x = sum_x / blocks.len() as f32;
    let center_z = sum_z / blocks.len() as f32;
    let spawn_y = max_y + 1.0;
    (center_x, spawn_y, center_z)
}

/// Creates a Roblox Part instance from a Block.
///
/// Block coordinates are pre-scaled (2x) by the frontend. Each block becomes
/// a 2x2x2 stud Part in Roblox.
fn create_part_from_block(block: &Block, referent_id: usize) -> InstanceBuilder {
    let color = hex_to_color3(&block.color).unwrap_or_else(|e| {
        eprintln!(
            "Warning: Failed to parse color '{}' for block {}: {}. Using default gray.",
            block.color, referent_id, e
        );
        Color3::new(0.5, 0.5, 0.5)
    });

    let position = Vector3::new(block.x as f32, block.y as f32, block.z as f32);
    let cframe = CFrame::new(position, Matrix3::identity());

    InstanceBuilder::new("Part")
        .with_property("Name", format!("Block{}", referent_id))
        .with_property("CFrame", cframe)
        .with_property("Size", Vector3::new(2.0, 2.0, 2.0))
        .with_property("Color", color)
        .with_property("Anchored", true)
}

/// Generates `.rbxlx` file content from Space JSON.
///
/// Creates a complete Roblox DataModel with:
/// - Workspace containing Terrain, Baseplate, user blocks, and SpawnLocation
/// - Required services (Players, Lighting, ReplicatedStorage, etc.)
pub fn generate_rbxlx(space_json: &SpaceJSON) -> Result<Vec<u8>, String> {
    let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
    let root_ref = dom.root_ref();

    // Create Workspace and Terrain
    let workspace_ref = dom.insert(root_ref, InstanceBuilder::new("Workspace"));
    dom.insert(workspace_ref, InstanceBuilder::new("Terrain"));

    // Add Baseplate (200x16x200 studs, top surface at Y=0)
    let baseplate_color = Color3::new(0.5, 0.5, 0.5);
    let baseplate_position = Vector3::new(0.0, -8.0, 0.0);
    let baseplate_cframe = CFrame::new(baseplate_position, Matrix3::identity());
    dom.insert(
        workspace_ref,
        InstanceBuilder::new("Part")
            .with_property("Name", "Baseplate")
            .with_property("CFrame", baseplate_cframe)
            .with_property("Size", Vector3::new(200.0, 16.0, 200.0))
            .with_property("Color", baseplate_color)
            .with_property("Anchored", true)
            .with_property("Locked", true)
            .with_property("Material", 256i32), // Plastic
    );

    // Add user-placed blocks as Parts
    for (index, block) in space_json.blocks.iter().enumerate() {
        let part = create_part_from_block(block, index);
        dom.insert(workspace_ref, part);
    }

    // Add SpawnLocation (fixed at origin, 12x1x12 studs)
    let spawn_position = Vector3::new(0.0, 0.5, 0.0);
    let spawn_cframe = CFrame::new(spawn_position, Matrix3::identity());
    let spawn_location_ref = dom.insert(
        workspace_ref,
        InstanceBuilder::new("SpawnLocation")
            .with_property("Name", "SpawnLocation")
            .with_property("Anchored", true)
            .with_property("CanCollide", true)
            .with_property("Neutral", true)
            .with_property("Duration", 0.0f32)
            .with_property("Size", Vector3::new(12.0, 1.0, 12.0))
            .with_property("CFrame", spawn_cframe)
            .with_property("Enabled", true)
            .with_property("AllowTeamChangeOnTouch", false),
    );

    // Add Decal to SpawnLocation (texture on top face)
    let decal_color = Color3::new(1.0, 1.0, 1.0);
    let spawn_texture_content = Content::from_uri("rbxasset://textures/SpawnLocation.png");
    let null_content = Content::none();
    dom.insert(
        spawn_location_ref,
        InstanceBuilder::new("Decal")
            .with_property("Color3", decal_color)
            .with_property("MetalnessMap", null_content.clone())
            .with_property("NormalMap", null_content.clone())
            .with_property("RoughnessMap", null_content.clone())
            .with_property("Texture", spawn_texture_content)
            .with_property("TexturePack", null_content)
            .with_property("TexturePackMetadata", String::new())
            .with_property("Transparency", 0.0f32)
            .with_property("UVOffset", Vector2::new(0.0, 0.0))
            .with_property("UVScale", Vector2::new(1.0, 1.0))
            .with_property("ZIndex", 1i32)
            .with_property("Face", 1i32) // Top face
            .with_property("Name", "Decal"),
    );

    // Add required Roblox services
    dom.insert(root_ref, InstanceBuilder::new("Players"));
    dom.insert(
        root_ref,
        InstanceBuilder::new("Lighting")
            .with_property("Technology", 3i32), // ShadowMap
    );
    dom.insert(root_ref, InstanceBuilder::new("ReplicatedStorage"));
    dom.insert(root_ref, InstanceBuilder::new("StarterGui"));
    dom.insert(root_ref, InstanceBuilder::new("StarterPack"));

    let starter_player_ref = dom.insert(root_ref, InstanceBuilder::new("StarterPlayer"));
    dom.insert(starter_player_ref, InstanceBuilder::new("StarterPlayerScripts"));
    dom.insert(starter_player_ref, InstanceBuilder::new("StarterCharacterScripts"));

    // Serialize to XML (services as direct children of <roblox>, not wrapped in DataModel)
    let top_level_refs: Vec<_> = dom.root().children().to_vec();
    let mut output = Vec::new();
    rbx_xml::to_writer_default(Cursor::new(&mut output), &dom, &top_level_refs)
        .map_err(|e| format!("Failed to serialize to XML: {}", e))?;

    // Fix Lighting Technology property (rbx_xml outputs <int>, Roblox expects <token>)
    let xml_string = String::from_utf8(output)
        .map_err(|e| format!("Failed to convert XML to string: {}", e))?;
    let result = xml_string
        .replace(r#"<int name="Technology">3</int>"#, r#"<token name="Technology">3</token>"#);

    Ok(result.into_bytes())
}
