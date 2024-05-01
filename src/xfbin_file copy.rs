use std::io::{Read, Seek};
use std::collections::HashMap;
use std::mem::size_of;
use anyhow::{Context, Result};
use binrw::{binrw, BinReaderExt, NullString, BinResult};



use super::nucc::*;


#[binrw]
#[derive(Debug, Clone, Default)]
pub struct XfbinFile {
    pub header: NuccHeader,
    pub chunk_table: NuccChunkTable,
    #[br(count = 0)]
    pub pages: Vec<XfbinPage>
}

impl XfbinFile {
    pub fn read<R: Read + Seek>(mut reader: R) -> BinResult<Self>

    where
        R: Read + Seek,
    { 
        let header = reader.read_be::<NuccHeader>()?;
        let chunk_table = reader.read_be::<NuccChunkTable>()?;

        let mut current_page_start = 0;
        let mut current_reference_start = 0;
        
        // Add the pages to the pages vector until we reach the end of the file
        let mut pages: Vec<XfbinPage> = Vec::new();
        loop {
            match XfbinPage::read_page(reader.by_ref(), &chunk_table, current_page_start, current_reference_start) {
                Ok(page) => {
                    pages.push(page);
                    // Get the the last chunk's page size (always a NuccPage chunk) and add it to the current page start
                    let nucc_page = pages.last().unwrap();
                    let page_size = nucc_page.get_page_size();
                    let reference_size = nucc_page.get_reference_size();

                    current_page_start += page_size;
                    current_reference_start += reference_size;
                }
                Err(ref e) if e.is_eof() => {
                    break; // End of file reached, exit the loop
                }
                Err(e) => return Err(e.into())
            }
        }
        
        Ok(Self { header, chunk_table, pages })
    }

    /*pub fn get_type_chunk_map(&self) -> HashMap<&str, Vec<&Chunk>> {
        let mut map: HashMap<&str, Vec<&Chunk>> = HashMap::new();

        for page in &self.pages {
            for chunk in &page.chunks {
                let chunk_type = chunk.chunk_type.as_str();
                if map.contains_key(chunk_type) {
                    map.get_mut(chunk_type).unwrap().push(chunk);
                } else {
                    map.insert(chunk_type, vec![chunk]);
                }
            }
        }

        return map
    }

    pub fn get_page_chunk_map(&self) -> HashMap<String, Vec<Chunk>> {
        let mut map: HashMap<String, Vec<Chunk>> = HashMap::new();
    
        for (i, page) in self.pages.iter().enumerate() {
            let mut chunks: Vec<Chunk> = Vec::new();
            let page_name = format!("XfbinPage{}", i);
    
            for chunk in &page.chunks {
                if chunk.chunk_type != "nuccChunkPage" && chunk.chunk_type != "nuccChunkNull" {
                    chunks.push(chunk.clone());
                }
            }
    
            map.insert(page_name, chunks);
        }
    
        return map
    }*/

    pub fn get_chunks_by_type(&self, chunk_type: &str) -> Vec<&Chunk> {
        let mut chunks: Vec<&Chunk> = Vec::new();

        for page in &self.pages {
            for chunk in &page.chunks {
                let chunk_map = self.chunk_table.chunk_maps[chunk.chunk_map_index as usize].clone();
                let _chunk_type = get_chunk_type(chunk_map.chunk_type_index, &self.chunk_table);
                if _chunk_type == chunk_type {
                    chunks.push(chunk);
                }
            }
        }
        
        return chunks
    }

    pub fn get_pages_by_type(&self, chunk_type: &str) -> Vec<&XfbinPage> {
        let mut pages: Vec<&XfbinPage> = Vec::new();

        for page in &self.pages {
            if page.get_chunks_by_type(chunk_type, &self.chunk_table).len() > 0 {
                pages.push(page);
            }
        }

        return pages
    }


    pub fn clear_pages(&mut self) {
        self.pages.clear();
    }

    pub fn get_chunk_page(&self, chunk: &Chunk) -> (u32, &XfbinPage) {
        for (i, page) in self.pages.iter().enumerate() {
            if page.chunks.contains(chunk) {
                return (i as u32, page)
            }
        }

        return (1, &self.pages[0])
    }

