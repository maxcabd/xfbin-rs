/*use std::io::{Read, Seek};

// Contains the structures for the ANM file format
use binrw::{binrw, BinRead, BinResult, ReadOptions};


use crate::utils::*;




#[binrw]
#[derive(Debug)]
pub struct NuccChunkAnm {
    pub anm_length: u32,
    pub frame_size: u32,
    pub entry_count: u16,
    pub looped: u16,
    pub clump_count: u16,
    pub other_entry_count: u16,
    pub coord_count: u32,

    #[br(count = clump_count)]
    pub clumps: Vec<BrAnmClump>,

    #[br(count = other_entry_count)]
    pub other_entries: Vec<u32>,


    #[br(count = coord_count)]
    pub coord_parents: Vec<CoordParent>,

    #[br(count = entry_count)]
    pub entries: Vec<AnmEntry>

}


#[binrw]
#[derive(Debug)]
pub struct BrAnmClump {
    pub clump_index: u32,
    pub bone_material_count: u16,
    pub model_count: u16,

    #[br(count = bone_material_count)]
    pub bone_material_indices: Vec<u32>,
    
    #[br(count = model_count)]
    pub model_indices: Vec<u32>,


}

#[binrw]
#[derive(Debug)]
pub struct CoordParent {
    pub parent: AnmCoord,
    pub child: AnmCoord,

}

#[binrw]
#[derive(Debug)]
pub struct AnmCoord {
    pub clump_index: i16,
    pub coord_index: u16
}


#[binrw]
#[derive(Debug)]
pub struct AnmEntry {
    pub coord: AnmCoord,
    pub entry_format: u16,
    pub curve_count: u16,

    #[br(count = curve_count)]
    pub curve_headers: Vec<CurveHeader>,



    #[br(parse_with = from_iterator_args(curve_headers.iter()))]
    pub curves: Vec<Curve>
}


#[binrw]
#[derive(Debug, Clone)]
pub struct CurveHeader {
    pub curve_index: u16,
    pub curve_format: u16,
    pub frame_count: u16,
    pub curve_size: u16,  
}




#[binrw]
#[derive(Debug)]
#[br(import_raw(header: &CurveHeader))]
pub enum Curve {
    #[br(pre_assert(matches!(header.curve_format, 0x05 | 0x08 | 0x15)))]
    Vector3(#[br(count = header.frame_count)] Vec<Vector3>),

    #[br(pre_assert(matches!(header.curve_format, 0x06)))]
    KeyframeVector3(#[br(count = header.frame_count)] Vec<KeyframeVector3>),

    #[br(pre_assert(matches!(header.curve_format, 0x0A)))]
    KeyframeVector4(#[br(count = header.frame_count)] Vec<KeyframeVector4>),

    #[br(pre_assert(matches!(header.curve_format, 0x0B | 0x16 | 0x18)))]
    Float(#[br(count = header.frame_count)] Vec<f32>),

    #[br(pre_assert(matches!(header.curve_format, 0x0C)))]
    KeyframeFloat(#[br(count = header.frame_count)] Vec<KeyframeFloat>),

    #[br(pre_assert(matches!(header.curve_format, 0x0F)))]
    Short(#[br(count = header.frame_count, if(header.frame_count % 2 == 1), pad_after = 2)] Vec<i16>),

    #[br(pre_assert(matches!(header.curve_format, 0x10)))]
    Vector3Short(#[br(count = header.frame_count, if(header.frame_count % 2 == 1), pad_after = 2)] Vec<Vector3Short>), // #[br(if(header.frame_count % 2 == 0), pad_after = 2)]

    #[br(pre_assert(matches!(header.curve_format, 0x11)))]
    QuaternionShort(#[br(count = header.frame_count)] Vec<QuaternionShort>),

    #[br(pre_assert(matches!(header.curve_format, 0x14)))]
    RGB(#[br(count = header.frame_count, pad_after = header.frame_count % 4)] Vec<RGB>),

    // Handle unknown curve formats
    #[br(pre_assert(false))]
    Unknown(#[br(count = header.curve_size)] Vec<u8>)
}


fn from_iterator_args<'it, R, T, Arg, Ret, It>(it: It) -> impl FnOnce(&mut R, &ReadOptions, ()) -> BinResult<Ret>
where
  T: BinRead<Args = &'it Arg>,
  R: Read + Seek,
  Arg: Clone + 'static,
  Ret: FromIterator<T> + 'static,
  It: Iterator<Item = &'it Arg> + 'it,
{
  move |reader, options, _| {
    it.map(|arg| T::read_options(reader, options, arg.clone())).collect()
  }
}

*/