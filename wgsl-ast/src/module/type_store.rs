use std::collections::HashMap;

use chumsky::span::SimpleSpan;

use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::{expression::TemplateElaboratedIdent, statement::declaration::Declaration},
        span::{SpanAble, Spanned, WithSpan},
    },
    module::declaration::r#type::generator::TypeGenerator,
};

use super::{
    declaration::r#type::{Alias, Mat, Plain, Scalar, Struct, StructMember, Type, VecType},
    store::{handle::Handle, Arena, Store},
};

pub struct TypeStore {
    pub types: Arena<Type>,
    pub identifiers: HashMap<String, Handle<Type>>,
}

// For defining a matrix type with less much boilerplate / repetition
macro_rules! mat_type {
    ($mat_ty:ident, $vec_ty:ident, $scalar_ty:ident) => {
        Type::Plain(Plain::Mat(Mat::$mat_ty(VecType::$vec_ty(
            Scalar::$scalar_ty,
        ))))
    };
}

impl Default for TypeStore {
    fn default() -> Self {
        // Initialize the store with pre-declared types and type-aliases
        let types = Arena::default();
        let identifiers = HashMap::new();

        let mut res = Self { types, identifiers };
        res.init();
        res
    }
}

impl TypeStore {
    pub fn add(&mut self, ident: impl Into<String>, ty: Type) {
        let handle = self.types.insert(ty);
        self.identifiers.insert(ident.into(), handle);
    }

    pub fn init(&mut self) {
        use Plain as P;
        use Scalar as S;
        use Type as T;

        // let mat = ||

        self.add("bool", T::Plain(P::Scalar(S::Boolean)));
        self.add("i32", T::Plain(P::Scalar(S::I32)));
        self.add("u32", T::Plain(P::Scalar(S::U32)));
        self.add("f32", T::Plain(P::Scalar(S::F32)));
        self.add("f16", T::Plain(P::Scalar(S::F16)));

        self.add("mat2x2f", mat_type!(Mat2, Vec2, F32));
        self.add("mat2x3f", mat_type!(Mat2, Vec3, F32));
        self.add("mat2x4f", mat_type!(Mat2, Vec4, F32));
        self.add("mat3x2f", mat_type!(Mat3, Vec2, F32));
        self.add("mat3x3f", mat_type!(Mat3, Vec3, F32));
        self.add("mat3x4f", mat_type!(Mat3, Vec4, F32));
        self.add("mat4x2f", mat_type!(Mat4, Vec2, F32));
        self.add("mat4x3f", mat_type!(Mat4, Vec3, F32));
        self.add("mat4x4f", mat_type!(Mat4, Vec4, F32));

        self.add("mat2x2h", mat_type!(Mat2, Vec2, F16));
        self.add("mat2x3h", mat_type!(Mat2, Vec3, F16));
        self.add("mat2x4h", mat_type!(Mat2, Vec4, F16));
        self.add("mat3x2h", mat_type!(Mat3, Vec2, F16));
        self.add("mat3x3h", mat_type!(Mat3, Vec3, F16));
        self.add("mat3x4h", mat_type!(Mat3, Vec4, F16));
        self.add("mat4x2h", mat_type!(Mat4, Vec2, F16));
        self.add("mat4x3h", mat_type!(Mat4, Vec3, F16));
        self.add("mat4x4h", mat_type!(Mat4, Vec4, F16));

        self.add("vec2i", Type::Plain(Plain::Vec(VecType::Vec2(Scalar::I32))));
        self.add("vec3i", Type::Plain(Plain::Vec(VecType::Vec3(Scalar::I32))));
        self.add("vec4i", Type::Plain(Plain::Vec(VecType::Vec4(Scalar::I32))));
        self.add("vec2u", Type::Plain(Plain::Vec(VecType::Vec2(Scalar::U32))));
        self.add("vec3u", Type::Plain(Plain::Vec(VecType::Vec3(Scalar::U32))));
        self.add("vec4u", Type::Plain(Plain::Vec(VecType::Vec4(Scalar::U32))));
        self.add("vec2f", Type::Plain(Plain::Vec(VecType::Vec2(Scalar::F32))));
        self.add("vec3f", Type::Plain(Plain::Vec(VecType::Vec3(Scalar::F32))));
        self.add("vec4f", Type::Plain(Plain::Vec(VecType::Vec4(Scalar::F32))));
        self.add("vec2h", Type::Plain(Plain::Vec(VecType::Vec2(Scalar::F16))));
        self.add("vec3h", Type::Plain(Plain::Vec(VecType::Vec3(Scalar::F16))));
        self.add("vec4h", Type::Plain(Plain::Vec(VecType::Vec4(Scalar::F16))));

        for generator in TypeGenerator::all_predeclared() {
            self.add(generator.0, T::Generator(generator.1));
        }
    }

