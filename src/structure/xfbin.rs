use binrw::{binrw, helpers::until_eof, helpers::until, NullString};

//use super::anm::BrAnm;

#[binrw]
#[derive(Debug, Default, Clone)]
pub struct Xfbin {
    pub header: NuccHeader,

    pub chunk_table: NuccChunkTable,

    // Read until the end of the file
    #[br(parse_with = until_eof)]
    pub pages: Vec<Page>
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
#[br(import(xfbin: Xfbin))] // Xfbin is the super parent struct. It contains the chunk table which we need to parse the chunks.
pub struct Chunk {
    pub size: u32,
    pub chunk_map_index: u32,
    pub version: u16,
    pub field0_a: u16,
    

    #[br(args(xfbin.chunk_table.chunk_maps[chunk_map_index as usize].clone()))]
    pub data: ChunkData
}


#[binrw]
#[br(import(chunk_map: ChunkMap))]
#[derive(Debug, Clone)]
pub enum ChunkData {
    
    NuccChunkNull {
        #[br(count = 0)]
        data: Vec<u8>
    },

    #[br(pre_assert(chunk_map.chunk_type_index == 2))]
    NuccChunkPage {
        page_size: u32,
        extra_indices_size: u32,
    },

    #[br(pre_assert(chunk_map.chunk_type_index == 1))]
    NuccChunkBinary {
        #[br(count = 60)]
        data: Vec<u8>
    },

    UnknownChunk {
        #[br(count = 0)]
        data: Vec<u8>
    }
    // Add more chunks here
}


#[binrw]
#[derive(Debug, Default, Clone)]
#[br(import(xfbin: Xfbin))]
pub struct Page {
    // Parse until we get a NuccChunkPage 
    #[br(args(xfbin), parse_with = until(|x: &Chunk| matches!(x.data, ChunkData::NuccChunkPage { .. } )))]
    pub chunks: Vec<Chunk>
}




#[binrw]
#[derive(Debug, Default, Clone)]
pub struct NuccChunkPage {
    pub page_size: u32,
    pub extra_indices_size: u32,
}


pub fn get_chunk_map_type(chunk_map: &ChunkMap, xfbin: &Xfbin) -> String {
    let chunk_map_type = get_chunk_type(chunk_map.chunk_type_index, xfbin);
    chunk_map_type


}

pub fn get_chunk_type(index: u32, xfbin: &Xfbin) -> String {
    let chunk_type = xfbin.chunk_table.chunk_types[index as usize].to_string();
    chunk_type
}