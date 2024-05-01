use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chunk {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub types: String,
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkMap {
    #[serde(rename = "Chunk Maps")]
    pub chunk_maps: Vec<Chunk>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkReference {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Chunk")]
    pub chunk: Chunk,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    #[serde(rename = "File Name")]
    pub file_name: String,
    #[serde(rename = "Chunk")]
    pub chunk: Chunk,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Page {
    #[serde(rename = "Chunk Maps")]
    pub chunk_maps: Vec<Chunk>,
    #[serde(rename = "Chunk References")]
    pub chunk_references: Vec<ChunkReference>,
    #[serde(rename = "Chunks")]
    pub files: Vec<Files>,
}

impl Page {
    pub fn from(filepath: &str) -> Page {
        let json_result = fs::read(filepath);

        let json_result = match json_result {
            Ok(json_result) => json_result,
            Err(_) => panic!("Failed to read JSON file"),
        };

        serde_json::from_slice(&json_result).unwrap()
    }

    pub fn to_json_file(&self, filepath: &str) {
        fs::File::create(filepath).unwrap();

        let json = serde_json::to_string_pretty(&self).unwrap();

        fs::write(filepath, json).unwrap();
    }
}