    pub fn insert_declarations(
        &mut self,
        declarations: &[Spanned<Declaration>],
    ) -> Result<(), Vec<Diagnostic>> {
        // Since module declarations can appear out of order, we need to resolve them later
        let mut diagnostics: Vec<Diagnostic> = vec![];

        for decl in declarations {
            match decl.as_inner() {
                Declaration::TypeAlias(ty) => {
                    if let Some(diag) = self.validate_no_conflicting_definitions(ty.ident.clone()) {
                        diagnostics.push(diag);
                    } else {
                        let alias_base = self.type_of_ident(&ty.value)?;
                        self.add(
                            &ty.ident.inner,
                            Type::Alias(Alias {
                                ast: ty.clone().with_span(decl.span),
                                ident: ty.ident.inner.clone(),
                                alias_base,
                                template_args: vec![],
                            }),
                        );
                    }
                }
                Declaration::Struct(s) => {
                    match self.struct_from_ast(s.clone().with_span(decl.span)) {
                        Ok(s) => self.add(s.ident.clone(), Type::Plain(Plain::Struct(s))),
                        Err(mut e) => diagnostics.append(&mut e),
                    }
                }
                // Other declarations do not produce types
                _ => {}
            }
        }

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }

    pub fn validate_no_conflicting_definitions(
        &self,
        ident: Spanned<String>,
    ) -> Option<Diagnostic> {
        match self.identifiers.get(ident.as_inner()) {
            Some(handle) => match self.span_of(handle) {
                Some(span) => {
                    let diag = Diagnostic::error(format!(
                        "Type '{}' is already defined",
                        ident.as_inner()
                    ))
                    .span(span)
                    .related("Conflicting declaration here", span);
                    Some(diag)
                }
                None => {
                    // Must be a builtin type
                    let diag = Diagnostic::error(format!(
                        "Type '{}' is already defined as a builtin",
                        ident.as_inner()
                    ))
                    .span(ident.span());

                    Some(diag)
                }
            },
            None => None,
        }
    }

    pub fn struct_from_ast(
        &mut self,
        struct_ast: Spanned<crate::front::ast::statement::declaration::Struct>,
    ) -> Result<Struct, Vec<Diagnostic>> {
        let mut diagnostics = vec![];
        let mut members: Vec<StructMember> = vec![];

        for member in struct_ast.as_inner().members.iter() {
            let ty = &member.value;
            let ident = &member.ident;

            // Check that this name is not already defined in this struct
            for defined in &members {
                if &defined.ident == ident.as_inner() {
                    diagnostics.push(
                        Diagnostic::error(format!(
                            "Member '{}' already exists on struct '{}'",
                            ident.as_inner(),
                            struct_ast.ident.as_inner()
                        ))
                        .span(member.span())
                        .related("Other member defined here", defined.ast.span),
                    )
                }
            }

            match self.type_of_ident(ty) {
                Ok(ty) => members.push(StructMember {
                    ast: member.clone(),
                    ident: member.ident.as_inner().clone(),
                    ty,
                }),
                Err(diag) => diagnostics.extend(diag),
            }
        }

        if diagnostics.is_empty() {
            let ident = struct_ast.ident.as_inner().clone();
            Ok(Struct {
                ast: struct_ast,
                ident,
                members,
            })
        } else {
            Err(diagnostics)
        }
    }

    pub fn contains_ident(&self, ident: TemplateElaboratedIdent) -> bool {
        self.identifiers.contains_key(&ident.0.inner())
    }

    pub fn span_of(&self, ty: &Handle<Type>) -> Option<SimpleSpan> {
        self.types.get(ty).definition_span()
    }

    pub fn get_type(&self, ty: &Handle<Type>) -> &Type {
        self.types.get(ty)
    }

    pub fn get_raw_ident_type(&self, ident: &str) -> Result<Handle<Type>, Diagnostic> {
        self.identifiers
            .get(ident)
            .cloned()
            .ok_or(Diagnostic::error(format!(
                "Type: '{}' is not defined",
                ident
            )))
    }

    pub fn handle_of_ident(&self, ident: &Spanned<String>) -> Result<Handle<Type>, Diagnostic> {
        self.identifiers.get(&ident.inner).cloned().ok_or(
            Diagnostic::error(format!("Type: '{}' is not defined", ident.as_inner()))
                .span(ident.span()),
        )
    }

    pub fn apply_template_args(
        &mut self,
        handle: Handle<Type>,
        ident: &Spanned<TemplateElaboratedIdent>,
    ) -> Result<Handle<Type>, Vec<Diagnostic>> {
        let inner_type = self.apply_template_args_inner(handle, ident)?;
        Ok(self.types.insert(inner_type))
    }

