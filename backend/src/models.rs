/**
 * Phase 5: Space JSON Models
 * 
 * Defines the data structures for deserializing Space JSON from the frontend.
 * Matches the TypeScript interfaces in frontend/src/export/serialize.ts
 */

use serde::Deserialize;

/// Space JSON root object
#[derive(Debug, Deserialize)]
pub struct SpaceJSON {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    pub name: Option<String>,
    pub blocks: Vec<Block>,
}

/// Block object representing a single voxel
#[derive(Debug, Deserialize)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub color: String,
}

