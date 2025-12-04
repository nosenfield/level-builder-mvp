/**
 * Phase 6: Backend Validation
 * 
 * Validates Space JSON input before processing.
 * All validations return structured errors for user-friendly feedback.
 */

use crate::models::SpaceJSON;
use std::collections::HashSet;

/// Validation error types
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidSchemaVersion { version: u32 },
    BlockCountExceeded { count: usize, limit: usize },
    CoordinateOutOfBounds { x: i32, y: i32, z: i32, index: usize },
    InvalidColorFormat { color: String, index: usize },
    DuplicatePosition { x: i32, y: i32, z: i32, index: usize },
}

impl ValidationError {
    /// Convert to error code string
    pub fn error_code(&self) -> &'static str {
        match self {
            ValidationError::InvalidSchemaVersion { .. } => "INVALID_SCHEMA_VERSION",
            ValidationError::BlockCountExceeded { .. } => "BLOCK_COUNT_EXCEEDED",
            ValidationError::CoordinateOutOfBounds { .. } => "COORDINATE_OUT_OF_BOUNDS",
            ValidationError::InvalidColorFormat { .. } => "INVALID_COLOR_FORMAT",
            ValidationError::DuplicatePosition { .. } => "DUPLICATE_POSITION",
        }
    }

    /// Convert to user-friendly error message
    pub fn message(&self) -> String {
        match self {
            ValidationError::InvalidSchemaVersion { version } => {
                format!("Unsupported schema version: {}. Only version 1 is supported.", version)
            }
            ValidationError::BlockCountExceeded { count, limit } => {
                format!("Block count ({}) exceeds maximum allowed ({}).", count, limit)
            }
            ValidationError::CoordinateOutOfBounds { x, y, z, index } => {
                format!(
                    "Block at position ({}, {}, {}) [index {}] is out of bounds. Valid range: X/Z: -500 to 500, Y: 0 to 500.",
                    x, y, z, index
                )
            }
            ValidationError::InvalidColorFormat { color, index } => {
                format!(
                    "Invalid color format '{}' at block index {}. Expected hex format: #RRGGBB or #RGB.",
                    color, index
                )
            }
            ValidationError::DuplicatePosition { x, y, z, index } => {
                format!(
                    "Duplicate block position ({}, {}, {}) found at index {}. Each block must have a unique position.",
                    x, y, z, index
                )
            }
        }
    }
}

/// Validate schema version (must be 1)
pub fn validate_schema_version(schema_version: u32) -> Result<(), ValidationError> {
    if schema_version != 1 {
        return Err(ValidationError::InvalidSchemaVersion { version: schema_version });
    }
    Ok(())
}

/// Validate block count (must be <= 10,000)
pub fn validate_block_count(count: usize) -> Result<(), ValidationError> {
    const MAX_BLOCKS: usize = 10_000;
    if count > MAX_BLOCKS {
        return Err(ValidationError::BlockCountExceeded {
            count,
            limit: MAX_BLOCKS,
        });
    }
    Ok(())
}

/// Validate coordinate bounds for a single block
/// X/Z: -500 to 500 (inclusive)
/// Y: 0 to 500 (inclusive)
pub fn validate_coordinate_bounds(x: i32, y: i32, z: i32, index: usize) -> Result<(), ValidationError> {
    const MIN_X: i32 = -500;
    const MAX_X: i32 = 500;
    const MIN_Z: i32 = -500;
    const MAX_Z: i32 = 500;
    const MIN_Y: i32 = 0;
    const MAX_Y: i32 = 500;

    if x < MIN_X || x > MAX_X || z < MIN_Z || z > MAX_Z || y < MIN_Y || y > MAX_Y {
        return Err(ValidationError::CoordinateOutOfBounds { x, y, z, index });
    }
    Ok(())
}

/// Validate color format (hex pattern: #RRGGBB or #RGB)
pub fn validate_color_format(color: &str, index: usize) -> Result<(), ValidationError> {
    // Check if starts with #
    if !color.starts_with('#') {
        return Err(ValidationError::InvalidColorFormat {
            color: color.to_string(),
            index,
        });
    }

    // Check length (must be 4 or 7: #RGB or #RRGGBB)
    let hex_part = &color[1..];
    if hex_part.len() != 3 && hex_part.len() != 6 {
        return Err(ValidationError::InvalidColorFormat {
            color: color.to_string(),
            index,
        });
    }

    // Check all characters are valid hex digits
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidColorFormat {
            color: color.to_string(),
            index,
        });
    }

    Ok(())
}

