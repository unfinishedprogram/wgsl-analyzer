mod scope;
pub mod validate;

use self::scope::ScopeStore;

use super::r#type::Type;
use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::statement::{attribute::Attribute, declaration, Statement},
        span::Spanned,
    },
    module::{store::handle::Handle, type_store::TypeStore},
};
pub enum FunctionBody {
    Validated(ScopeStore),
    Unprocessed(Vec<Spanned<Statement>>),
}

pub struct FunctionParameter {
    pub attributes: Vec<Attribute>,
    pub ident: Spanned<String>,
    pub ty: Handle<Type>,
}

pub struct Function {
    pub is_builtin: bool,
    pub attributes: Vec<Attribute>,
    pub ident: Spanned<String>,
    pub parameters: Vec<FunctionParameter>,
    pub return_attributes: Vec<Attribute>,
    pub return_type: Option<Handle<Type>>,
    pub body: FunctionBody,
}

impl Function {
    pub fn unprocessed_from_ast(
        type_store: &mut TypeStore,
        ast_function: declaration::Function,
    ) -> Result<Self, Vec<Diagnostic>> {
        let mut parameters = vec![];
        for ast_parameter in ast_function.parameters {
            let ident = ast_parameter.ident;
            let attributes = ast_parameter.attributes;
            let ty = type_store.type_of_ident(&ast_parameter.value)?;

            parameters.push(FunctionParameter {
                attributes,
                ident,
                ty,
            });
        }

        let body = FunctionBody::Unprocessed(ast_function.body);
        let attributes = ast_function.attributes;
        let ident = ast_function.ident;

        let (return_type, return_attributes) = if let Some(return_type) = ast_function.return_type {
            let ty = type_store.type_of_ident(&return_type.1)?;
            (Some(ty), return_type.0)
        } else {
            (None, vec![])
        };

        Ok(Self {
            is_builtin: false,
            attributes,
            ident,
            parameters,
            return_attributes,
            return_type,
            body,
        })
    }
}
