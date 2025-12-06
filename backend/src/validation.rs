//! Space JSON validation.
//!
//! Validates incoming Space JSON payloads before `.rbxlx` generation.
//! All validators return structured errors with error codes and user-friendly messages.

use crate::models::SpaceJSON;
use std::collections::HashSet;

/// Validation error variants with associated context data.
#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidSchemaVersion { version: u32 },
    BlockCountExceeded { count: usize, limit: usize },
    CoordinateOutOfBounds { x: i32, y: i32, z: i32, index: usize },
    InvalidColorFormat { color: String, index: usize },
    DuplicatePosition { x: i32, y: i32, z: i32, index: usize },
}

impl ValidationError {
    /// Returns a machine-readable error code for API responses.
    pub fn error_code(&self) -> &'static str {
        match self {
            ValidationError::InvalidSchemaVersion { .. } => "INVALID_SCHEMA_VERSION",
            ValidationError::BlockCountExceeded { .. } => "BLOCK_COUNT_EXCEEDED",
            ValidationError::CoordinateOutOfBounds { .. } => "COORDINATE_OUT_OF_BOUNDS",
            ValidationError::InvalidColorFormat { .. } => "INVALID_COLOR_FORMAT",
            ValidationError::DuplicatePosition { .. } => "DUPLICATE_POSITION",
        }
    }

    /// Returns a human-readable error message for display to users.
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
                    "Block at position ({}, {}, {}) [index {}] is out of bounds. Valid range: X/Z: -1000 to 1000, Y: 0 to 1000.",
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

/// Validates that schema version is 1 (only supported version).
pub fn validate_schema_version(schema_version: u32) -> Result<(), ValidationError> {
    if schema_version != 1 {
        return Err(ValidationError::InvalidSchemaVersion { version: schema_version });
    }
    Ok(())
}

/// Maximum allowed blocks per level.
pub const MAX_BLOCKS: usize = 10_000;

/// Validates that block count does not exceed [`MAX_BLOCKS`].
pub fn validate_block_count(count: usize) -> Result<(), ValidationError> {
    if count > MAX_BLOCKS {
        return Err(ValidationError::BlockCountExceeded {
            count,
            limit: MAX_BLOCKS,
        });
    }
    Ok(())
}

// Coordinate bounds (in Roblox studs, already scaled 2x from Three.js units)
const MIN_X: i32 = -1000;
const MAX_X: i32 = 1000;
const MIN_Z: i32 = -1000;
const MAX_Z: i32 = 1000;
const MIN_Y: i32 = 0;
const MAX_Y: i32 = 1000;

/// Validates that block coordinates are within allowed bounds.
///
/// Bounds: X/Z: -1000 to 1000, Y: 0 to 1000 (in Roblox studs).
pub fn validate_coordinate_bounds(x: i32, y: i32, z: i32, index: usize) -> Result<(), ValidationError> {
    if x < MIN_X || x > MAX_X || z < MIN_Z || z > MAX_Z || y < MIN_Y || y > MAX_Y {
        return Err(ValidationError::CoordinateOutOfBounds { x, y, z, index });
    }
    Ok(())
}

/// Validates hex color format (`#RRGGBB` or `#RGB`).
pub fn validate_color_format(color: &str, index: usize) -> Result<(), ValidationError> {
    if !color.starts_with('#') {
        return Err(ValidationError::InvalidColorFormat {
            color: color.to_string(),
            index,
        });
    }

    let hex_part = &color[1..];
    if hex_part.len() != 3 && hex_part.len() != 6 {
        return Err(ValidationError::InvalidColorFormat {
            color: color.to_string(),
            index,
        });
    }

    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidColorFormat {
            color: color.to_string(),
            index,
        });
    }

    Ok(())
}

/// Validates that no two blocks occupy the same position.
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

/// Validates an entire Space JSON payload.
///
/// Performs all validations in order, returning the first error found (fail-fast):
/// 1. Schema version
/// 2. Block count
/// 3. Each block's coordinates and color
/// 4. No duplicate positions
pub fn validate_space_json(space_json: &SpaceJSON) -> Result<(), ValidationError> {
    validate_schema_version(space_json.schema_version)?;
    validate_block_count(space_json.blocks.len())?;

    for (index, block) in space_json.blocks.iter().enumerate() {
        validate_coordinate_bounds(block.x, block.y, block.z, index)?;
        validate_color_format(&block.color, index)?;
    }

    validate_no_duplicates(&space_json.blocks)?;
    Ok(())
}