/// Validate no duplicate positions
pub fn validate_no_duplicates(blocks: &[crate::models::Block]) -> Result<(), ValidationError> {
    let mut positions = HashSet::new();
    
    for (index, block) in blocks.iter().enumerate() {
        let position = (block.x, block.y, block.z);
        if positions.contains(&position) {
            return Err(ValidationError::DuplicatePosition {
                x: block.x,
                y: block.y,
                z: block.z,
                index,
            });
        }
        positions.insert(position);
    }
    
    Ok(())
}

/// Validate entire Space JSON structure
/// Returns first validation error found (fail-fast)
pub fn validate_space_json(space_json: &SpaceJSON) -> Result<(), ValidationError> {
    // 1. Validate schema version first
    validate_schema_version(space_json.schema_version)?;

    // 2. Validate block count
    validate_block_count(space_json.blocks.len())?;

    // 3. Validate each block
    for (index, block) in space_json.blocks.iter().enumerate() {
        // 3a. Validate coordinate bounds
        validate_coordinate_bounds(block.x, block.y, block.z, index)?;

        // 3b. Validate color format
        validate_color_format(&block.color, index)?;
    }

    // 4. Validate no duplicate positions (check after individual validations)
    validate_no_duplicates(&space_json.blocks)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Block;

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

    #[test]
    fn test_validate_block_count_accepts_10000() {
        assert!(validate_block_count(10_000).is_ok());
    }

    #[test]
    fn test_validate_block_count_accepts_0() {
        assert!(validate_block_count(0).is_ok());
    }

    #[test]
    fn test_validate_block_count_rejects_10001() {
        assert!(matches!(
            validate_block_count(10_001),
            Err(ValidationError::BlockCountExceeded { count: 10_001, limit: 10_000 })
        ));
    }

    #[test]
    fn test_validate_coordinate_bounds_accepts_valid() {
        assert!(validate_coordinate_bounds(0, 250, 0, 0).is_ok());
        assert!(validate_coordinate_bounds(-500, 0, -500, 0).is_ok());
        assert!(validate_coordinate_bounds(500, 500, 500, 0).is_ok());
    }

    #[test]
    fn test_validate_coordinate_bounds_rejects_x_too_low() {
        assert!(matches!(
            validate_coordinate_bounds(-501, 0, 0, 0),
            Err(ValidationError::CoordinateOutOfBounds { x: -501, .. })
        ));
    }

    #[test]
    fn test_validate_coordinate_bounds_rejects_x_too_high() {
        assert!(matches!(
            validate_coordinate_bounds(501, 0, 0, 0),
            Err(ValidationError::CoordinateOutOfBounds { x: 501, .. })
        ));
    }

    #[test]
    fn test_validate_coordinate_bounds_rejects_z_too_low() {
        assert!(matches!(
            validate_coordinate_bounds(0, 0, -501, 0),
            Err(ValidationError::CoordinateOutOfBounds { z: -501, .. })
        ));
    }

    #[test]
    fn test_validate_coordinate_bounds_rejects_z_too_high() {
        assert!(matches!(
            validate_coordinate_bounds(0, 0, 501, 0),
            Err(ValidationError::CoordinateOutOfBounds { z: 501, .. })
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
            validate_coordinate_bounds(0, 501, 0, 0),
            Err(ValidationError::CoordinateOutOfBounds { y: 501, .. })
        ));
    }

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
            Block { x: 0, y: 0, z: 0, color: "#0000FF".to_string() }, // duplicate
        ];
        assert!(matches!(
            validate_no_duplicates(&blocks),
            Err(ValidationError::DuplicatePosition { x: 0, y: 0, z: 0, index: 2 })
        ));
    }

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
        let mut blocks = Vec::new();
        for i in 0..10_001 {
            blocks.push(Block {
                x: i as i32,
                y: 0,
                z: 0,
                color: "#FF0000".to_string(),
            });
        }
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Test Level".to_string()),
            blocks,
        };
        assert!(matches!(
            validate_space_json(&space_json),
            Err(ValidationError::BlockCountExceeded { count: 10_001, limit: 10_000 })
        ));
    }

    #[test]
    fn test_validate_space_json_fails_out_of_bounds() {
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Test Level".to_string()),
            blocks: vec![
                Block { x: 501, y: 0, z: 0, color: "#FF0000".to_string() },
            ],
        };
        assert!(matches!(
            validate_space_json(&space_json),
            Err(ValidationError::CoordinateOutOfBounds { x: 501, .. })
        ));
    }

    #[test]
    fn test_validate_space_json_fails_invalid_color() {
        let space_json = SpaceJSON {
            schema_version: 1,
            name: Some("Test Level".to_string()),
            blocks: vec![
                Block { x: 0, y: 0, z: 0, color: "not-a-color".to_string() },
            ],
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
}

