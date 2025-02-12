pub mod orange_box;
pub mod source;

use std::fmt::Debug;

use zerocopy_derive::*;

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug)]
#[repr(C)]
pub struct Plane {
    pub normal: [f32; 3],
    pub dist: f32,
    pub ty: i32,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug)]
#[repr(C)]
pub struct Vertex {
    pub point: [f32; 3],
}

/// Compressed color format
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug)]
#[repr(C)]
pub struct ColorRGBExp32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub exponent: i8,
}

pub const LIGHTMAP_COUNT: usize = 4;

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug)]
#[repr(C)]
pub struct Lightmap {
    pub mins: [i32; 2],
    pub maxs: [i32; 2],
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct PrimitiveCount(u16);

impl PrimitiveCount {
    pub fn new(primitive_count: u16, allow_dynamic_shadows: bool) -> Self {
        let mut instance = Self(0);
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveCount")
            .field("allow_dynamic_shadows", &self.allow_dynamic_shadows())
            .field("primitive_count", &self.primitive_count())
            .finish()
    }
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug)]
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
#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Debug)]
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

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Debug)]
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

#[cfg(test)]
mod test {
    use super::PrimitiveCount;

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
}