    pub fn update_chunk<'a>(&mut self, chunk: &'a Chunk) -> Option<Vec<&'a Chunk>> {
        let mut chunks: Vec<&'a Chunk> = Vec::new();
        chunks.push(chunk);

        let existing_page = self.get_chunk_page(chunk);
    
        let index = existing_page.0 as usize;
        let page = existing_page.1;
        self.pages[index] = page.clone();

        return Some(chunks)   
    }
    
    pub fn add_chunk_page(&mut self, chunk: &Chunk) -> Result<&XfbinPage, anyhow::Error> {
        let result = self.update_chunk(chunk);

        if let Some(chunks) = result {
            let page = self.pages.last_mut().unwrap();
            for chunk in chunks {
                page.add_chunk(chunk.clone());
            }
            return self.pages.last().context("Failed to add chunk page")

        } else {
            Err(anyhow::Error::msg("Failed to update chunk"))
        }
    }
}

#[binrw]
#[brw(magic = b"NUCC")]
#[derive(Debug, Clone, Default)]
pub struct NuccHeader {
    #[bw(calc = 0x79)] // Just set to 121
    pub version: u32,
    #[brw(pad_after = 6)]
    pub encrypted: u16,

    
    
}

#[binrw]
#[derive(Debug, Clone, Default)]
pub struct NuccChunkTable {

    #[bw(calc = self.get_chunk_table_size())]
    pub chunk_table_size: u32,

    #[bw(calc = 0x3)] // Usually 0x3
    pub min_page_size: u32,

    #[bw(calc = 0x79)] 
    pub version: u16,

    unknown: u16,

    #[bw(calc = self.chunk_types.len() as u32)] 
    pub chunk_type_count: u32,

    #[bw(calc = self.chunk_types.iter().map(|x| (x.len() + 1) as u32).sum())]
    pub chunk_type_size: u32,

    #[bw(calc = self.filepaths.len() as u32)]
    pub filepath_count: u32,

    #[bw(calc = self.filepaths.iter().map(|x| (x.len() + 1) as u32).sum())]
    pub filepath_size: u32,

    #[bw(calc = self.chunk_names.len() as u32)]
    pub chunk_name_count: u32,

    #[bw(calc = self.chunk_names.iter().map(|x| (x.len() + 1) as u32).sum())]
    pub chunk_name_size: u32,

    
    #[bw(calc = self.chunk_maps.len() as u32)]
    pub chunk_map_count: u32,

    #[bw(calc = (self.chunk_maps.len() * size_of::<ChunkMap>()) as u32)]
    pub chunk_map_size: u32,

    #[bw(calc = self.chunk_map_indices.len() as u32)]
    pub chunk_map_indices_count: u32,

    #[bw(calc = self.chunk_map_references.len() as u32)]
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

    /// Calculate the size of the chunk table for writing
    pub fn get_chunk_table_size(&self) -> u32 {
        let chunk_types_size: u32 = self.chunk_types.iter().map(|x| (x.len() + 1) as u32).sum();
        let filepaths_size: u32 = self.filepaths.iter().map(|x|  (x.len() + 1) as u32).sum();
        let chunk_names_size: u32 = self.chunk_names.iter().map(|x|  (x.len() + 1) as u32).sum();

        let string_sizes = chunk_types_size + filepaths_size + chunk_names_size;
 
        0x28 + string_sizes + (4 - (string_sizes % 4)) // Add the header size, size of the strings buffer, and the aligned size of the strings
        + (self.chunk_maps.len() as u32 * size_of::<ChunkMap>() as u32)
        + (self.chunk_map_indices.len() as u32 * size_of::<u32>() as u32)
    
        
    }

    /// Get the chunk type, filepath, and chunk name from the chunk map
    pub fn get_chunk_info(&self, chunk_map: &ChunkMap) -> (String, String, String) {
        
        let chunk_type = get_chunk_type(chunk_map.chunk_type_index, self);
        let filepath = get_chunk_filepath(chunk_map.filepath_index, self);
        let chunk_name = get_chunk_name(chunk_map.chunk_name_index, self);

        return (chunk_type, filepath, chunk_name)
    }
}

#[binrw]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChunkMap {
    pub chunk_type_index: u32,
    pub filepath_index: u32,
    pub chunk_name_index: u32,
}

