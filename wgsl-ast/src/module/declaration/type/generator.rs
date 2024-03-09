use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::expression::TemplateElaboratedIdent,
        span::{Spanned, WithSpan},
    },
    module::{store::Store, type_store::TypeStore},
};

use super::{Plain, Scalar, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeGenerator {
    Array,
    Atomic,
    Mat2x2,
    Mat2x3,
    Mat2x4,
    Mat3x2,
    Mat3x3,
    Mat3x4,
    Mat4x2,
    Mat4x3,
    Mat4x4,
    Ptr,
    Texture1D,
    Texture2D,
    Texture2DArray,
    Texture3D,
    TextureCube,
    TextureCubeArray,
    TextureMultisampled2D,
    TextureDepthMultisampled2D,
    TextureStorage2D,
    TextureStorage2DArray,
    TextureStorage3D,
    Vec2,
    Vec3,
    Vec4,
}

impl TypeGenerator {
    pub fn all_predeclared() -> Vec<(&'static str, Self)> {
        vec![
            ("array", Self::Array),
            ("atomic", Self::Atomic),
            ("mat2x2", Self::Mat2x2),
            ("mat2x3", Self::Mat2x3),
            ("mat2x4", Self::Mat2x4),
            ("mat3x2", Self::Mat3x2),
            ("mat3x3", Self::Mat3x3),
            ("mat3x4", Self::Mat3x4),
            ("mat4x2", Self::Mat4x2),
            ("mat4x3", Self::Mat4x3),
            ("mat4x4", Self::Mat4x4),
            ("ptr", Self::Ptr),
            ("texture_1d", Self::Texture1D),
            ("texture_2d", Self::Texture2D),
            ("texture_2d_array", Self::Texture2DArray),
            ("texture_3d", Self::Texture3D),
            ("texture_cube", Self::TextureCube),
            ("texture_cube_array", Self::TextureCubeArray),
            ("texture_multisampled_2d", Self::TextureMultisampled2D),
            (
                "texture_depth_multisampled_2d",
                Self::TextureDepthMultisampled2D,
            ),
            ("texture_storage_2d", Self::TextureStorage2D),
            ("texture_storage_2d_array", Self::TextureStorage2DArray),
            ("texture_storage_3d", Self::TextureStorage3D),
            ("vec2", Self::Vec2),
            ("vec3", Self::Vec3),
            ("vec4", Self::Vec4),
        ]
    }

    fn valid_arg_count_range(&self) -> (usize, usize) {
        match self {
            Self::Array => (1, 2),
            Self::Atomic => (1, 1),
            Self::Mat2x2
            | Self::Mat2x3
            | Self::Mat2x4
            | Self::Mat3x2
            | Self::Mat3x3
            | Self::Mat3x4
            | Self::Mat4x2
            | Self::Mat4x3
            | Self::Mat4x4 => (1, 1),
            Self::Ptr => (1, 3),
            Self::Texture1D
            | Self::Texture2D
            | Self::Texture2DArray
            | Self::Texture3D
            | Self::TextureCube
            | Self::TextureCubeArray => (1, 1),
            Self::TextureMultisampled2D => (1, 1),
            Self::TextureDepthMultisampled2D => (0, 0),
            Self::TextureStorage2D | Self::TextureStorage2DArray | Self::TextureStorage3D => (2, 2),
            Self::Vec2 | Self::Vec3 | Self::Vec4 => (1, 1),
        }
    }

    fn validate_arg_count(
        &self,
        args: &[Spanned<TemplateElaboratedIdent>],
    ) -> Result<(), Diagnostic> {
        let (min, max) = self.valid_arg_count_range();
        let count = args.len();

        if count < min {
            return Err(Diagnostic::error(format!(
                "Type requires at least {min} template arguments, got : {count}",
            )));
        }

        if count > max {
            return Err(Diagnostic::error(format!(
                "Type requires at most {max} template arguments, got : {count}",
            )));
        }

        Ok(())
    }

    // Apply template parameters to the given type generator
    pub fn apply_template_args(
        &self,
        store: &mut TypeStore,
        args: Vec<Spanned<TemplateElaboratedIdent>>,
    ) -> Result<Type, Vec<Diagnostic>> {
        self.validate_arg_count(&args)?;

        // let mut diagnostics: Vec<Diagnostic> = Vec::new();

        match self {
            TypeGenerator::Array => {
                // TODO: Implement fixed size arrays
                let content_type = store.type_of_ident(&args[0])?;
                Ok(Type::Plain(
                    crate::module::declaration::r#type::Plain::Array(content_type, None),
                ))
            }
            TypeGenerator::Atomic => {
                let content_type = store.type_of_ident(&args[0])?;
                let content_type = store.types.get(&content_type);
                match content_type {
                    Type::Plain(Plain::Scalar(scalar))
                        if matches!(scalar, Scalar::I32 | Scalar::U32) =>
                    {
                        Ok(Type::Plain(
                            crate::module::declaration::r#type::Plain::Atomic(scalar.clone()),
                        ))
                    }
                    _ => Err(vec![Diagnostic::error(
                        "Invalid atomic type provided. Atomic types can only be u32 or i32",
                    )
                    .span(args[0].span())]),
                }
            }
            TypeGenerator::Mat2x2 => todo!(),
            TypeGenerator::Mat2x3 => todo!(),
            TypeGenerator::Mat2x4 => todo!(),
            TypeGenerator::Mat3x2 => todo!(),
            TypeGenerator::Mat3x3 => todo!(),
            TypeGenerator::Mat3x4 => todo!(),
            TypeGenerator::Mat4x2 => todo!(),
            TypeGenerator::Mat4x3 => todo!(),
            TypeGenerator::Mat4x4 => todo!(),
            TypeGenerator::Ptr => todo!(),
            TypeGenerator::Texture1D => todo!(),
            TypeGenerator::Texture2D => todo!(),
            TypeGenerator::Texture2DArray => todo!(),
            TypeGenerator::Texture3D => todo!(),
            TypeGenerator::TextureCube => todo!(),
            TypeGenerator::TextureCubeArray => todo!(),
            TypeGenerator::TextureMultisampled2D => todo!(),
            TypeGenerator::TextureDepthMultisampled2D => todo!(),
            TypeGenerator::TextureStorage2D => todo!(),
            TypeGenerator::TextureStorage2DArray => todo!(),
            TypeGenerator::TextureStorage3D => todo!(),
            TypeGenerator::Vec2 => todo!(),
            TypeGenerator::Vec3 => todo!(),
            TypeGenerator::Vec4 => todo!(),
        }

        // todo!();
    }
}