    pub fn apply_template_args_inner(
        &mut self,
        handle: Handle<Type>,
        ident: &Spanned<TemplateElaboratedIdent>,
    ) -> Result<Type, Vec<Diagnostic>> {
        let args = ident.1.clone();
        let ty = self.types.get(&handle).clone();

        match ty {
            Type::Alias(Alias { alias_base, .. }) => {
                self.apply_template_args_inner(alias_base, ident)
            }
            Type::Plain(_) => match args {
                Some(args) => Err(
                    Diagnostic::error("Type does not take any template arguments")
                        .span(if let Some(arg) = args.0.first() {
                            arg.span()
                        } else {
                            SimpleSpan::new(0, 0)
                        })
                        .into(),
                ),
                None => Ok(ty.clone()),
            },
            Type::Generator(gen) => {
                let args = args.unwrap_or_default();
                let applied = gen.apply_template_args(self, args).map_err(|mut err| {
                    err[0] = err[0].clone().span_if_none(ident.span());
                    err
                })?;
                Ok(applied)
            }
        }
    }

    pub fn type_of_ident(
        &mut self,
        ident: &Spanned<TemplateElaboratedIdent>,
    ) -> Result<Handle<Type>, Vec<Diagnostic>> {
        let handle = self.handle_of_ident(&ident.0)?;
        self.apply_template_args(handle, ident)
    }
}

pub mod tests {
    #[test]
    pub fn matrix_type_aliases() {
        use super::*;
        let store = TypeStore::default();

        for (ident, ty) in [
            ("mat2x2f", Mat::Mat2(VecType::Vec2(Scalar::F32))),
            ("mat2x3f", Mat::Mat2(VecType::Vec3(Scalar::F32))),
            ("mat2x4f", Mat::Mat2(VecType::Vec4(Scalar::F32))),
            ("mat3x2f", Mat::Mat3(VecType::Vec2(Scalar::F32))),
            ("mat3x3f", Mat::Mat3(VecType::Vec3(Scalar::F32))),
            ("mat3x4f", Mat::Mat3(VecType::Vec4(Scalar::F32))),
            ("mat4x2f", Mat::Mat4(VecType::Vec2(Scalar::F32))),
            ("mat4x3f", Mat::Mat4(VecType::Vec3(Scalar::F32))),
            ("mat4x4f", Mat::Mat4(VecType::Vec4(Scalar::F32))),
            ("mat2x2h", Mat::Mat2(VecType::Vec2(Scalar::F16))),
            ("mat2x3h", Mat::Mat2(VecType::Vec3(Scalar::F16))),
            ("mat2x4h", Mat::Mat2(VecType::Vec4(Scalar::F16))),
            ("mat3x2h", Mat::Mat3(VecType::Vec2(Scalar::F16))),
            ("mat3x3h", Mat::Mat3(VecType::Vec3(Scalar::F16))),
            ("mat3x4h", Mat::Mat3(VecType::Vec4(Scalar::F16))),
            ("mat4x2h", Mat::Mat4(VecType::Vec2(Scalar::F16))),
            ("mat4x3h", Mat::Mat4(VecType::Vec3(Scalar::F16))),
            ("mat4x4h", Mat::Mat4(VecType::Vec4(Scalar::F16))),
        ] {
            let ty = Type::Plain(Plain::Mat(ty));
            let handle = store.get_raw_ident_type(ident).unwrap();
            assert_eq!(*store.types.get(&handle), ty);
        }
    }

    #[test]
    pub fn vector_type_aliases() {
        use super::*;
        let store = TypeStore::default();

        for (ident, ty) in [
            ("vec2i", VecType::Vec2(Scalar::I32)),
            ("vec3i", VecType::Vec3(Scalar::I32)),
            ("vec4i", VecType::Vec4(Scalar::I32)),
            ("vec2u", VecType::Vec2(Scalar::U32)),
            ("vec3u", VecType::Vec3(Scalar::U32)),
            ("vec4u", VecType::Vec4(Scalar::U32)),
            ("vec2f", VecType::Vec2(Scalar::F32)),
            ("vec3f", VecType::Vec3(Scalar::F32)),
            ("vec4f", VecType::Vec4(Scalar::F32)),
            ("vec2h", VecType::Vec2(Scalar::F16)),
            ("vec3h", VecType::Vec3(Scalar::F16)),
            ("vec4h", VecType::Vec4(Scalar::F16)),
        ] {
            let ty = Type::Plain(Plain::Vec(ty));

            let handle = store.get_raw_ident_type(ident).unwrap();

            assert_eq!(*store.types.get(&handle), ty);
        }
    }
}