#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkReference {
    pub chunk_name_index: u32,
    pub chunk_map_index: u32,
}

#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    #[bw(calc = self.get_chunk_size())]
    pub size: u32,
    
    pub chunk_map_index: u32,
    pub major_version: u16,
    pub minor_version: u16,

    pub data: NuccChunk
}

impl Chunk {
    fn read_chunk<R: Read + Seek>(mut reader: R, chunk_table: &NuccChunkTable, page_start: u32) -> BinResult<Self>

    where
        R: Read + Seek,
    {   
        let size = reader.read_be::<u32>()?;
        let chunk_map_index = reader.read_be::<u32>()?;

        let chunk_map = chunk_table.chunk_maps[get_chunk_map_index(chunk_map_index, page_start, chunk_table) as usize].clone();
        
        let chunk_type = chunk_table.get_chunk_info(&chunk_map).0;
        

        let major_version = reader.read_be::<u16>()?;
        let minor_version = reader.read_be::<u16>()?; 
        let data = Self::read_chunk_data(size, chunk_type.as_str(), reader.by_ref())?;

        let chunk = Chunk {
            chunk_map_index,
            major_version,
            minor_version,
            data
        };
   
        Ok(chunk)
    }

    fn read_chunk_data<R: Read + Seek>(size: u32, chunk_type: &str, reader: &mut R) -> BinResult<NuccChunk> {

        match chunk_type {
            "nuccChunkAnm" => Ok(NuccChunk::NuccChunkAnm(reader.read_be::<NuccAnm>()?)),
            "nuccChunkAnmStrm" => Ok(NuccChunk::NuccChunkAnmStrm(reader.read_be::<NuccAnmStrm>()?)),  
            "nuccChunkAnmStrmFrame" => Ok(NuccChunk::NuccChunkAnmStrmFrame(reader.read_be::<NuccAnmStrmFrame>()?)), 
            "nuccChunkCamera" => Ok(NuccChunk::NuccChunkCamera(reader.read_be::<NuccCamera>()?)),
            "nuccChunkBinary" => {
                let mut bytes = vec![0; size as usize];
                reader.read_exact(&mut bytes)?;
                Ok(NuccChunk::NuccChunkBinary(bytes))
            }

            "nuccChunkNub" => {
                let mut bytes = vec![0; size as usize];
                reader.read_exact(&mut bytes)?;
                Ok(NuccChunk::NuccChunkNub(bytes))
            }

             
            "nuccChunkPage" => Ok(NuccChunk::NuccChunkPage(reader.read_be::<NuccPage>()?)),

            _ => {
                // Read the unknown chunk data as a byte array
                let mut bytes = vec![0; size as usize];
                reader.read_exact(&mut bytes)?;
                Ok(NuccChunk::NuccChunkUnknown(bytes))
            }
        }
    }

    pub fn get_chunk_size(&self) -> u32 {
        let data = &self.data;

        let size = match data {
           
            NuccChunk::NuccChunkBinary(_) => data.as_bytes().len() as u32,
            NuccChunk::NuccChunkPage(_) => size_of::<NuccPage>() as u32,
            NuccChunk::NuccChunkCamera(_) => size_of::<NuccCamera>() as u32,
            NuccChunk::NuccChunkUnknown(_) => data.as_bytes().len() as u32,
            _ => { let bytes = data.as_bytes(); bytes.len() as u32 }
        };


        return size

    }
}

#[binrw]
#[derive(Debug, Clone, Default)]
pub struct XfbinPage {
    #[br(count = 0)]
    pub chunks: Vec<Chunk>,

    #[brw(ignore)]
    pub chunk_references: Vec<ChunkReference> // Store the chunk references for later use
}

impl XfbinPage {
    fn read_page<R: Read + Seek>(
        mut reader: R, chunk_table: &NuccChunkTable, current_page_start: u32, current_reference_start: u32
    ) -> BinResult<Self>

