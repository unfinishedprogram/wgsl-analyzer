use chumsky::span::SimpleSpan;
pub mod generator;

use crate::{
    diagnostic::Diagnostic,
    front::span::Spanned,
    module::{
        store::{handle::Handle, Store},
        type_store::TypeStore,
    },
};

use self::generator::TypeGenerator;

// Always pre-declared
// This cannot be created via user code

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Alias(Alias),
    Plain(Plain),
    Generator(TypeGenerator),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Plain {
    Scalar(Scalar),
    // TODO: Use int literal type for arr len
    Array(Handle<Type>, Option<String>),
    Struct(Struct),
    Mat(Mat),
    Vec(VecType),
    Atomic(Scalar),
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
pub enum Templated {}

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

    pub fn validate_is_constructable(&self, type_store: &mut TypeStore) -> Result<(), Diagnostic> {
        match self {
            Type::Alias(ty) => {
                let base = type_store.types.get(&ty.alias_base).clone();
                base.validate_is_constructable(type_store)
            }
            Type::Generator(_) => Err(Diagnostic::error("type generators are not constructable")),
            Type::Plain(ty) => match ty {
                Plain::Scalar(_) => Ok(()),
                Plain::Array(_, None) => {
                    Err(Diagnostic::error("unsized arrays are not constructable"))
                }
                Plain::Array(ty, Some(_)) => {
                    let component = type_store.types.get(ty).clone();
                    component.validate_is_constructable(type_store)
                }
                Plain::Struct(s) => {
                    for member in &s.members {
                        let member_ty = type_store.types.get(&member.ty).clone();
                        member_ty
                            .validate_is_constructable(type_store)
                            .map_err(|err| err.span_if_none(member.ast.span))?;
                    }
                    Ok(())
                }
                Plain::Mat(_) => Ok(()),
                Plain::Vec(_) => Ok(()),
                Plain::Atomic(_) => Err(Diagnostic::error("atomic types are not constructable")),
            },
        }
    }
}
