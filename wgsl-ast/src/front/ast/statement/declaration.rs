use chumsky::prelude::*;

use super::attribute::Attribute;

use crate::front::{
    ast::{
        expression::{
            expression, template_elaborated_ident, template_list, Expression,
            TemplateElaboratedIdent, TemplateList,
        },
        ParserInput, RichErr,
    },
    span::{map_span, Spanned},
    token::{Keyword, Token},
};

use super::{compound_statement, optionally_typed_ident, OptionallyTypedIdent, Statement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub attributes: Vec<Attribute>,
    pub ident: Spanned<OptionallyTypedIdent>,
    pub scope: Option<TemplateList>,
    pub initial_value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleConstant {
    pub ident: Spanned<OptionallyTypedIdent>,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalConstant {
    pub ident: Spanned<OptionallyTypedIdent>,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias {
    pub ident: Spanned<String>,
    pub value: Spanned<TemplateElaboratedIdent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructMember {
    pub attributes: Vec<Attribute>,
    pub ident: Spanned<String>,
    pub value: Spanned<TemplateElaboratedIdent>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub attributes: Vec<Attribute>,
    pub ident: Spanned<String>,
    pub members: Vec<Spanned<StructMember>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub attributes: Vec<Attribute>,
    pub ident: Spanned<String>,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<(Vec<Attribute>, Spanned<TemplateElaboratedIdent>)>,
    pub body: Vec<Spanned<Statement>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Variable(Variable),
    ModuleConstant(ModuleConstant),
    LocalConstant(LocalConstant),
    TypeAlias(TypeAlias),
    Struct(Struct),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParameter {
    pub attributes: Vec<Attribute>,
    pub ident: Spanned<String>,
    pub value: Spanned<TemplateElaboratedIdent>,
}

pub fn variable_or_value_decl<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Declaration, RichErr<'src, 'tokens>> + Clone {
    let variable_decl = Attribute::list_parser()
        .then_ignore(just(Token::Keyword(Keyword::Var)))
        .then(template_list(expression()).or_not())
        .then(optionally_typed_ident(expression()))
        .then(
            just(Token::SyntaxToken("="))
                .ignore_then(expression())
                .or_not(),
        )
        .map(|(((attributes, scope), ident), initial_value)| {
            Declaration::Variable(Variable {
                attributes,
                scope,
                ident,
                initial_value,
            })
        })
        .boxed();

    let module_const = just(Token::Keyword(Keyword::Const))
        .ignore_then(optionally_typed_ident(expression()))
        .then(just(Token::SyntaxToken("=")).ignore_then(expression()))
        .map(|(ident, value)| Declaration::ModuleConstant(ModuleConstant { ident, value }));

    let local_const = just(Token::Keyword(Keyword::Let))
        .ignore_then(optionally_typed_ident(expression()))
        .then(just(Token::SyntaxToken("=")).ignore_then(expression()))
        .map(|(ident, value)| Declaration::LocalConstant(LocalConstant { ident, value }));

    choice((variable_decl, module_const, local_const)).boxed()
}

fn type_alias_decl<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Declaration, RichErr<'src, 'tokens>> + Clone {
    just(Token::Keyword(Keyword::Alias))
        .ignore_then(select!(Token::Ident(ident) => ident.to_owned()).map_with(map_span))
        .then(just(Token::SyntaxToken("=")).ignore_then(template_elaborated_ident(expression())))
        .map(|(ident, value)| Declaration::TypeAlias(TypeAlias { ident, value }))
        .boxed()
}

fn struct_decl<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Declaration, RichErr<'src, 'tokens>> + Clone {
    let struct_member = Attribute::list_parser()
        .then(
            select!(Token::Ident(ident) => ident.to_owned())
                .map_with(map_span)
                .then(
                    just(Token::SyntaxToken(":"))
                        .ignore_then(template_elaborated_ident(expression())),
                ),
        )
        .map(|(attributes, (ident, value))| StructMember {
            attributes,
            ident,
            value,
        })
        .map_with(map_span)
        .boxed();

    let struct_body = struct_member
        .separated_by(just(Token::SyntaxToken(",")))
        .allow_trailing()
        .collect()
        .delimited_by(just(Token::SyntaxToken("{")), just(Token::SyntaxToken("}")))
        .boxed();

    Attribute::list_parser()
        .then(
            just(Token::Keyword(Keyword::Struct))
                .ignore_then(select!(Token::Ident(ident) => ident.to_owned()).map_with(map_span))
                .then(struct_body),
        )
        .map(|(attributes, (ident, members))| {
            Declaration::Struct(Struct {
                attributes,
                ident,
                members,
            })
        })
        .boxed()
}

fn function_decl<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Declaration, RichErr<'src, 'tokens>> + Clone {
    let param = Attribute::list_parser()
        .then(
            select!(Token::Ident(ident) => ident.to_owned())
                .map_with(map_span)
                .then(
                    just(Token::SyntaxToken(":"))
                        .ignore_then(template_elaborated_ident(expression())),
                ),
        )
        .map(|(attributes, (ident, value))| FunctionParameter {
            attributes,
            ident,
            value,
        })
        .boxed();

    let param_list = param
        .separated_by(just(Token::SyntaxToken(",")))
        .allow_trailing()
        .collect()
        .delimited_by(just(Token::SyntaxToken("(")), just(Token::SyntaxToken(")")));

    let return_type = just(Token::SyntaxToken("->"))
        .ignore_then(Attribute::list_parser().then(template_elaborated_ident(expression())))
        .or_not()
        .boxed();

    Attribute::list_parser()
        .then(
            just(Token::Keyword(Keyword::Fn))
                .ignore_then(select!(Token::Ident(ident) => ident.to_owned()).map_with(map_span))
                .then(param_list)
                .then(return_type)
                .then(compound_statement(stmt)),
        )
        .map(|(attributes, (((ident, parameters), return_type), body))| {
            Declaration::Function(Function {
                attributes,
                ident,
                parameters,
                return_type,
                body,
            })
        })
        .boxed()
}

pub fn declaration<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Declaration, RichErr<'src, 'tokens>> + Clone {
    // TODO: Make semicolon shared parser.
    let semi = just(Token::SyntaxToken(";")).labelled("semicolon");
    choice((
        variable_or_value_decl().then_ignore(semi.clone()),
        type_alias_decl().then_ignore(semi.clone()),
        struct_decl(),
        function_decl(stmt),
    ))
    .labelled("declaration")
}
