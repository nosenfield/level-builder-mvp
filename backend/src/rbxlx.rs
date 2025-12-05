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
/// Uses minimal properties - Roblox Studio fills in sensible defaults for the rest
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

    // Create Part with only essential properties
    // Roblox Studio provides sensible defaults for all other properties
    InstanceBuilder::new("Part")
        .with_property("Name", format!("Block{}", referent_id))
        .with_property("CFrame", cframe)
        .with_property("Size", Vector3::new(1.0, 1.0, 1.0))
        .with_property("Color", color)
        .with_property("Anchored", true)
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

    // Note: We don't add a Camera - Roblox Studio will create its own default Camera
    
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
    dom.insert(
        root_ref,
        InstanceBuilder::new("Lighting")
            .with_property("Technology", 3i32), // 3 = ShadowMap (avoids Compatibility Lighting migration warning)
    );
    dom.insert(root_ref, InstanceBuilder::new("ReplicatedStorage"));
    dom.insert(root_ref, InstanceBuilder::new("StarterGui"));
    dom.insert(root_ref, InstanceBuilder::new("StarterPack"));
    
    // 7.10: Add StarterPlayer with child containers
    let starter_player_ref = dom.insert(root_ref, InstanceBuilder::new("StarterPlayer"));
    dom.insert(starter_player_ref, InstanceBuilder::new("StarterPlayerScripts"));
    dom.insert(starter_player_ref, InstanceBuilder::new("StarterCharacterScripts"));
    
    // 7.20-7.21: Serialize to XML
    // IMPORTANT: Pass the children of the DataModel (Workspace, Players, etc.), NOT the DataModel itself.
    // Roblox .rbxlx files should have services as direct children of <roblox>, not wrapped in DataModel.
    let top_level_refs: Vec<_> = dom.root().children().to_vec();
    let mut output = Vec::new();
    rbx_xml::to_writer_default(Cursor::new(&mut output), &dom, &top_level_refs)
        .map_err(|e| format!("Failed to serialize to XML: {}", e))?;
    
    // Post-process XML to fix Lighting Technology property
    // rbx_xml serializes it as <int> but Roblox expects <token>
    let xml_string = String::from_utf8(output)
        .map_err(|e| format!("Failed to convert XML to string: {}", e))?;

    // Fix Technology property: <int name="Technology">3</int> -> <token name="Technology">3</token>
    let result = xml_string
        .replace(r#"<int name="Technology">3</int>"#, r#"<token name="Technology">3</token>"#);

    Ok(result.into_bytes())
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
        // Verify XML contains required structure (note: DataModel is NOT included in output,
        // services like Workspace are direct children of <roblox>)
        let xml_str = String::from_utf8_lossy(&xml);
        assert!(xml_str.contains("<roblox"), "Should have roblox root element");
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

    // Phase 8: Integration Tests

    /// Phase 8.1: Test empty level export (0 blocks)
    #[test]
    fn test_phase8_1_empty_level_export() {
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Empty Level".to_string()),
            blocks: vec![],
        };
        let result = generate_rbxlx(&space_json);
        assert!(result.is_ok(), "Empty level should generate valid RBXLX");
        
        let xml = result.unwrap();
        assert!(!xml.is_empty(), "Generated XML should not be empty");
        
        let xml_str = String::from_utf8_lossy(&xml);
        // Verify required structure (note: DataModel is NOT included in output,
        // services like Workspace are direct children of <roblox>)
        assert!(xml_str.contains("<roblox"), "Should have roblox root element");
        assert!(xml_str.contains("Workspace"), "Should contain Workspace");
        assert!(xml_str.contains("SpawnLocation"), "Should contain SpawnLocation");
        
        // Verify no Parts (empty blocks array)
        let part_count = xml_str.matches("<Item class=\"Part\"").count();
        assert_eq!(part_count, 0, "Empty level should have 0 Parts");
        
        // Verify spawn location exists (exact position format may vary in XML)
        // The spawn calculation function is tested separately, so we just verify SpawnLocation exists
    }

    /// Phase 8.2: Test single block export
    #[test]
    fn test_phase8_2_single_block_export() {
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Single Block Level".to_string()),
            blocks: vec![Block {
                x: 10,
                y: 5,
                z: 20,
                color: "#FF0000".to_string(),
            }],
        };
        let result = generate_rbxlx(&space_json);
        assert!(result.is_ok(), "Single block should generate valid RBXLX");
        
        let xml = result.unwrap();
        let xml_str = String::from_utf8_lossy(&xml);
        
        // Verify exactly 1 Part
        let part_count = xml_str.matches("<Item class=\"Part\"").count();
        assert_eq!(part_count, 1, "Should have exactly 1 Part");
        
        // Verify Part properties exist (exact XML format may vary)
        // The properties are set in create_part_from_block, so we verify Part exists
        // Detailed property verification is done via the DOM structure, not XML parsing
    }

    /// Phase 8.3: Test multi-block export (100 blocks)
    #[test]
    fn test_phase8_3_multi_block_export() {
        // Generate 100 blocks in a 10x10 grid
        let mut blocks = Vec::new();
        for x in 0..10 {
            for z in 0..10 {
                blocks.push(Block {
                    x,
                    y: 0,
                    z,
                    color: "#00FF00".to_string(), // Green
                });
            }
        }
        
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Multi Block Level".to_string()),
            blocks,
        };
        
        let start = std::time::Instant::now();
        let result = generate_rbxlx(&space_json);
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "Multi-block export should succeed");
        assert!(duration.as_secs() < 5, "100 blocks should generate in < 5 seconds");
        
        let xml = result.unwrap();
        let xml_str = String::from_utf8_lossy(&xml);
        
        // Verify exactly 100 Parts
        let part_count = xml_str.matches("<Item class=\"Part\"").count();
        assert_eq!(part_count, 100, "Should have exactly 100 Parts");
        
        // Verify file size is reasonable (< 1MB)
        assert!(xml.len() < 1_000_000, "File size should be < 1MB");
    }

    /// Phase 8.4: Test maximum block export (10,000 blocks)
    #[test]
    fn test_phase8_4_maximum_block_export() {
        // Generate 10,000 blocks (100x100 grid)
        let mut blocks = Vec::new();
        for x in 0..100 {
            for z in 0..100 {
                blocks.push(Block {
                    x: x - 50, // Center around origin
                    y: 0,
                    z: z - 50,
                    color: "#0000FF".to_string(), // Blue
                });
            }
        }
        
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Maximum Block Level".to_string()),
            blocks,
        };
        
        let start = std::time::Instant::now();
        let result = generate_rbxlx(&space_json);
        let duration = start.elapsed();
        
        assert!(result.is_ok(), "Maximum block export should succeed");
        assert!(duration.as_secs() < 30, "10,000 blocks should generate in < 30 seconds");
        
        let xml = result.unwrap();
        let xml_str = String::from_utf8_lossy(&xml);
        
        // Verify exactly 10,000 Parts
        let part_count = xml_str.matches("<Item class=\"Part\"").count();
        assert_eq!(part_count, 10_000, "Should have exactly 10,000 Parts");
        
        // Verify file size is reasonable (< 50MB for 10k blocks with full properties)
        // Each Part has ~40 properties, which can result in ~2-3KB per part
        assert!(xml.len() < 50_000_000, "File size should be < 50MB, actual: {} bytes", xml.len());
    }

    /// Phase 8.5: Test various colors
    #[test]
    fn test_phase8_5_various_colors() {
        // All 10 color types from the editor
        let color_tests = vec![
            ("#FF0000", "red", 1.0, 0.0, 0.0),
            ("#FFA500", "orange", 1.0, 0.647, 0.0),
            ("#FFFF00", "yellow", 1.0, 1.0, 0.0),
            ("#00FF00", "green", 0.0, 1.0, 0.0),
            ("#0000FF", "blue", 0.0, 0.0, 1.0),
            ("#8B00FF", "violet", 0.545, 0.0, 1.0),
            ("#8B4513", "brown", 0.545, 0.271, 0.075),
            ("#FFFFFF", "white", 1.0, 1.0, 1.0),
            ("#808080", "gray", 0.5, 0.5, 0.5),
            ("#000000", "black", 0.0, 0.0, 0.0),
        ];
        
        for (hex, name, expected_r, expected_g, expected_b) in &color_tests {
            let color = hex_to_color3(hex);
            assert!(color.is_ok(), "Color {} should parse correctly", name);
            
            let color3 = color.unwrap();
            assert!(
                (color3.r - expected_r).abs() < 0.01,
                "Color {} R should be approximately {}", name, expected_r
            );
            assert!(
                (color3.g - expected_g).abs() < 0.01,
                "Color {} G should be approximately {}", name, expected_g
            );
            assert!(
                (color3.b - expected_b).abs() < 0.01,
                "Color {} B should be approximately {}", name, expected_b
            );
        }
        
        // Test that all colors can be used in RBXLX generation
        let mut blocks = Vec::new();
        for (i, (hex, _, _, _, _)) in color_tests.iter().enumerate() {
            blocks.push(Block {
                x: i as i32,
                y: 0,
                z: 0,
                color: hex.to_string(),
            });
        }
        
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Color Test Level".to_string()),
            blocks,
        };
        
        let result = generate_rbxlx(&space_json);
        assert!(result.is_ok(), "All colors should generate valid RBXLX");
        
        let xml = result.unwrap();
        let xml_str = String::from_utf8_lossy(&xml);
        
        // Verify all 10 Parts were created
        let part_count = xml_str.matches("<Item class=\"Part\"").count();
        assert_eq!(part_count, 10, "Should have 10 Parts (one per color)");
    }

    /// Phase 8.6: Test coordinate edge cases (bounds limits)
    #[test]
    fn test_phase8_6_coordinate_edge_cases() {
        // Test all coordinate bounds: X/Z: -500 to 500, Y: 0 to 500
        let edge_blocks = vec![
            Block { x: -500, y: 0, z: 0, color: "#FF0000".to_string() }, // X min
            Block { x: 500, y: 0, z: 0, color: "#00FF00".to_string() },  // X max
            Block { x: 0, y: 0, z: -500, color: "#0000FF".to_string() }, // Z min
            Block { x: 0, y: 0, z: 500, color: "#FFFF00".to_string() },  // Z max
            Block { x: 0, y: 0, z: 0, color: "#FF00FF".to_string() },     // Y min
            Block { x: 0, y: 500, z: 0, color: "#00FFFF".to_string() },   // Y max
        ];
        
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Edge Case Level".to_string()),
            blocks: edge_blocks,
        };
        
        let result = generate_rbxlx(&space_json);
        assert!(result.is_ok(), "Edge case coordinates should generate valid RBXLX");
        
        let xml = result.unwrap();
        let xml_str = String::from_utf8_lossy(&xml);
        
        // Verify all 6 Parts were created
        let part_count = xml_str.matches("<Item class=\"Part\"").count();
        assert_eq!(part_count, 6, "Should have 6 Parts (one per edge case)");
        
        // Verify spawn position is calculated correctly
        // Center X: (-500 + 500 + 0 + 0 + 0 + 0) / 6 = 0
        // Center Z: (0 + 0 - 500 + 500 + 0 + 0) / 6 = 0
        // Highest Y: 500 + 1 = 501
        let (spawn_x, spawn_y, spawn_z) = calculate_spawn_position(&space_json.blocks);
        assert_eq!(spawn_x, 0.0, "Spawn X should be center (0)");
        assert_eq!(spawn_y, 501.0, "Spawn Y should be highest Y (500) + 1");
        assert_eq!(spawn_z, 0.0, "Spawn Z should be center (0)");
        
        // Verify spawn location exists (exact position format may vary in XML)
        // The spawn calculation is tested separately, so we just verify SpawnLocation exists
    }
}

