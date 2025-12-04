/**
 * Phase 7: Backend RBXLX Generation
 * 
 * Generates valid .rbxlx files from Space JSON using rbx-dom-weak and rbx-xml.
 * Creates a complete Roblox DataModel with Workspace, services, and level geometry.
 */

use crate::models::{Block, SpaceJSON};
use rbx_dom_weak::{InstanceBuilder, WeakDom};
use rbx_types::{CFrame, Color3, Vector3, Matrix3};
use std::io::Cursor;

/// Convert hex color string to Color3 (RGB 0-1 range)
/// Supports both #RRGGBB and #RGB formats
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

/// Calculate spawn position from blocks
/// Returns (x, y, z) where:
/// - x, z: Center of level (average of all block positions)
/// - y: Highest block Y + 1 (spawn above highest block)
/// If no blocks, returns default (0, 0.5, 0)
pub fn calculate_spawn_position(blocks: &[Block]) -> (f32, f32, f32) {
    if blocks.is_empty() {
        return (0.0, 0.5, 0.0);
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
    let spawn_y = max_y + 1.0; // Spawn 1 stud above highest block
    
    (center_x, spawn_y, center_z)
}

/// Create a Part instance from a Block
fn create_part_from_block(block: &Block, referent_id: usize) -> InstanceBuilder {
    // Convert hex color to Color3
    let color = hex_to_color3(&block.color)
        .unwrap_or_else(|_| Color3::new(0.5, 0.5, 0.5)); // Default to gray on error
    
    // Convert block position to Roblox coordinates
    // Y offset: Space JSON Y becomes Part CFrame Y + 0.5 (block center)
    let position = Vector3::new(
        block.x as f32,
        block.y as f32 + 0.5,
        block.z as f32,
    );
    
    // Create identity rotation matrix (no rotation)
    let cframe = CFrame::new(position, Matrix3::identity());
    
    InstanceBuilder::new("Part")
        .with_property("Name", format!("Block{}", referent_id))
        .with_property("Anchored", true)
        .with_property("CanCollide", true)
        .with_property("Size", Vector3::new(1.0, 1.0, 1.0))
        .with_property("CFrame", cframe)
        .with_property("Color", color)
        .with_property("Material", 256i32) // 256 = Plastic material
        .with_property("Shape", 1i32) // 1 = Block shape
}

/// Generate .rbxlx file content from Space JSON
pub fn generate_rbxlx(space_json: &SpaceJSON) -> Result<Vec<u8>, String> {
    // Create DataModel root
    let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
    let root_ref = dom.root_ref();
    
    // 7.2: Create Workspace service
    let workspace_ref = dom.insert(root_ref, InstanceBuilder::new("Workspace"));
    
    // 7.3: Add Terrain instance to Workspace
    dom.insert(workspace_ref, InstanceBuilder::new("Terrain"));
    
    // 7.4: Add Camera instance to Workspace
    let camera_position = Vector3::new(0.0, 10.0, 20.0);
    let camera_cframe = CFrame::new(camera_position, Matrix3::identity());
    dom.insert(
        workspace_ref,
        InstanceBuilder::new("Camera")
            .with_property("Name", "Camera")
            .with_property("CameraType", 0i32) // 0 = Custom
            .with_property("CFrame", camera_cframe),
    );
    
    // 7.15: Add all Parts as children of Workspace (7.11-7.14 handled in create_part_from_block)
    for (index, block) in space_json.blocks.iter().enumerate() {
        let part = create_part_from_block(block, index);
        dom.insert(workspace_ref, part);
    }
    
    // 7.16-7.19: Add SpawnLocation
    let (spawn_x, spawn_y, spawn_z) = calculate_spawn_position(&space_json.blocks);
    let spawn_position = Vector3::new(spawn_x, spawn_y, spawn_z);
    let spawn_cframe = CFrame::new(spawn_position, Matrix3::identity());
    dom.insert(
        workspace_ref,
        InstanceBuilder::new("SpawnLocation")
            .with_property("Name", "SpawnLocation")
            .with_property("Anchored", true)
            .with_property("CanCollide", true)
            .with_property("Neutral", true)
            .with_property("Duration", 0.0f32)
            .with_property("Size", Vector3::new(6.0, 1.0, 6.0))
            .with_property("CFrame", spawn_cframe)
            .with_property("Enabled", true)
            .with_property("AllowTeamChangeOnTouch", false),
    );
    
    // 7.5-7.10: Create required services
    dom.insert(root_ref, InstanceBuilder::new("Players"));
    dom.insert(root_ref, InstanceBuilder::new("Lighting"));
    dom.insert(root_ref, InstanceBuilder::new("ReplicatedStorage"));
    dom.insert(root_ref, InstanceBuilder::new("StarterGui"));
    dom.insert(root_ref, InstanceBuilder::new("StarterPack"));
    
    // 7.10: Add StarterPlayer with child containers
    let starter_player_ref = dom.insert(root_ref, InstanceBuilder::new("StarterPlayer"));
    dom.insert(starter_player_ref, InstanceBuilder::new("StarterPlayerScripts"));
    dom.insert(starter_player_ref, InstanceBuilder::new("StarterCharacterScripts"));
    
    // 7.20-7.21: Serialize to XML
    let mut output = Vec::new();
    rbx_xml::to_writer_default(Cursor::new(&mut output), &dom, &[root_ref])
        .map_err(|e| format!("Failed to serialize to XML: {}", e))?;
    
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_color3_rrggbb() {
        let color = hex_to_color3("#FF0000").unwrap();
        assert!((color.r - 1.0).abs() < 0.001);
        assert!(color.g.abs() < 0.001);
        assert!(color.b.abs() < 0.001);
    }

    #[test]
    fn test_hex_to_color3_rgb() {
        let color = hex_to_color3("#F00").unwrap();
        assert!((color.r - 1.0).abs() < 0.001);
        assert!(color.g.abs() < 0.001);
        assert!(color.b.abs() < 0.001);
    }

    #[test]
    fn test_hex_to_color3_lowercase() {
        let color = hex_to_color3("#00ff00").unwrap();
        assert!(color.r.abs() < 0.001);
        assert!((color.g - 1.0).abs() < 0.001);
        assert!(color.b.abs() < 0.001);
    }

    #[test]
    fn test_hex_to_color3_invalid() {
        assert!(hex_to_color3("not-a-color").is_err());
        assert!(hex_to_color3("#GGGGGG").is_err());
    }

    #[test]
    fn test_calculate_spawn_position_empty() {
        let blocks = vec![];
        let (x, y, z) = calculate_spawn_position(&blocks);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.5);
        assert_eq!(z, 0.0);
    }

    #[test]
    fn test_calculate_spawn_position_single() {
        let blocks = vec![Block {
            x: 10,
            y: 5,
            z: 20,
            color: "#FF0000".to_string(),
        }];
        let (x, y, z) = calculate_spawn_position(&blocks);
        assert_eq!(x, 10.0);
        assert_eq!(y, 6.0); // Highest Y (5) + 1
        assert_eq!(z, 20.0);
    }

    #[test]
    fn test_calculate_spawn_position_multiple() {
        let blocks = vec![
            Block {
                x: 0,
                y: 0,
                z: 0,
                color: "#FF0000".to_string(),
            },
            Block {
                x: 10,
                y: 5,
                z: 10,
                color: "#00FF00".to_string(),
            },
            Block {
                x: 20,
                y: 3,
                z: 20,
                color: "#0000FF".to_string(),
            },
        ];
        let (x, y, z) = calculate_spawn_position(&blocks);
        assert_eq!(x, 10.0); // Average of 0, 10, 20
        assert_eq!(y, 6.0); // Highest Y (5) + 1
        assert_eq!(z, 10.0); // Average of 0, 10, 20
    }

    #[test]
    fn test_generate_rbxlx_empty() {
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Test Level".to_string()),
            blocks: vec![],
        };
        let result = generate_rbxlx(&space_json);
        assert!(result.is_ok());
        let xml = result.unwrap();
        assert!(!xml.is_empty());
        // Verify XML contains DataModel
        let xml_str = String::from_utf8_lossy(&xml);
        assert!(xml_str.contains("DataModel"));
        assert!(xml_str.contains("Workspace"));
        assert!(xml_str.contains("SpawnLocation"));
    }

    #[test]
    fn test_generate_rbxlx_single_block() {
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Test Level".to_string()),
            blocks: vec![Block {
                x: 0,
                y: 0,
                z: 0,
                color: "#FF0000".to_string(),
            }],
        };
        let result = generate_rbxlx(&space_json);
        assert!(result.is_ok());
        let xml = result.unwrap();
        assert!(!xml.is_empty());
        let xml_str = String::from_utf8_lossy(&xml);
        assert!(xml_str.contains("Part"));
    }

    #[test]
    fn test_generate_rbxlx_multiple_blocks() {
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Test Level".to_string()),
            blocks: vec![
                Block {
                    x: 0,
                    y: 0,
                    z: 0,
                    color: "#FF0000".to_string(),
                },
                Block {
                    x: 1,
                    y: 0,
                    z: 0,
                    color: "#00FF00".to_string(),
                },
            ],
        };
        let result = generate_rbxlx(&space_json);
        assert!(result.is_ok());
        let xml = result.unwrap();
        assert!(!xml.is_empty());
    }
}

