use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::expression::{Expression, ExpressionInner, TemplateElaboratedIdent, TemplateList},
        span::{Spanned, WithSpan},
        token::Literal,
    },
    module::{store::Store, type_store::TypeStore},
};

use super::{Mat, Plain, Scalar, Type, VecType};

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

fn expect_ident(expr: &Expression) -> Result<&Spanned<TemplateElaboratedIdent>, Diagnostic> {
    match expr.as_inner() {
        ExpressionInner::Ident(ident) => Ok(ident),
        _ => Err(Diagnostic::error("Expected an identifier").span(expr.span())),
    }
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

    fn validate_arg_count(&self, count: usize) -> Result<(), Diagnostic> {
        let (min, max) = self.valid_arg_count_range();

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
        args: TemplateList,
    ) -> Result<Type, Vec<Diagnostic>> {
        let args = args.0;
        self.validate_arg_count(args.len())?;

        fn not_implemented() -> Result<Type, Vec<Diagnostic>> {
            Err(vec![Diagnostic::error("Not implemented")])
        }

        match self {
            TypeGenerator::Array => {
                let content_type = if let ExpressionInner::Ident(ident) = &args[0].inner {
                    store.type_of_ident(ident)?
                } else {
                    return Err(vec![Diagnostic::error(
                        "Array type specifier must be an identifier",
                    )
                    .span(args[0].span())]);
                };

                let array_length = if let Some(arg) = args.get(1) {
                    match arg.as_inner() {
                        ExpressionInner::Literal(Literal::Int(int)) => Some(int.clone()),
                        // TODO: Constant evaluation
                        _ => return Err(vec![Diagnostic::error(
                            "Invalid array length specifier. Array length must evaluate to a constant of type i32 or u32",
                        )
                        .span(arg.span())]),
                    }
                } else {
                    None
                };

                Ok(Type::Plain(
                    crate::module::declaration::r#type::Plain::Array(content_type, array_length),
                ))
            }
            TypeGenerator::Atomic => {
                let content_type = store.type_of_ident(expect_ident(&args[0])?)?;
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
            TypeGenerator::Mat2x2
            | TypeGenerator::Mat2x3
            | TypeGenerator::Mat2x4
            | TypeGenerator::Mat3x2
            | TypeGenerator::Mat3x3
            | TypeGenerator::Mat3x4
            | TypeGenerator::Mat4x2
            | TypeGenerator::Mat4x3
            | TypeGenerator::Mat4x4 => {
                let content_type = store.type_of_ident(expect_ident(&args[0])?)?;
                let content_type = store.types.get(&content_type);

                let scalar_type = match content_type {
                    Type::Plain(Plain::Scalar(scalar))
                        if matches!(scalar, Scalar::F32 | Scalar::F16) =>
                    {
                        Ok(scalar.clone())
                    }
                    _ => Err(vec![Diagnostic::error(
                        "Invalid matrix component type provided. Matrix types can only be f32, f16",
                    )
                    .span(args[0].span())]),
                }?;

                let res = match self {
                    TypeGenerator::Mat2x2 => Mat::Mat2(VecType::Vec2(scalar_type)),
                    TypeGenerator::Mat2x3 => Mat::Mat2(VecType::Vec3(scalar_type)),
                    TypeGenerator::Mat2x4 => Mat::Mat2(VecType::Vec4(scalar_type)),
                    TypeGenerator::Mat3x2 => Mat::Mat3(VecType::Vec2(scalar_type)),
                    TypeGenerator::Mat3x3 => Mat::Mat3(VecType::Vec3(scalar_type)),
                    TypeGenerator::Mat3x4 => Mat::Mat3(VecType::Vec4(scalar_type)),
                    TypeGenerator::Mat4x2 => Mat::Mat4(VecType::Vec2(scalar_type)),
                    TypeGenerator::Mat4x3 => Mat::Mat4(VecType::Vec3(scalar_type)),
                    TypeGenerator::Mat4x4 => Mat::Mat4(VecType::Vec4(scalar_type)),
                    _ => unreachable!(),
                };

                Ok(Type::Plain(Plain::Mat(res)))
            }
            TypeGenerator::Ptr => not_implemented(),
            TypeGenerator::Texture1D => not_implemented(),
            TypeGenerator::Texture2D => not_implemented(),
            TypeGenerator::Texture2DArray => not_implemented(),
            TypeGenerator::Texture3D => not_implemented(),
            TypeGenerator::TextureCube => not_implemented(),
            TypeGenerator::TextureCubeArray => not_implemented(),
            TypeGenerator::TextureMultisampled2D => not_implemented(),
            TypeGenerator::TextureDepthMultisampled2D => not_implemented(),
            TypeGenerator::TextureStorage2D => not_implemented(),
            TypeGenerator::TextureStorage2DArray => not_implemented(),
            TypeGenerator::TextureStorage3D => not_implemented(),
            TypeGenerator::Vec2 | TypeGenerator::Vec3 | TypeGenerator::Vec4 => {
                let ident = expect_ident(&args[0])?;

                let component_handle = store.type_of_ident(&ident)?;
                let component_type = store.types.get(&component_handle);

                let scalar_type = match component_type {
                    Type::Plain(Plain::Scalar(scalar)) => Ok(scalar.clone()),
                    _ => Err(vec![Diagnostic::error(
                        "Invalid component type. Vector components must be of scalar types",
                    )
                    .span(ident.span())]),
                }?;

                let res = match self {
                    TypeGenerator::Vec2 => VecType::Vec2(scalar_type),
                    TypeGenerator::Vec3 => VecType::Vec3(scalar_type),
                    TypeGenerator::Vec4 => VecType::Vec4(scalar_type),
                    _ => unreachable!(),
                };

                Ok(Type::Plain(Plain::Vec(res)))
            }
        }
    }
}
