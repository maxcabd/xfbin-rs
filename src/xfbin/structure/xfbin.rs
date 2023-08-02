use binrw::{binrw, helpers::until_eof, helpers::until, NullString};
use std::{collections::HashMap, vec};


#[binrw]
#[derive(Debug, Default, Clone)]
pub struct Xfbin {
    pub header: NuccHeader,

    pub chunk_table: NuccChunkTable,

    // Read until the end of the file
    #[br(parse_with = until_eof)]
    pub pages: Vec<Page>
}

impl Xfbin {
    pub fn set_chunk_types(&mut self) {
        let chunk_type_map: HashMap<u32, String> = self
            .chunk_table
            .chunk_maps
            .iter()
            .enumerate()
            .map(|(index, chunk_map)| (index as u32, get_chunk_map_type(chunk_map, self)))
            .collect();

       
        for page in self.pages.iter_mut() {
            for chunk in page.chunks.iter_mut() {
                let chunk_type = chunk_type_map
                    .get(&chunk.chunk_map_index)
                    .cloned()
                    .unwrap_or_else(|| String::from("Unknown"));

                chunk.chunk_type = chunk_type;
            }
        }
    }

    
    pub fn get_chunk_by_type(&self, chunk_type: &str) -> Vec<Chunk> {
        let mut chunks: Vec<Chunk> = Vec::new();
        for page in &self.pages {
            for chunk in &page.chunks {
                if chunk.chunk_type == chunk_type.to_string() {
                    chunks.push(chunk.clone());
                }
            }
        }
        chunks
    }
}


#[binrw]
#[brw(magic = b"NUCC")]
#[derive(Debug, Default, Clone)]
pub struct NuccHeader {
    pub version: u32,

    #[brw(pad_before = 8)]
    pub chunk_table_size: u32,
    pub min_page_size: u32,
    pub version1: u16,
    pub field1_a: u16,
}

#[binrw]
#[derive(Debug, Default, Clone)]
pub struct NuccChunkTable {
    pub chunk_type_count: u32,
    pub chunk_type_size: u32,

    pub filepath_count: u32,
    pub filepath_size: u32,

    pub chunk_name_count: u32,
    pub chunk_name_size: u32,

    pub chunk_map_count: u32,
    pub chunk_map_size: u32,

    pub chunk_map_indices_count: u32,
    pub chunk_map_references_count: u32,

    #[br(count = chunk_type_count)]
    pub chunk_types: Vec<NullString>,

    #[br(count = filepath_count)]
    pub filepaths: Vec<NullString>,

    #[br(count = chunk_name_count)]
    pub chunk_names: Vec<NullString>,

    #[brw(align_before = 4)]
    #[br(count = chunk_map_count)]
    pub chunk_maps: Vec<ChunkMap>,

    #[br(count = chunk_map_references_count)]
    pub chunk_map_references: Vec<ChunkReference>,

    #[br(count = chunk_map_indices_count)]
    pub chunk_map_indices: Vec<u32>
}

impl NuccChunkTable {
    pub fn get_chunk_info(&self, chunk_map_index: u32) -> (String, String, String) {
        let chunk_map = &self.chunk_maps[chunk_map_index as usize];
        let chunk_type = self.chunk_types[chunk_map.chunk_type_index as usize].to_string();
        let filepath = self.filepaths[chunk_map.filepath_index as usize].to_string();
        let chunk_name = self.chunk_names[chunk_map.chunk_name_index as usize].to_string();

        (chunk_type, filepath, chunk_name)
    }
}


#[binrw]
#[derive(Debug, Clone)]
pub struct ChunkMap {
    pub chunk_type_index: u32,
    pub filepath_index: u32,
    pub chunk_name_index: u32,
}


#[binrw]
#[derive(Debug, Clone)]
pub struct ChunkReference {
    pub chunk_name_index: u32,
    pub chunk_map_index: u32,
}


#[binrw]
#[derive(Debug, Clone)]
#[br(import_raw(xfbin: Xfbin))]
pub struct Chunk {
    pub size: u32,
    pub chunk_map_index: u32,
    pub version: u16,
    pub field0_a: u16,
    
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    #[br(calc = Self::parse_chunk_type(chunk_map_index, &xfbin))]
    pub chunk_type: String,


    #[br(args(chunk_map_index, size, chunk_type.clone()))]
    pub data: ChunkData
}

impl Chunk {
    fn parse_chunk_type(chunk_map_index: u32, xfbin: &Xfbin) -> String {
        if let Some(chunk_map) = xfbin.chunk_table.chunk_maps.get(chunk_map_index as usize) {
            let chunk_type_index = chunk_map.chunk_type_index as usize;
            if let Some(chunk_type) = xfbin.chunk_table.chunk_types.get(chunk_type_index) {
                return chunk_type.to_string().clone();
            }

            return String::from("Unknown");
        }

        String::from("Unknown")
    }
}


#[binrw]
#[br(import(chunk_map_index: u32, size: u32, chunk_type: String))]
#[derive(Debug, Clone)]
pub enum ChunkData {

    #[br(pre_assert(chunk_type == "nuccChunkNull"))]
    NuccChunkNull {
        #[br(count = size)]
        data: Vec<u8>
    },

    #[br(pre_assert(chunk_type == "nuccChunkBinary"))]
    NuccChunkBinary(#[br(count = size)] Vec<u8>),

    #[br(pre_assert(chunk_map_index == 2))]
    NuccChunkPage {
        #[br(count = size)]
        data: Vec<u8>
    },

    UnknownChunk {
        #[br(count = size)]
        data: Vec<u8>
    }
    // Add more chunks here
}

impl ChunkData {
    pub fn get_data(&self) -> Vec<u8> {
        match self {
            ChunkData::NuccChunkNull { data } => data.clone(),
            ChunkData::NuccChunkBinary(data) => data.clone(),
            ChunkData::NuccChunkPage { data } => data.clone(),
            ChunkData::UnknownChunk { data } => data.clone(),
        }
    }
}





#[binrw]
#[derive(Debug, Default, Clone)]
pub struct Page {
    // Parse until we get a NuccChunkPage 
    #[br(parse_with = until(|chunk: &Chunk| matches!(chunk.data, ChunkData::NuccChunkPage { .. } )))]
    pub chunks: Vec<Chunk>
}


#[binrw]
#[derive(Debug, Default, Clone)]
pub struct NuccChunkPage {
    pub page_size: u32,
    pub extra_indices_size: u32,
}


pub fn get_chunk_map_type(chunk_map: &ChunkMap, xfbin: &Xfbin) -> String {
    let chunk_type = get_chunk_type(chunk_map.chunk_type_index, xfbin);
    chunk_type


}

pub fn get_chunk_type(index: u32, xfbin: &Xfbin) -> String {
    let chunk_type = xfbin
        .chunk_table
        .chunk_types
        .get(index as usize)
        .map(|null_string| null_string.to_string())
        .unwrap_or_else(|| String::from("Unknown"));
    chunk_type
}

