//! Tests for Space JSON validation.

use backend::models::{Block, SpaceJSON};
use backend::validation::{
    validate_block_count, validate_color_format, validate_coordinate_bounds,
    validate_no_duplicates, validate_schema_version, validate_space_json, ValidationError,
    MAX_BLOCKS,
};

// Schema version tests

#[test]
fn test_validate_schema_version_accepts_1() {
    assert!(validate_schema_version(1).is_ok());
}

#[test]
fn test_validate_schema_version_rejects_0() {
    assert!(matches!(
        validate_schema_version(0),
        Err(ValidationError::InvalidSchemaVersion { version: 0 })
    ));
}

#[test]
fn test_validate_schema_version_rejects_2() {
    assert!(matches!(
        validate_schema_version(2),
        Err(ValidationError::InvalidSchemaVersion { version: 2 })
    ));
}

// Block count tests

#[test]
fn test_validate_block_count_accepts_max() {
    assert!(validate_block_count(MAX_BLOCKS).is_ok());
}

#[test]
fn test_validate_block_count_accepts_0() {
    assert!(validate_block_count(0).is_ok());
}

#[test]
fn test_validate_block_count_rejects_over_max() {
    assert!(matches!(
        validate_block_count(MAX_BLOCKS + 1),
        Err(ValidationError::BlockCountExceeded { count, limit }) if count == MAX_BLOCKS + 1 && limit == MAX_BLOCKS
    ));
}

// Coordinate bounds tests

#[test]
fn test_validate_coordinate_bounds_accepts_valid() {
    assert!(validate_coordinate_bounds(0, 500, 0, 0).is_ok());
    assert!(validate_coordinate_bounds(-1000, 0, -1000, 0).is_ok());
    assert!(validate_coordinate_bounds(1000, 1000, 1000, 0).is_ok());
}

#[test]
fn test_validate_coordinate_bounds_rejects_x_too_low() {
    assert!(matches!(
        validate_coordinate_bounds(-1001, 0, 0, 0),
        Err(ValidationError::CoordinateOutOfBounds { x: -1001, .. })
    ));
}

#[test]
fn test_validate_coordinate_bounds_rejects_x_too_high() {
    assert!(matches!(
        validate_coordinate_bounds(1001, 0, 0, 0),
        Err(ValidationError::CoordinateOutOfBounds { x: 1001, .. })
    ));
}

#[test]
fn test_validate_coordinate_bounds_rejects_z_too_low() {
    assert!(matches!(
        validate_coordinate_bounds(0, 0, -1001, 0),
        Err(ValidationError::CoordinateOutOfBounds { z: -1001, .. })
    ));
}

#[test]
fn test_validate_coordinate_bounds_rejects_z_too_high() {
    assert!(matches!(
        validate_coordinate_bounds(0, 0, 1001, 0),
        Err(ValidationError::CoordinateOutOfBounds { z: 1001, .. })
    ));
}

#[test]
fn test_validate_coordinate_bounds_rejects_y_too_low() {
    assert!(matches!(
        validate_coordinate_bounds(0, -1, 0, 0),
        Err(ValidationError::CoordinateOutOfBounds { y: -1, .. })
    ));
}

#[test]
fn test_validate_coordinate_bounds_rejects_y_too_high() {
    assert!(matches!(
        validate_coordinate_bounds(0, 1001, 0, 0),
        Err(ValidationError::CoordinateOutOfBounds { y: 1001, .. })
    ));
}

// Color format tests

#[test]
fn test_validate_color_format_accepts_rrggbb() {
    assert!(validate_color_format("#FF0000", 0).is_ok());
    assert!(validate_color_format("#00FF00", 0).is_ok());
    assert!(validate_color_format("#0000FF", 0).is_ok());
    assert!(validate_color_format("#ABCDEF", 0).is_ok());
    assert!(validate_color_format("#123456", 0).is_ok());
}

#[test]
fn test_validate_color_format_accepts_rgb() {
    assert!(validate_color_format("#F00", 0).is_ok());
    assert!(validate_color_format("#0F0", 0).is_ok());
    assert!(validate_color_format("#00F", 0).is_ok());
    assert!(validate_color_format("#ABC", 0).is_ok());
}

#[test]
fn test_validate_color_format_rejects_missing_hash() {
    assert!(matches!(
        validate_color_format("FF0000", 0),
        Err(ValidationError::InvalidColorFormat { .. })
    ));
}

