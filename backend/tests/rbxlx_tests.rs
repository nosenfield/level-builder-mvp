//! Tests for RBXLX file generation.

use backend::models::{Block, SpaceJSON};
use backend::rbxlx::{calculate_spawn_position, generate_rbxlx, hex_to_color3};

// Color conversion tests

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

// Spawn position calculation tests

#[test]
fn test_calculate_spawn_position_empty() {
    let blocks = vec![];
    let (x, y, z) = calculate_spawn_position(&blocks);
    assert_eq!(x, 0.0);
    assert_eq!(y, 1.0);
    assert_eq!(z, 0.0);
}

#[test]
fn test_calculate_spawn_position_single() {
    let blocks = vec![Block {
        x: 20,
        y: 10,
        z: 40,
        color: "#FF0000".to_string(),
    }];
    let (x, y, z) = calculate_spawn_position(&blocks);
    assert_eq!(x, 20.0);
    assert_eq!(y, 11.0);
    assert_eq!(z, 40.0);
}

#[test]
fn test_calculate_spawn_position_multiple() {
    let blocks = vec![
        Block { x: 0, y: 0, z: 0, color: "#FF0000".to_string() },
        Block { x: 20, y: 10, z: 20, color: "#00FF00".to_string() },
        Block { x: 40, y: 6, z: 40, color: "#0000FF".to_string() },
    ];
    let (x, y, z) = calculate_spawn_position(&blocks);
    assert_eq!(x, 20.0);
    assert_eq!(y, 11.0);
    assert_eq!(z, 20.0);
}

// RBXLX generation tests

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
    let xml_str = String::from_utf8_lossy(&xml);
    assert!(xml_str.contains("<roblox"));
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
    let xml_str = String::from_utf8_lossy(&xml);
    assert!(xml_str.contains("Part"));
}

#[test]
fn test_generate_rbxlx_multiple_blocks() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Test Level".to_string()),
        blocks: vec![
            Block { x: 0, y: 0, z: 0, color: "#FF0000".to_string() },
            Block { x: 1, y: 0, z: 0, color: "#00FF00".to_string() },
        ],
    };
    let result = generate_rbxlx(&space_json);
    assert!(result.is_ok());
}

// Integration tests

#[test]
fn test_empty_level_export() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Empty Level".to_string()),
        blocks: vec![],
    };
    let result = generate_rbxlx(&space_json);
    assert!(result.is_ok(), "Empty level should generate valid RBXLX");

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    assert!(xml_str.contains("<roblox"));
    assert!(xml_str.contains("Workspace"));
    assert!(xml_str.contains("SpawnLocation"));

    let part_count = xml_str.matches("<Item class=\"Part\"").count();
    assert_eq!(part_count, 1, "Empty level should have 1 Part (baseplate)");
}

#[test]
fn test_single_block_export() {
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
    assert!(result.is_ok());

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    let part_count = xml_str.matches("<Item class=\"Part\"").count();
    assert_eq!(part_count, 2, "Should have exactly 2 Parts (baseplate + 1 block)");
}

#[test]
fn test_multi_block_export() {
    let blocks: Vec<Block> = (0..10)
        .flat_map(|x| {
            (0..10).map(move |z| Block {
                x,
                y: 0,
                z,
                color: "#00FF00".to_string(),
            })
        })
        .collect();

    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Multi Block Level".to_string()),
        blocks,
    };

    let start = std::time::Instant::now();
    let result = generate_rbxlx(&space_json);
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(duration.as_secs() < 5, "100 blocks should generate in < 5 seconds");

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    let part_count = xml_str.matches("<Item class=\"Part\"").count();
    assert_eq!(part_count, 101, "Should have 101 Parts (baseplate + 100 blocks)");
    assert!(xml.len() < 1_000_000, "File size should be < 1MB");
}

#[test]
fn test_maximum_block_export() {
    let blocks: Vec<Block> = (0..100)
        .flat_map(|x| {
            (0..100).map(move |z| Block {
                x: x - 50,
                y: 0,
                z: z - 50,
                color: "#0000FF".to_string(),
            })
        })
        .collect();

    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Maximum Block Level".to_string()),
        blocks,
    };

    let start = std::time::Instant::now();
    let result = generate_rbxlx(&space_json);
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(duration.as_secs() < 30, "10,000 blocks should generate in < 30 seconds");

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    let part_count = xml_str.matches("<Item class=\"Part\"").count();
    assert_eq!(part_count, 10_001, "Should have 10,001 Parts");
    assert!(xml.len() < 50_000_000, "File size should be < 50MB");
}

