mod builtin;
mod scope;
pub mod validate;

use self::{builtin::Builtin, scope::ScopeStore};

use super::r#type::Type;
use crate::{
    diagnostic::Diagnostic,
    front::{
        ast::statement::{attribute::Attribute, declaration, Statement},
        span::{SpanAble, Spanned},
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

pub struct UserDefined {
    pub attributes: Vec<Attribute>,
    pub ident: Spanned<String>,
    pub parameters: Vec<FunctionParameter>,
    pub return_attributes: Vec<Attribute>,
    pub return_type: Option<Handle<Type>>,
    pub body: FunctionBody,
}

pub enum Function {
    UserDefined(Spanned<UserDefined>),
    Builtin(builtin::Builtin),
}

impl Function {
    pub fn get_all_builtin_functions(type_store: &TypeStore) -> Result<Vec<Function>, Diagnostic> {
        let res = Builtin::get_all_builtin_functions(type_store)?;
        Ok(res.into_iter().map(Function::Builtin).collect())
    }

    pub fn unprocessed_from_ast(
        type_store: &mut TypeStore,
        ast_function: Spanned<declaration::Function>,
    ) -> Result<Self, Vec<Diagnostic>> {
        let mut parameters = vec![];
        let ast_span = ast_function.span;
        let ast_function = ast_function.inner;

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

        Ok(Self::UserDefined(
            UserDefined {
                attributes,
                ident,
                parameters,
                return_attributes,
                return_type,
                body,
            }
            .with_span(ast_span),
        ))
    }
}
