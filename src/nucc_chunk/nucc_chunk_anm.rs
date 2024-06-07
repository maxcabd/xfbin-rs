//! # nuccAnm
//! nuccAnm is a chunk that contains animation data.
//! The extension ".anm" stands for "Animation".
//!
use std::{default, io::{Read, Seek, Write}};
use binrw::{binrw, BinRead, BinResult, BinWrite, ReadOptions, WriteOptions};

use super::{NuccChunk, NuccChunkType};

#[binrw]
#[brw(big)]
#[br(import_raw(version: u16))]
#[derive(Debug, Clone, Default)]
pub struct NuccChunkAnm {
    #[brw(ignore)]
    pub version: u16,

    pub frame_count: u32,

    #[bw(calc = 100)]
    pub frame_size: u32,

    #[bw(calc = entries.len() as u16)]
    pub entry_count: u16,

    pub is_looped: u16,

    #[bw(calc = clumps.len() as u16)]
    pub clump_count: u16,

    #[bw(calc = other_entries_indices.len() as u16)]
    pub other_entry_count: u16,

    #[bw(calc = unk_entry_indices.len() as u16)]
    pub unk_entry_count: u16,

    #[bw(calc = coord_parents.len() as u16)]
    pub coord_count: u16,

    #[br(count = clump_count)]
    pub clumps: Vec<AnmClump>,

    #[br(count = other_entry_count)]
    pub other_entries_indices: Vec<u32>,

    #[br(count = unk_entry_count)]
    pub unk_entry_indices: Vec<u32>,

    #[br(count = coord_count)]
    pub coord_parents: Vec<CoordParent>,

    #[br(count = entry_count)]
    pub entries: Vec<AnmEntry>,
}

#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct AnmClump {
    pub clump_index: u32,

    #[bw(calc = bone_material_indices.len() as u16)]
    pub bone_material_count: u16,

    #[bw(calc = model_indices.len() as u16)]
    pub model_count: u16,

    #[br(count = bone_material_count)]
    pub bone_material_indices: Vec<u32>,

    #[br(count = model_count)]
    pub model_indices: Vec<u32>,
}


#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct CoordParent {
    pub parent: AnmCoord,
    pub child: AnmCoord,
}


#[binrw]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnmCoord {
    pub clump_index: i16,
    pub coord_index: u16,
}




#[binrw]
#[brw(repr(u16))]
#[derive(Debug, Clone, Default)]
pub enum EntryFormat {
    Coord = 1,
    Camera = 2,
    Material = 4,
    LightDirc = 5,
    LightPoint = 6,
    Ambient = 8,
    MorphModel = 9,

    #[default]
    Unknown,
}


#[binrw]
#[derive(Debug, Clone)]
pub struct AnmEntry {
    pub coord: AnmCoord,
    pub entry_format: EntryFormat,

    #[bw(calc = curve_headers.len() as u16)]
    pub curve_count: u16,

    #[br(count = curve_count)]
    pub curve_headers: Vec<CurveHeader>,

    #[br(parse_with = read_curve(curve_headers.iter()))]
    #[br(align_after = 4)]
    pub curves: Vec<Curve>,
}


#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct CurveHeader {
    pub curve_index: u16,
    pub curve_format: CurveFormat,
    pub frame_count: u16,
    pub curve_size: u16,
}


#[binrw]
#[brw(repr(u16))]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CurveFormat {
    Vector3Fixed = 0x5,
    Vector3Linear = 0x6,
    Vector3Bezier = 0x7,
    EulerXYZFixed  = 0x8,
    EulerInterpolated = 0x9,
    QuaternionLinear = 0xA,
    FloatFixed = 0xB,
    FloatLinear = 0xC,
    Vector2Fixed = 0xD,
    Vector2Linear = 0xE,
    OpacityShortTable = 0xF,
    ScaleShortTable = 0x10,
    QuaternionShortTable = 0x11,
    ColorRGBTable = 0x14,
    Vector3Table = 0x15,
    FloatTable = 0x16,
    QuaternionTable = 0x17,
    FloatTableNoInterp = 0x18,
    Vector3ShortLinear = 0x19,
    Vector3TableNoInterp = 0x1A,
    QuaternionShortTableNoInterp = 0x1B,
    OpacityShortTableNoInterp = 0x1D,

    #[default]
    Unknown,
}

impl CurveFormat {
    pub fn size_per_frame(&self) -> usize {
        match self {
            CurveFormat::OpacityShortTable | CurveFormat::OpacityShortTableNoInterp => 0x02,
            CurveFormat::ColorRGBTable => 0x03,
            CurveFormat::FloatFixed | CurveFormat::FloatTable | CurveFormat::FloatTableNoInterp => {
                0x04
            }
            CurveFormat::ScaleShortTable => 0x06,
            CurveFormat::FloatLinear
            | CurveFormat::Vector2Fixed
            | CurveFormat::QuaternionShortTable
            | CurveFormat::QuaternionShortTableNoInterp => 0x08,
            CurveFormat::Vector3Fixed
            | CurveFormat::EulerXYZFixed
            | CurveFormat::Vector2Linear
            | CurveFormat::Vector3Table
            | CurveFormat::Vector3TableNoInterp => 0x0C,
            CurveFormat::Vector3Linear | CurveFormat::QuaternionTable => 0x10,
            CurveFormat::QuaternionLinear => 0x14,
            CurveFormat::Vector3Bezier => todo!(),
            CurveFormat::EulerInterpolated => todo!(),
            CurveFormat::Vector3ShortLinear => todo!(),
            CurveFormat::Unknown => todo!(),
        }
    }

}

