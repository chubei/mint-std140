#![deny(warnings)]
#![deny(missing_docs)]

//! This library contains a single trait [AsStd140] which is implemented for [mint] types that can be converted to [std140] types.
//!
//! # Examples
//!
//! ```rust
//! use mint_std140::AsStd140;
//!
//! let mint_vector = mint::Vector2 { x: 0.0f32, y: 0.0f32 };
//! let std140_vector = mint_vector.as_std140();
//! assert_eq!(mint_vector.x, std140_vector[0]);
//! assert_eq!(mint_vector.y, std140_vector[1]);
//!
//! let mint_matrix = mint::ColumnMatrix2 {
//!     x: mint::Vector2 { x: 0.0f32, y: 1.0f32 },
//!     y: mint::Vector2 { x: 2.0f32, y: 3.0f32 },
//! };
//! let std140_matrix = mint_matrix.as_std140();
//! ```

use std140::*;

/// A type that can be converted to a std140 type.
pub trait AsStd140 {
    /// The std140 type that this type can be converted to.
    type Std140Type;

    /// Convert this type to a std140 type.
    fn as_std140(&self) -> Self::Std140Type;
}

macro_rules! impl_as_std140_for_vector {
    ($mint_type:ty, $std140_name:ident, [$($field:ident),+]) => {
        impl AsStd140 for $mint_type {
            type Std140Type = $std140_name;

            fn as_std140(&self) -> Self::Std140Type {
                $std140_name($(self.$field),+)
            }
        }
    };
}

impl_as_std140_for_vector!(mint::Vector2<f32>, vec2, [x, y]);
impl_as_std140_for_vector!(mint::Vector3<f32>, vec3, [x, y, z]);
impl_as_std140_for_vector!(mint::Vector4<f32>, vec4, [x, y, z, w]);
impl_as_std140_for_vector!(mint::Vector2<i32>, ivec2, [x, y]);
impl_as_std140_for_vector!(mint::Vector3<i32>, ivec3, [x, y, z]);
impl_as_std140_for_vector!(mint::Vector4<i32>, ivec4, [x, y, z, w]);
impl_as_std140_for_vector!(mint::Vector2<u32>, uvec2, [x, y]);
impl_as_std140_for_vector!(mint::Vector3<u32>, uvec3, [x, y, z]);
impl_as_std140_for_vector!(mint::Vector4<u32>, uvec4, [x, y, z, w]);

macro_rules! impl_as_std140_for_column_matrix {
    ($mint_type:ty, $std140_name:ident, [$($field:ident),+]) => {
        impl AsStd140 for $mint_type {
            type Std140Type = $std140_name;

            fn as_std140(&self) -> Self::Std140Type {
                $std140_name(
                    $(self.$field.as_std140()),+
                )
            }
        }
    };
}

impl_as_std140_for_column_matrix!(mint::ColumnMatrix2<f32>, mat2x2, [x, y]);
impl_as_std140_for_column_matrix!(mint::ColumnMatrix3x2<f32>, mat2x3, [x, y]);
impl_as_std140_for_column_matrix!(mint::ColumnMatrix4x2<f32>, mat2x4, [x, y]);
impl_as_std140_for_column_matrix!(mint::ColumnMatrix2x3<f32>, mat3x2, [x, y, z]);
impl_as_std140_for_column_matrix!(mint::ColumnMatrix3<f32>, mat3x3, [x, y, z]);
impl_as_std140_for_column_matrix!(mint::ColumnMatrix4x3<f32>, mat3x4, [x, y, z]);
impl_as_std140_for_column_matrix!(mint::ColumnMatrix2x4<f32>, mat4x2, [x, y, z, w]);
impl_as_std140_for_column_matrix!(mint::ColumnMatrix3x4<f32>, mat4x3, [x, y, z, w]);
impl_as_std140_for_column_matrix!(mint::ColumnMatrix4<f32>, mat4x4, [x, y, z, w]);

#[cfg(test)]
mod tests {
    use super::AsStd140;

    #[test]
    fn vectors() {
        let vector = mint::Vector2 { x: 1.0f32, y: 2.0f32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);

        let vector = mint::Vector3 { x: 1.0f32, y: 2.0f32, z: 3.0f32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);
        assert_eq!(vector.z, std140_vector[2]);

        let vector = mint::Vector4 { x: 1.0f32, y: 2.0f32, z: 3.0f32, w: 4.0f32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);
        assert_eq!(vector.z, std140_vector[2]);
        assert_eq!(vector.w, std140_vector[3]);

        let vector = mint::Vector2 { x: 1i32, y: 2i32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);

        let vector = mint::Vector3 { x: 1i32, y: 2i32, z: 3i32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);
        assert_eq!(vector.z, std140_vector[2]);

        let vector = mint::Vector4 { x: 1i32, y: 2i32, z: 3i32, w: 4i32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);
        assert_eq!(vector.z, std140_vector[2]);
        assert_eq!(vector.w, std140_vector[3]);

        let vector = mint::Vector2 { x: 1u32, y: 2u32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);

        let vector = mint::Vector3 { x: 1u32, y: 2u32, z: 3u32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);
        assert_eq!(vector.z, std140_vector[2]);

        let vector = mint::Vector4 { x: 1u32, y: 2u32, z: 3u32, w: 4u32 };
        let std140_vector = vector.as_std140();
        assert_eq!(vector.x, std140_vector[0]);
        assert_eq!(vector.y, std140_vector[1]);
        assert_eq!(vector.z, std140_vector[2]);
        assert_eq!(vector.w, std140_vector[3]);
    }
}
