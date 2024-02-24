use chumsky::span::{SimpleSpan, Span};

use crate::{front::span::Spanned, module::store::handle::Handle};

// Always pre-declared
// This cannot be created via user code
pub struct TypeGenerator {
    pub types: Vec<Type>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Alias(Alias),
    Plain(Plain),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Plain {
    Scalar(Scalar),
    Vec(VecType),
    Mat(Mat),
    Array(Handle<Type>, Option<u32>),
    Struct(Struct),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alias {
    pub ast: Spanned<crate::front::ast::statement::declaration::TypeAlias>,
    pub ident: String,
    pub alias_base: Handle<Type>,
    pub template_args: Vec<Handle<Type>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Struct {
    pub ast: Spanned<crate::front::ast::statement::declaration::Struct>,
    pub ident: String,
    pub members: Vec<StructMember>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructMember {
    pub ast: Spanned<crate::front::ast::statement::declaration::StructMember>,
    pub ident: String,
    pub ty: Handle<Type>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VecType {
    Vec2(Scalar),
    Vec3(Scalar),
    Vec4(Scalar),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Mat {
    Mat2(VecType),
    Mat3(VecType),
    Mat4(VecType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scalar {
    Boolean,
    AbstractInt,
    AbstractFloat,
    I32,
    U32,
    F32,
    F16,
}

impl Type {
    pub fn definition_span(&self) -> Option<SimpleSpan> {
        match self {
            Type::Alias(a) => Some(a.ast.span),
            Type::Plain(Plain::Struct(s)) => Some(s.ast.span),
            _ => None,
        }
    }
}
