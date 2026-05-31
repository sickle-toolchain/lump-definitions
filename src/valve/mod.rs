pub mod orange_box;
pub mod source;

use core::fmt::Debug;

use derive_more::TryFrom;
use zerocopy_derive::*;

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Plane {
    pub normal: [f32; 3],
    pub dist: f32,
    pub ty: i32,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Vertex {
    pub point: [f32; 3],
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Edge {
    pub edge: [u16; 2],
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct SurfaceEdge {
    pub edge_index: i32,
}

/// Compressed color format
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct ColorRGBExp32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub exponent: i8,
}

pub const LIGHTMAP_COUNT: usize = 4;

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Lightmap {
    pub mins: [i32; 2],
    pub maxs: [i32; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceFlags {}

impl SurfaceFlags {
    pub const LIGHT: u16 = 0x0001;
    pub const SKY2D: u16 = 0x0002;
    pub const SKY: u16 = 0x0004;
    pub const WARP: u16 = 0x0008;
    pub const TRANS: u16 = 0x0010;
    pub const NOPORTAL: u16 = 0x0020;
    pub const TRIGGER: u16 = 0x0040;
    pub const NODRAW: u16 = 0x0080;
    pub const HINT: u16 = 0x0100;
    pub const SKIP: u16 = 0x0200;
    pub const NOLIGHT: u16 = 0x0400;
    pub const BUMPLIGHT: u16 = 0x0800;
    pub const NOSHADOWS: u16 = 0x1000;
    pub const NODECALS: u16 = 0x2000;
    pub const NOCHOP: u16 = 0x4000;
    pub const HITBOX: u16 = 0x8000;
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy)]
#[repr(C)]
pub struct TextureMapping {
    pub xyz: [f32; 3],
    pub offset: f32,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy)]
#[repr(C)]
pub struct TextureInfo {
    /// [s/t][xyz offset]
    pub texels: [TextureMapping; 2],
    /// [s/t][xyz offset] - length is in units of texels/area
    pub luxels: [TextureMapping; 2],
    /// Miptex flags + overrides
    pub flags: i32,
    /// Index into texture data lump
    pub texture_data_index: i32,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy)]
#[repr(C)]
pub struct TextureData {
    pub reflectivity: [f32; 3],
    pub name_index: i32,
    pub width: i32,
    pub height: i32,
    pub view_width: i32,
    pub view_height: i32,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy)]
#[repr(C)]
pub struct PrimitiveCount(u16);

impl PrimitiveCount {
    pub fn new(primitive_count: u16, allow_dynamic_shadows: bool) -> Self {
        let mut instance = Self::default();
        instance.set_primitive_count(primitive_count);
        instance.set_allow_dynamic_shadows(allow_dynamic_shadows);

        instance
    }

    pub fn allow_dynamic_shadows(&self) -> bool {
        self.0 & 0x8000 == 0
    }

    pub fn set_allow_dynamic_shadows(&mut self, allow_dynamic_shadows: bool) {
        if allow_dynamic_shadows {
            self.0 &= !0x8000;
        } else {
            self.0 |= 0x8000;
        }
    }

    pub fn primitive_count(&self) -> u16 {
        self.0 & 0x7FFF
    }

    pub fn set_primitive_count(&mut self, primitive_count: u16) {
        assert!((primitive_count & 0x8000) == 0);
        self.0 &= !0x7FFF;
        self.0 |= primitive_count & 0x7FFF;
    }
}

impl Debug for PrimitiveCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PrimitiveCount")
            .field("allow_dynamic_shadows", &self.allow_dynamic_shadows())
            .field("primitive_count", &self.primitive_count())
            .finish()
    }
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Default, Clone, Copy)]
#[repr(C)]
pub struct AreaFlags(i16);

impl AreaFlags {
    pub fn new(area: u16, flags: u8) -> Self {
        let mut instance = Self::default();
        instance.set_area(area);
        instance.set_flags(flags);
        instance
    }