    where
        R: Read + Seek,
    {   
        let page_start = current_page_start;
        let reference_start = current_reference_start;

        let mut page = reader.read_be::<XfbinPage>()?;

        let mut chunks: Vec<Chunk> = Vec::new();

        loop { // Keep reading chunks until we reach the end of the page which is when the chunk_type is "nuccChunkPage"
            let chunk = Chunk::read_chunk(reader.by_ref(), chunk_table, page_start)?;
            
            chunks.push(chunk.clone());

            if matches!(chunks.last(), Some(last_chunk) if matches!(last_chunk.data, NuccChunk::NuccChunkPage(_))) {
                break;
            }
        }

        // Get the chunk references associated with this page
        let mut chunk_references: Vec<ChunkReference> = Vec::new();
        
        let nucc_page: NuccPage = match &chunks.last().unwrap().data {
            NuccChunk::NuccChunkPage(nucc_page) => nucc_page.clone(),
            _ => panic!("Expected NuccPage chunk!")
        };

        let reference_size = nucc_page.reference_size;

        if reference_size != 0 {
            for i in 0..reference_size {
                let chunk_reference = chunk_table.chunk_map_references[(reference_start + i) as usize].clone();
                chunk_references.push(chunk_reference);
            }
        }
        

        page.chunks = chunks;
        page.chunk_references = chunk_references;

        Ok(page)
    }

    fn get_page_size(&self) -> u32 {
        if let Some(last_chunk) = self.chunks.last() {
            if let NuccChunk::NuccChunkPage(nucc_page) = &last_chunk.data {
                return nucc_page.page_size;
            }
        }
        return 0 // Return 0 if the last chunk is not a NuccPage chunk
    }

    fn get_reference_size(&self) -> u32 {
        if let Some(last_chunk) = self.chunks.last() {
            if let NuccChunk::NuccChunkPage(nucc_page) = &last_chunk.data {
                return nucc_page.reference_size;
            }
        }
        return 0
    }

    pub fn get_chunks_by_type(&self, chunk_type: &str, chunk_table: &NuccChunkTable) -> Vec<&Chunk> {
        let mut chunks: Vec<&Chunk> = Vec::new();

        for chunk in &self.chunks {
            let chunk_map = chunk_table.chunk_maps[chunk.chunk_map_index as usize].clone();

            if get_chunk_type(chunk_map.chunk_type_index, chunk_table) == chunk_type {
                chunks.push(chunk);
            }
            
            

           
            if chunk_type != "nuccChunkPage" && chunk_type != "nuccChunkNull" {
                chunks.push(chunk);
            }
        }
        
        return chunks
    }


    pub fn add_chunk(&mut self, chunk: Chunk) {
        if self.chunks.contains(&chunk) {
            return
        
        } else {
            self.chunks.push(chunk);
        }
    }

    /*pub fn cleanup(mut self) -> Self {
        // Remove NuccPage NuccNull chunks from the page and  chunks from the page
        self.chunks.retain(|chunk| chunk.chunk_type != "nuccChunkPage" && chunk.chunk_type != "nuccChunkNull");

        return self;
    }*/

    

    pub fn replace_chunk(&mut self, chunk: &Chunk) {
        let index = self.chunks.iter().position(|x| *x == chunk.clone()).unwrap();
        self.chunks[index] = chunk.clone();
    }
}

fn get_chunk_map_index(index: u32, page_start: u32, chunk_table: &NuccChunkTable) -> u32 {
    let chunk_map_index = chunk_table.chunk_map_indices[(page_start + index) as usize];

    return chunk_map_index

}

fn get_chunk_type(chunk_type_index: u32, chunk_table: &NuccChunkTable) -> String {
    let chunk_type = chunk_table.chunk_types[chunk_type_index as usize].clone();
    return chunk_type.to_string()
}

fn get_chunk_filepath(chunk_map_index: u32, chunk_table: &NuccChunkTable) -> String {
    let chunk_map = chunk_table.chunk_maps[chunk_map_index as usize].clone();
    let filepath_index = chunk_map.filepath_index;
    let filepath = chunk_table.filepaths[filepath_index as usize].clone();

    return filepath.to_string();
    
}

fn get_chunk_name(chunk_map_index: u32, chunk_table: &NuccChunkTable) -> String {
    let chunk_map = chunk_table.chunk_maps[chunk_map_index as usize].clone();
    let chunk_name_index = chunk_map.chunk_name_index;
    let chunk_name = chunk_table.chunk_names[chunk_name_index as usize].clone();
 
    return chunk_name.to_string();
}