#[test]
fn test_validate_color_format_rejects_wrong_length() {
    assert!(matches!(
        validate_color_format("#FF00", 0),
        Err(ValidationError::InvalidColorFormat { .. })
    ));
    assert!(matches!(
        validate_color_format("#FF00000", 0),
        Err(ValidationError::InvalidColorFormat { .. })
    ));
}

#[test]
fn test_validate_color_format_rejects_non_hex() {
    assert!(matches!(
        validate_color_format("#GGGGGG", 0),
        Err(ValidationError::InvalidColorFormat { .. })
    ));
    assert!(matches!(
        validate_color_format("#XYZ", 0),
        Err(ValidationError::InvalidColorFormat { .. })
    ));
}

// Duplicate position tests

#[test]
fn test_validate_no_duplicates_accepts_unique() {
    let blocks = vec![
        Block { x: 0, y: 0, z: 0, color: "#FF0000".to_string() },
        Block { x: 1, y: 0, z: 0, color: "#00FF00".to_string() },
        Block { x: 0, y: 1, z: 0, color: "#0000FF".to_string() },
    ];
    assert!(validate_no_duplicates(&blocks).is_ok());
}

#[test]
fn test_validate_no_duplicates_rejects_duplicate() {
    let blocks = vec![
        Block { x: 0, y: 0, z: 0, color: "#FF0000".to_string() },
        Block { x: 1, y: 0, z: 0, color: "#00FF00".to_string() },
        Block { x: 0, y: 0, z: 0, color: "#0000FF".to_string() },
    ];
    assert!(matches!(
        validate_no_duplicates(&blocks),
        Err(ValidationError::DuplicatePosition { x: 0, y: 0, z: 0, index: 2 })
    ));
}

// Full Space JSON validation tests

#[test]
fn test_validate_space_json_passes_valid() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Test Level".to_string()),
        blocks: vec![
            Block { x: 0, y: 0, z: 0, color: "#FF0000".to_string() },
            Block { x: 1, y: 0, z: 0, color: "#00FF00".to_string() },
        ],
    };
    assert!(validate_space_json(&space_json).is_ok());
}

#[test]
fn test_validate_space_json_fails_invalid_schema() {
    let space_json = SpaceJSON {
        schema_version: 2,
        name: Some("Test Level".to_string()),
        blocks: vec![],
    };
    assert!(matches!(
        validate_space_json(&space_json),
        Err(ValidationError::InvalidSchemaVersion { version: 2 })
    ));
}

#[test]
fn test_validate_space_json_fails_block_count_exceeded() {
    let blocks: Vec<Block> = (0..MAX_BLOCKS + 1)
        .map(|i| Block {
            x: i as i32,
            y: 0,
            z: 0,
            color: "#FF0000".to_string(),
        })
        .collect();
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Test Level".to_string()),
        blocks,
    };
    assert!(matches!(
        validate_space_json(&space_json),
        Err(ValidationError::BlockCountExceeded { .. })
    ));
}

#[test]
fn test_validate_space_json_fails_out_of_bounds() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Test Level".to_string()),
        blocks: vec![Block {
            x: 1001,
            y: 0,
            z: 0,
            color: "#FF0000".to_string(),
        }],
    };
    assert!(matches!(
        validate_space_json(&space_json),
        Err(ValidationError::CoordinateOutOfBounds { x: 1001, .. })
    ));
}

#[test]
fn test_validate_space_json_fails_invalid_color() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Test Level".to_string()),
        blocks: vec![Block {
            x: 0,
            y: 0,
            z: 0,
            color: "not-a-color".to_string(),
        }],
    };
    assert!(matches!(
        validate_space_json(&space_json),
        Err(ValidationError::InvalidColorFormat { .. })
    ));
}

#[test]
fn test_validate_space_json_fails_duplicate() {
    let space_json = SpaceJSON {
        schema_version: 1,
        name: Some("Test Level".to_string()),
        blocks: vec![
            Block { x: 0, y: 0, z: 0, color: "#FF0000".to_string() },
            Block { x: 0, y: 0, z: 0, color: "#00FF00".to_string() },
        ],
    };
    assert!(matches!(
        validate_space_json(&space_json),
        Err(ValidationError::DuplicatePosition { x: 0, y: 0, z: 0, .. })
    ));
}