#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub enum Math {
    Vec3 { channels: (f32, f32, f32) },
    Vec3Linear { frame: i32, channels: (f32, f32, f32) },
    Vec4Linear { frame: i32, channels: (f32, f32, f32, f32) },
    Float { channels: f32 },
    FloatLinear { frame: i32, channels: f32 },
    I16Vec { channels: i16 },
    I16Vec3 { channels: (i16, i16, i16) },
    I16Vec4 { channels: (i16, i16, i16, i16) },
    Color { channels: (u8, u8, u8) },

    // Add more variants as needed...
    Unknown {}
    
}

impl default::Default for Math {
    fn default() -> Self {
        Math::Unknown {}
    }
}


#[binrw]
#[br(import_raw(header: CurveHeader))]
#[derive(Debug, Clone, PartialEq)]
pub struct Curve {
    #[brw(ignore)]
    pub curve_format: CurveFormat, // Used for writing the data

    #[brw(ignore)]
    pub keyframe: Math,

    #[br(count = header.curve_format.size_per_frame() * header.frame_count as usize)]
    #[bw(write_with = |_data, writer, wo, () | write_curve(curve_format.clone(), keyframe.clone(), writer, wo, ()))]
    pub data: Vec<u8>,
}



fn read_curve<'it, R, T, Arg, Ret, It>(
    it: It,
) -> impl FnOnce(&mut R, &ReadOptions, ()) -> BinResult<Ret>
where
    T: BinRead<Args = Arg>,
    R: Read + Seek,
    Arg: Clone + 'static,
    Ret: FromIterator<T> + 'static,
    It: Iterator<Item = &'it Arg> + 'it,
{
    move |reader, options, _| {
        it.map(|arg| T::read_options(reader, options, arg.clone()))
            .collect()
    }
}

fn write_curve<R: Write + Seek>(
    curve_format: CurveFormat,
    values: Math,
    writer: &mut R,
    wo: &WriteOptions,
    
    _: ()
) -> BinResult<()> {

    match (curve_format, values) {
        (CurveFormat::Vector3Fixed, Math::Vec3 { channels }) |
        (CurveFormat::EulerXYZFixed, Math::Vec3 { channels }) |
        (CurveFormat::Vector3Table, Math::Vec3 { channels }) => {
            channels.0.write_options(writer, wo, ())?;
            channels.1.write_options(writer, wo, ())?;
            channels.2.write_options(writer, wo, ())?;
        }
    
        (CurveFormat::Vector3Linear, Math::Vec3Linear { frame, channels }) => {
            frame.write_options(writer, wo, ())?;
            channels.0.write_options(writer, wo, ())?;
            channels.1.write_options(writer, wo, ())?;
            channels.2.write_options(writer, wo, ())?;
        }

        (CurveFormat::QuaternionLinear, Math::Vec4Linear { frame, channels }) => {
            frame.write_options(writer, wo, ())?;
            channels.0.write_options(writer, wo, ())?;
            channels.1.write_options(writer, wo, ())?;
            channels.2.write_options(writer, wo, ())?;
            channels.3.write_options(writer, wo, ())?;
        }

        (CurveFormat::FloatFixed, Math::Float { channels }) |
        (CurveFormat::FloatTable, Math::Float { channels}) |
        (CurveFormat::FloatTableNoInterp, Math::Float { channels}) => {
            channels.write_options(writer, wo, ())?;
        }

        (CurveFormat::FloatLinear, Math::FloatLinear { frame, channels }) => {
            frame.write_options(writer, wo, ())?;
            channels.write_options(writer, wo, ())?;
        }

        (CurveFormat::OpacityShortTable, Math::I16Vec { channels }) |
        (CurveFormat::OpacityShortTableNoInterp, Math::I16Vec { channels }) => {
            channels.write_options(writer, wo, ())?;
        }

        (CurveFormat::ScaleShortTable, Math::I16Vec3 { channels }) => {
            channels.0.write_options(writer, wo, ())?;
            channels.1.write_options(writer, wo, ())?;
            channels.2.write_options(writer, wo, ())?;
        }

        (CurveFormat::QuaternionShortTable, Math::I16Vec4 { channels }) |
        (CurveFormat::QuaternionShortTableNoInterp, Math::I16Vec4 { channels }) => {
            channels.0.write_options(writer, wo, ())?;
            channels.1.write_options(writer, wo, ())?;
            channels.2.write_options(writer, wo, ())?;
            channels.3.write_options(writer, wo, ())?;
        }

        (CurveFormat::ColorRGBTable, Math::Color { channels }) => {
            channels.0.write_options(writer, wo, ())?;
            channels.1.write_options(writer, wo, ())?;
            channels.2.write_options(writer, wo, ())?;
        }
        // Handle other CurveFormat cases and Keyframe variants...
        _ => todo!(),
    }
    


    Ok(())
}




impl NuccChunk for NuccChunkAnm {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkAnm
    }

    fn version(&self) -> u16 {
        self.version
    }

    fn extension(&self) -> String {
        String::from(".anm")
    }
}
