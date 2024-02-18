use chumsky::prelude::*;

mod lhs_expression;
pub mod relational_expression;
pub use lhs_expression::{core_lhs_expression, lhs_expression, LHSExpression};

use self::relational_expression::{
    AdditiveOperator, BinaryOperator, BitwiseOperator, MultiplicativeOperator, RelationalOperator,
    ShiftOperator, ShortCircuitOperator, UnaryOperator,
};

use super::{ParserInput, RichErr};
use crate::front::{
    span::{map_span, SpanAble, Spanned, WithSpan},
    token::{Literal, Token},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComponentOrSwizzleSpecifierInner {
    IndexExpression(Box<Expression>),
    MemberAccess(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentOrSwizzleSpecifier(
    ComponentOrSwizzleSpecifierInner,
    Option<Box<ComponentOrSwizzleSpecifier>>,
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateElaboratedIdent(pub String, pub Option<TemplateList>);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateList(pub Vec<Expression>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallPhrase(Spanned<TemplateElaboratedIdent>, ArgumentExpressionList);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentExpressionList(Vec<Expression>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionInner {
    None,
    Ident(Spanned<TemplateElaboratedIdent>),
    CallExpression(CallPhrase),
    Literal(Literal),
    ParenExpression(Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Singular(Box<Expression>, ComponentOrSwizzleSpecifier),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}

pub type Expression = Spanned<ExpressionInner>;

pub fn st<'tokens, 'src: 'tokens>(
    t: &'src str,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Token, RichErr<'src, 'tokens>> + Clone {
    just(Token::SyntaxToken(t))
}

pub fn component_or_swizzle_specifier<'tokens, 'src: 'tokens>(
    expression: impl Parser<'tokens, ParserInput<'tokens, 'src>, Expression, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    ComponentOrSwizzleSpecifier,
    RichErr<'src, 'tokens>,
> + Clone {
    let ident_str = select!(Token::Ident(ident) => ident.to_owned());

    recursive(|this| {
        choice((
            expression
                .delimited_by(st("["), st("]"))
                .map(Box::new)
                .map(ComponentOrSwizzleSpecifierInner::IndexExpression), // | `'['` expression `']'` component_or_swizzle_specifier ?
            st(".")
                .ignore_then(ident_str)
                .map(ComponentOrSwizzleSpecifierInner::MemberAccess), // | `'.'` swizzle_name component_or_swizzle_specifier ?
            st(".")
                .ignore_then(ident_str)
                .map(ComponentOrSwizzleSpecifierInner::MemberAccess), // | `'.'` member_ident component_or_swizzle_specifier ?
        ))
        .then(this.or_not())
        .map(|(inner, next)| ComponentOrSwizzleSpecifier(inner, next.map(Box::new)))
    })
    .boxed()
}

pub fn expression_inner<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, ExpressionInner, RichErr<'src, 'tokens>> + Clone
{
    let binary_span = |lhs: Expression, (op, rhs): (BinaryOperator, Expression)| {
        let s = lhs.span().union(rhs.span());
        ExpressionInner::Binary(Box::new(lhs), op, Box::new(rhs)).with_span(s)
    };

    recursive(|expression| {
        let spanned_expr = expression
            .clone()
            .map_with(|expr: ExpressionInner, e| expr.with_span(e.span()));

        let primary_expression = choice((
            select!(Token::Literal(lit) => ExpressionInner::Literal(lit)),
            primary_expression(spanned_expr.clone())
                .then(component_or_swizzle_specifier(spanned_expr.clone()).or_not())
                .map(|(expr, comp_or_swizz)| match comp_or_swizz {
                    Some(comp_or_swizz) => ExpressionInner::Singular(Box::new(expr), comp_or_swizz),
                    None => expr.inner,
                }),
        ))
        .boxed();

        let spanned_primary = primary_expression
            .clone()
            .map_with(|expr, e| expr.with_span(e.span()));

        let unary_expression = {
            let unary_op = choice((
                st("!").to(UnaryOperator::Not),
                st("&").to(UnaryOperator::AddrOf),
                st("*").to(UnaryOperator::Deref),
                st("-").to(UnaryOperator::Negative),
                st("~").to(UnaryOperator::BitNot),
            ))
            .map_with(|op, e| op.with_span(e.span()))
            .boxed();

            unary_op.repeated().foldr(
                spanned_primary,
                |op: Spanned<UnaryOperator>, expr: Expression| -> Expression {
                    let s = op.span().union(expr.span());
                    let expr = ExpressionInner::Unary(op.inner, Box::new(expr));
                    expr.with_span(s)
                },
            )
        }
        .boxed();

        let bitwise_expression_post_unary_expression = {
            let bit_and = st("&").to(BinaryOperator::Bitwise(BitwiseOperator::And));
            let bit_xor = st("^").to(BinaryOperator::Bitwise(BitwiseOperator::Xor));
            let bit_or = st("|").to(BinaryOperator::Bitwise(BitwiseOperator::Or));

            choice((
                unary_expression.clone().foldl(
                    bit_and
                        .then(unary_expression.clone())
                        .repeated()
                        .at_least(1),
                    binary_span,
                ),
                unary_expression.clone().foldl(
                    bit_xor
                        .then(unary_expression.clone())
                        .repeated()
                        .at_least(1),
                    binary_span,
                ),
                unary_expression.clone().foldl(
                    bit_or.then(unary_expression.clone()).repeated().at_least(1),
                    binary_span,
                ),
            ))
        }
        .boxed();

        let shift_expression_post_unary_expression = {
            let multiplicative_operator = choice((
                st("*").to(BinaryOperator::Multiplicative(
                    MultiplicativeOperator::Multiply,
                )),
                st("/").to(BinaryOperator::Multiplicative(
                    MultiplicativeOperator::Divide,
                )),
                st("%").to(BinaryOperator::Multiplicative(
                    MultiplicativeOperator::Modulo,
                )),
            ));

            let additive_operator = choice((
                st("+").to(BinaryOperator::Additive(AdditiveOperator::Plus)),
                st("-").to(BinaryOperator::Additive(AdditiveOperator::Minus)),
            ));

            let shift_operator = choice((
                st(">>").to(BinaryOperator::Shift(ShiftOperator::Right)),
                st("<<").to(BinaryOperator::Shift(ShiftOperator::Left)),
            ));

            let multiplicative_fold = unary_expression
                .clone()
                .foldl(
                    multiplicative_operator
                        .then(unary_expression.clone())
                        .repeated(),
                    binary_span,
                )
                .boxed();

            let additive_fold = multiplicative_fold
                .clone()
                .foldl(
                    additive_operator
                        .then(multiplicative_fold.clone())
                        .repeated(),
                    binary_span,
                )
                .boxed();

            let shift = unary_expression
                .clone()
                .then(shift_operator)
                .then(unary_expression.clone())
                .map(
                    |((expr1, op), expr2): ((Expression, BinaryOperator), Expression)| {
                        ExpressionInner::Binary(Box::new(expr1), op, Box::new(expr2))
                    },
                )
                .map_with(|expr, e| expr.with_span(e.span()))
                .boxed();

            choice((shift, additive_fold))
        }
        .boxed();

        let relational_expression_post_unary_expression = {
            use RelationalOperator::*;

            let ops = choice((
                st("==").to(Equal),
                st("!=").to(NotEqual),
                st("<=").to(LessThanEqual),
                st("<").to(LessThan),
                st(">=").to(GreaterThanEqual),
                st(">").to(GreaterThan),
            ));

            let relational_ops = shift_expression_post_unary_expression
                .clone()
                .then(ops)
                .then(shift_expression_post_unary_expression.clone())
                .map(|((lhs, op), rhs)| {
                    ExpressionInner::Binary(
                        Box::new(lhs),
                        BinaryOperator::Relational(op),
                        Box::new(rhs),
                    )
                })
                .map_with(|expr, e| expr.with_span(e.span()))
                .boxed();

            choice((relational_ops, shift_expression_post_unary_expression))
        }
        .boxed();

        let relational_and = relational_expression_post_unary_expression
            .clone()
            .foldl(
                st("&&")
                    .to(ShortCircuitOperator::And)
                    .then(relational_expression_post_unary_expression.clone())
                    .repeated()
                    .at_least(1),
                |prev, (op, next)| {
                    ExpressionInner::Binary(
                        Box::new(prev.clone()),
                        BinaryOperator::ShortCircuit(op),
                        Box::new(next.clone()),
                    )
                    .with_span(prev.span().union(next.span()))
                },
            )
            .boxed();

        let relational_or = relational_expression_post_unary_expression
            .clone()
            .foldl(
                st("||")
                    .to(ShortCircuitOperator::Or)
                    .then(relational_expression_post_unary_expression.clone())
                    .repeated()
                    .at_least(1),
                |prev, (op, next)| {
                    ExpressionInner::Binary(
                        Box::new(prev.clone()),
                        BinaryOperator::ShortCircuit(op),
                        Box::new(next.clone()),
                    )
                    .with_span(prev.span().union(next.span()))
                },
            )
            .boxed();

        let relational_exprs = choice((relational_and, relational_or)).boxed();

        // Expression
        choice((
            relational_exprs,
            bitwise_expression_post_unary_expression,
            relational_expression_post_unary_expression,
            unary_expression,
        ))
        .map(|e| e.inner())
    })
    .boxed()
}

pub fn expression<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Expression, RichErr<'src, 'tokens>> + Clone {
    expression_inner().map_with(|expr, e| expr.with_span(e.span()))
}

pub fn template_list<'tokens, 'src: 'tokens>(
    expression: impl Parser<'tokens, ParserInput<'tokens, 'src>, Expression, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, TemplateList, RichErr<'src, 'tokens>> + Clone
{
    expression
        .clone()
        .separated_by(just(Token::SyntaxToken(",")))
        .allow_trailing()
        .at_least(1)
        .collect()
        .delimited_by(just(Token::TemplateArgsStart), just(Token::TemplateArgsEnd))
        .map(TemplateList)
}

pub fn template_elaborated_ident<'tokens, 'src: 'tokens>(
    expression: impl Parser<'tokens, ParserInput<'tokens, 'src>, Expression, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<TemplateElaboratedIdent>,
    RichErr<'src, 'tokens>,
> + Clone {
    let ident = select!(Token::Ident(ident) => ident.to_owned());
    let template_list = template_list(expression);

    ident
        .then(template_list.or_not())
        .map(|(ident, template)| TemplateElaboratedIdent(ident, template))
        .map_with(map_span)
}

pub fn call_expression<'tokens, 'src: 'tokens>(
    expression: impl Parser<'tokens, ParserInput<'tokens, 'src>, Expression, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, CallPhrase, RichErr<'src, 'tokens>> + Clone {
    template_elaborated_ident(expression.clone())
        .then(
            expression
                .separated_by(st(","))
                .allow_trailing()
                .collect()
                .delimited_by(st("("), st(")"))
                .map(ArgumentExpressionList),
        )
        .map(|(ident, args)| CallPhrase(ident, args))
}

#[allow(non_snake_case)]
pub fn primary_expression<'tokens, 'src: 'tokens>(
    expression: impl Parser<'tokens, ParserInput<'tokens, 'src>, Expression, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Expression, RichErr<'src, 'tokens>> + Clone {
    let paren_expression = expression
        .clone()
        .delimited_by(st("("), st(")"))
        .map(Box::new)
        .map(ExpressionInner::ParenExpression);

    let literal = select!(Token::Literal(lit) => ExpressionInner::Literal(lit));

    choice((
        paren_expression,
        literal,
        call_expression(expression.clone()).map(ExpressionInner::CallExpression),
        template_elaborated_ident(expression.clone()).map(ExpressionInner::Ident),
    ))
    .map_with(map_span)
}
