//! Space JSON data models.
//!
//! Defines structures for deserializing Space JSON from the frontend.
//! Schema corresponds to `frontend/src/export/serialize.ts`.

use serde::Deserialize;

/// Root object of a Space JSON payload.
///
/// Contains metadata and the list of blocks that make up a level.
#[derive(Debug, Deserialize)]
pub struct SpaceJSON {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    pub name: Option<String>,
    pub blocks: Vec<Block>,
}

/// A single voxel block with position and color.
///
/// Coordinates are in Roblox studs (already scaled 2x from Three.js units by frontend).
#[derive(Debug, Clone, Deserialize)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// Hex color string in `#RRGGBB` or `#RGB` format.
    pub color: String,
}