    pub fn area(&self) -> u16 {
        (self.0 as u16) & 0x01FF
    }

    pub fn set_area(&mut self, area: u16) {
        assert!((area & !0x01FF) == 0);
        let bits = self.0 as u16;
        self.0 = ((bits & !0x01FF) | (area & 0x01FF)) as i16;
    }

    pub fn flags(&self) -> u8 {
        (((self.0 as u16) >> 9) & 0x7F) as u8
    }

    pub fn set_flags(&mut self, flags: u8) {
        assert!((flags & 0x80) == 0);
        let bits = self.0 as u16;
        self.0 = ((bits & 0x01FF) | (((flags as u16) & 0x7F) << 9)) as i16;
    }
}

impl Debug for AreaFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AreaFlags")
            .field("area", &self.area())
            .field("flags", &self.flags())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LeafFlags {}

impl LeafFlags {
    pub const SKY: u8 = 0x01;
    pub const RADIAL: u8 = 0x02;
    pub const SKY2D: u8 = 0x04;
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub struct Face {
    pub plane_index: u16,
    pub side: i8,
    pub is_on_node: i8,
    pub edge_index: i32,
    pub edge_count: i16,
    pub texture_info_index: i16,
    pub displacement_info_index: i16,
    pub surface_fog_volume_id: i16,
    pub styles: [u8; LIGHTMAP_COUNT],
    pub light_offset: i32,
    pub area: f32,
    pub lightmap: Lightmap,
    pub original_face: i32,
    pub primitive_count: PrimitiveCount,
    pub primitive_index: u16,
    pub smoothing_groups: u32,
}

/// Lights used to illuminate the world
#[derive(
    TryFromBytes,
    TryFrom,
    IntoBytes,
    KnownLayout,
    Immutable,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[try_from(repr)]
#[repr(u32)]
pub enum EmitType {
    /// 90 degree spotlight
    Surface = 0,
    /// simple point light source
    Point,
    /// Spotlight with penumbra
    Spotlight,
    /// Directional light with no falloff (surface must trace to SKY texture)
    SkyLight,
    /// Linear falloff, non-lambertian
    QuakeLight,
    /// Spherical light source with no falloff (surface must trace to SKY texture)
    SkyAmbient,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub struct WorldLight {
    pub origin: [f32; 3],
    pub intensity: [f32; 3],
    /// For surfaces and spotlights
    pub normal: [f32; 3],
    pub cluster: i32,
    // TODO: it would be really nice if we could use `EmitType` directly here.
    pub ty: u32,
    pub style: i32,
    /// Start of penumbra for emit_spotlight
    pub penumbra_start: f32,
    /// End of penumbra for emit_spotlight
    pub penumbra_end: f32,
    pub exponent: f32,
    /// Cutoff distance
    pub radius: f32,
    /// Falloff for emit_spotlight + emit_point:
    /// 1 / (constant_attn + linear_attn * dist + quadratic_attn * dist^2)
    pub constant_attn: f32,
    pub linear_attn: f32,
    pub quadratic_attn: f32,
    /// Uses a combination of the DWL_FLAGS_ defines.
    pub flags: i32,
    pub texinfo: i32,
    /// Entity that this light is relative to
    pub owner: i32,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
#[repr(C)]
pub struct Model {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub origin: [f32; 3],
    pub head_node: i32,
    pub face_index: i32,
    pub face_count: i32,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Brush {
    pub first_side: i32,
    pub num_sides: i32,
    pub contents: i32,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct BrushSide {
    pub plane_num: u16,
    pub tex_info: i16,
    pub disp_info: i16,
    pub bevel: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Contents {}

impl Contents {
    pub const SOLID: i32 = 0x1;
    pub const WINDOW: i32 = 0x2;
    pub const AUX: i32 = 0x4;
    pub const GRATE: i32 = 0x8;
    pub const SLIME: i32 = 0x10;
    pub const WATER: i32 = 0x20;
    pub const MIST: i32 = 0x40;
    pub const OPAQUE: i32 = 0x80;
    pub const TESTFOGVOLUME: i32 = 0x100;
    pub const TEAM1: i32 = 0x800;
    pub const TEAM2: i32 = 0x1000;
    pub const IGNORE_NODRAW_OPAQUE: i32 = 0x2000;
    pub const MOVEABLE: i32 = 0x4000;
    pub const AREAPORTAL: i32 = 0x8000;
    pub const PLAYERCLIP: i32 = 0x10000;
    pub const MONSTERCLIP: i32 = 0x20000;
    pub const CURRENT_0: i32 = 0x40000;
    pub const CURRENT_90: i32 = 0x80000;
    pub const CURRENT_180: i32 = 0x100000;
    pub const CURRENT_270: i32 = 0x200000;
    pub const CURRENT_UP: i32 = 0x400000;
    pub const CURRENT_DOWN: i32 = 0x800000;
    pub const ORIGIN: i32 = 0x1000000;
    pub const MONSTER: i32 = 0x2000000;
    pub const DEBRIS: i32 = 0x4000000;
    pub const DETAIL: i32 = 0x8000000;
    pub const TRANSLUCENT: i32 = 0x10000000;
    pub const LADDER: i32 = 0x20000000;
    pub const HITBOX: i32 = 0x40000000;
}

pub const MASK_OPAQUE: i32 = Contents::SOLID | Contents::MOVEABLE | Contents::OPAQUE;

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Node {
    pub plane_num: i32,
    pub children: [i32; 2],
    pub mins: [i16; 3],
    pub maxs: [i16; 3],
    pub first_face: u16,
    pub num_faces: u16,
    pub area: i16,
    pub _padding: i16,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Leaf {
    pub contents: i32,
    pub cluster: i16,
    pub area_flags: AreaFlags,
    pub mins: [i16; 3],
    pub maxs: [i16; 3],
    pub first_leaf_face: u16,
    pub num_leaf_faces: u16,
    pub first_leaf_brush: u16,
    pub num_leaf_brushes: u16,
    pub leaf_water_data_id: i16,
    pub _padding: i16,
}

#[cfg(test)]
mod test {
    use super::{AreaFlags, LeafFlags, PrimitiveCount};

    #[test]
    fn primitive_count() {
        let mut primitive_count = PrimitiveCount::new(0, false);
        assert_eq!(primitive_count.allow_dynamic_shadows(), false);
        assert_eq!(primitive_count.primitive_count(), 0);

        primitive_count.set_allow_dynamic_shadows(true);
        assert_eq!(primitive_count.allow_dynamic_shadows(), true);
        assert_eq!(primitive_count.primitive_count(), 0);

        primitive_count.set_primitive_count(512);
        assert_eq!(primitive_count.allow_dynamic_shadows(), true);
        assert_eq!(primitive_count.primitive_count(), 512);
    }

    #[test]
    #[should_panic]
    fn primitive_count_invalid() {
        let mut primitive_count = PrimitiveCount::new(0, false);
        primitive_count.set_primitive_count(u16::MAX);
    }

    #[test]
    fn area_flags() {
        let mut af = AreaFlags::new(0, 0);
        assert_eq!(af.area(), 0);
        assert_eq!(af.flags(), 0);

        af.set_area(511);
        assert_eq!(af.area(), 511);
        assert_eq!(af.flags(), 0);

        af.set_flags(LeafFlags::SKY | LeafFlags::SKY2D);
        assert_eq!(af.area(), 511);
        assert_eq!(af.flags(), LeafFlags::SKY | LeafFlags::SKY2D);

        af.set_area(42);
        assert_eq!(af.area(), 42);
        assert_eq!(af.flags(), LeafFlags::SKY | LeafFlags::SKY2D);
    }

    #[test]
    #[should_panic]
    fn area_flags_area_overflow() {
        AreaFlags::new(0, 0).set_area(512);
    }

    #[test]
    #[should_panic]
    fn area_flags_flags_overflow() {
        AreaFlags::new(0, 0).set_flags(0x80);
    }
}
