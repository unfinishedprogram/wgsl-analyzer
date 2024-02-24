use std::collections::HashMap;

use chumsky::span::SimpleSpan;

use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::{expression::TemplateElaboratedIdent, statement::declaration::Declaration},
        span::{SpanAble, Spanned, WithSpan},
    },
};

use super::{
    declaration::r#type::{Alias, Plain, Scalar, Struct, StructMember, Type},
    store::{handle::Handle, Arena, Store},
};

pub struct TypeStore {
    pub types: Arena<Type>,
    pub identifiers: HashMap<String, Handle<Type>>,
}

impl TypeStore {
    pub fn new() -> Self {
        // Initialize the store with pre-declared types and type-aliases
        let types = Arena::default();
        let identifiers = HashMap::new();

        let mut res = Self { types, identifiers };
        res.init();
        res
    }

    pub fn add_with_alias(&mut self, ident: impl Into<String>, ty: Type) {
        let handle = self.types.insert(ty);
        self.identifiers.insert(ident.into(), handle);
    }

    pub fn init(&mut self) {
        self.add_with_alias("bool", Type::Plain(Plain::Scalar(Scalar::Boolean)));
        self.add_with_alias("i32", Type::Plain(Plain::Scalar(Scalar::I32)));
        self.add_with_alias("u32", Type::Plain(Plain::Scalar(Scalar::U32)));
        self.add_with_alias("f32", Type::Plain(Plain::Scalar(Scalar::F32)));
        self.add_with_alias("f16", Type::Plain(Plain::Scalar(Scalar::F16)));
    }

    pub fn insert_declarations(
        &mut self,
        declarations: Vec<Spanned<Declaration>>,
    ) -> Result<(), Vec<Diagnostic>> {
        // Since module declarations can appear out of order, we need to resolve them later
        let mut to_resolve: Vec<Spanned<Declaration>> = vec![];
        let mut diagnostics: Vec<Diagnostic> = vec![];

        for decl in declarations {
            match decl.inner {
                Declaration::TypeAlias(ty) => {
                    if let Some(diag) = self.validate_no_conflicting_definitions(ty.ident.clone()) {
                        diagnostics.push(diag);
                    } else {
                        self.add_with_alias(
                            &ty.ident.inner,
                            Type::Alias(Alias {
                                ast: ty.clone().with_span(decl.span),
                                ident: ty.ident.inner.clone(),
                                alias_base: Handle::new(0),
                                template_args: vec![],
                            }),
                        );
                    }
                }
                Declaration::Struct(s) => match self.struct_from_ast(s.with_span(decl.span)) {
                    Ok(s) => self.add_with_alias(s.ident.clone(), Type::Plain(Plain::Struct(s))),
                    Err(mut e) => diagnostics.append(&mut e),
                },
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
        match self.identifiers.get(&ident.inner) {
            Some(handle) => match self.span_of(handle) {
                Some(span) => {
                    let diag = Diagnostic::error(
                        format!("Type '{}' is already defined", ident.inner),
                        span,
                    )
                    .related("Conflicting declaration here", span);
                    Some(diag)
                }
                None => {
                    // Must be a builtin type
                    let diag = Diagnostic::error(
                        format!("Type '{}' is already defined as a builtin", ident.inner),
                        ident.span,
                    );

                    Some(diag)
                }
            },
            None => None,
        }
    }

    pub fn struct_from_ast(
        &self,
        struct_ast: Spanned<crate::front::ast::statement::declaration::Struct>,
    ) -> Result<Struct, Vec<Diagnostic>> {
        let mut diagnostics = vec![];
        let mut members: Vec<StructMember> = vec![];

        for member in struct_ast.inner.members.iter() {
            let ty = &member.value;
            let ident = &member.ident;

            // Check that this name is not already defined in this struct
            for defined in &members {
                if &defined.ident == ident.as_inner() {
                    diagnostics.push(
                        Diagnostic::error(
                            format!(
                                "Member '{}' already exists on struct '{}'",
                                ident.as_inner(),
                                struct_ast.ident.as_inner()
                            ),
                            member.span(),
                        )
                        .related("Other member defined here", defined.ast.span),
                    )
                }
            }

            match self.handle_of_ident(ty.0.clone().with_span(ty.span())) {
                Ok(ty) => members.push(StructMember {
                    ast: member.clone(),
                    ident: member.ident.as_inner().clone(),
                    ty,
                }),
                Err(diag) => diagnostics.push(diag),
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
        self.identifiers.contains_key(&ident.0)
    }

    pub fn span_of(&self, ty: &Handle<Type>) -> Option<SimpleSpan> {
        self.types.get(ty).definition_span()
    }

    pub fn handle_of_ident(&self, ident: Spanned<String>) -> Result<Handle<Type>, Diagnostic> {
        self.identifiers
            .get(&ident.inner)
            .cloned()
            .ok_or(Diagnostic::error(
                format!("Identifier: '{}' is not defined", ident.as_inner()),
                ident.span(),
            ))
    }

    pub fn type_of_ident(
        &self,
        ident: &Spanned<TemplateElaboratedIdent>,
    ) -> Result<Type, Diagnostic> {
        self.handle_of_ident(ident.0.clone().with_span(ident.span()))
            .map(|handle| self.types.get(&handle).to_owned())
    }
}