#[test]
fn test_various_colors() {
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
        let color = hex_to_color3(hex).expect(&format!("Color {} should parse", name));
        assert!((color.r - expected_r).abs() < 0.01, "Color {} R mismatch", name);
        assert!((color.g - expected_g).abs() < 0.01, "Color {} G mismatch", name);
        assert!((color.b - expected_b).abs() < 0.01, "Color {} B mismatch", name);
    }

    let blocks: Vec<Block> = color_tests
        .iter()
        .enumerate()
        .map(|(i, (hex, _, _, _, _))| Block {
            x: i as i32,
            y: 0,
            z: 0,
            color: hex.to_string(),
        })
        .collect();

    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Color Test Level".to_string()),
        blocks,
    };

    let result = generate_rbxlx(&space_json);
    assert!(result.is_ok());

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    let part_count = xml_str.matches("<Item class=\"Part\"").count();
    assert_eq!(part_count, 11, "Should have 11 Parts (baseplate + 10 color blocks)");
}

#[test]
fn test_coordinate_edge_cases() {
    let edge_blocks = vec![
        Block { x: -1000, y: 0, z: 0, color: "#FF0000".to_string() },
        Block { x: 1000, y: 0, z: 0, color: "#00FF00".to_string() },
        Block { x: 0, y: 0, z: -1000, color: "#0000FF".to_string() },
        Block { x: 0, y: 0, z: 1000, color: "#FFFF00".to_string() },
        Block { x: 0, y: 0, z: 0, color: "#FF00FF".to_string() },
        Block { x: 0, y: 1000, z: 0, color: "#00FFFF".to_string() },
    ];

    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Edge Case Level".to_string()),
        blocks: edge_blocks.clone(),
    };

    let result = generate_rbxlx(&space_json);
    assert!(result.is_ok());

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    let part_count = xml_str.matches("<Item class=\"Part\"").count();
    assert_eq!(part_count, 7, "Should have 7 Parts (baseplate + 6 edge blocks)");

    let (spawn_x, spawn_y, spawn_z) = calculate_spawn_position(&edge_blocks);
    assert_eq!(spawn_x, 0.0);
    assert_eq!(spawn_y, 1001.0);
    assert_eq!(spawn_z, 0.0);
}

#[test]
fn test_baseplate_creation() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Baseplate Test".to_string()),
        blocks: vec![],
    };
    let result = generate_rbxlx(&space_json);
    assert!(result.is_ok());

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    assert!(xml_str.contains("Baseplate"));
    assert!(xml_str.contains(r#"<string name="Name">Baseplate</string>"#));
}

#[test]
fn test_spawn_platform_fixed_position() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Empty Level".to_string()),
        blocks: vec![],
    };
    let result = generate_rbxlx(&space_json);
    assert!(result.is_ok());

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    // Spawn platform size should be 12x1x12
    assert!(
        xml_str.contains(r#"<X>12</X>"#) || xml_str.contains(r#"<X>12.0</X>"#),
        "Spawn platform X size should be 12"
    );
}

#[test]
fn test_spawn_platform_with_blocks() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Level with Blocks".to_string()),
        blocks: vec![
            Block { x: 10, y: 5, z: 20, color: "#FF0000".to_string() },
            Block { x: -10, y: 3, z: -20, color: "#00FF00".to_string() },
        ],
    };
    let result = generate_rbxlx(&space_json);
    assert!(result.is_ok());

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    // Spawn should still be at fixed position regardless of blocks
    assert!(xml_str.contains("SpawnLocation"));
}

#[test]
fn test_spawn_platform_decal() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Decal Test".to_string()),
        blocks: vec![],
    };
    let result = generate_rbxlx(&space_json);
    assert!(result.is_ok());

    let xml = result.unwrap();
    let xml_str = String::from_utf8_lossy(&xml);

    assert!(xml_str.contains("Decal"));
    assert!(xml_str.contains(r#"rbxasset://textures/SpawnLocation.png"#));

    // Verify texture is not serialized as string
    let decal_start = xml_str.find("<Item class=\"Decal\"").unwrap_or(0);
    let decal_section = &xml_str[decal_start..(decal_start + 500).min(xml_str.len())];
    assert!(
        !decal_section.contains(r#"<string name="Texture">"#),
        "Texture should be Content type, not string"
    );
}
