pub mod attribute;
pub mod declaration;

use chumsky::prelude::*;

use crate::front::{
    span::{map_span, SpanAble, Spanned},
    token::Keyword,
};

use self::declaration::{declaration, variable_or_value_decl, Declaration};
use attribute::Attribute;

use super::{
    expression::{
        call_expression, expression, lhs_expression,
        relational_expression::{
            AdditiveOperator, BinaryOperator, BitwiseOperator, MultiplicativeOperator,
            ShiftOperator,
        },
        template_elaborated_ident, CallPhrase, Expression, LHSExpression, TemplateElaboratedIdent,
    },
    ParserInput, RichErr, Token,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Trivia,
    Compound(Vec<Statement>),
    Assignment(LHSExpression, AssignmentOperator, Expression),
    Increment(LHSExpression),
    Decrement(LHSExpression),
    Return(Option<Expression>),
    Continue,
    Continuing(Vec<Attribute>, Vec<Statement>),
    Break,
    BreakIf(Expression),
    If {
        if_block: (Expression, Vec<Statement>),
        else_if_blocks: Vec<(Expression, Vec<Statement>)>,
        else_block: Option<Vec<Statement>>,
    },
    Declaration(Declaration),
    FuncCall(CallPhrase),
    Discard,
    Loop {
        loop_attributes: Vec<Attribute>,
        body_attributes: Vec<Attribute>,
        body: Vec<Statement>,
    },
    For {
        attributes: Vec<Attribute>,
        init: Box<Option<Statement>>,
        expression: Box<Option<Expression>>,
        update: Box<Option<Statement>>,
        body: Vec<Statement>,
    },
    While {
        attributes: Vec<Attribute>,
        expression: Expression,
        body: Vec<Statement>,
    },
    Switch {
        attributes: Vec<Attribute>,
        expression: Expression,
        body: Vec<SwitchClause>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchClause {
    Case(Vec<CaseClause>, Vec<Statement>),
    Default(Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseClause {
    Expression(Expression),
    Default,
}

impl CaseClause {
    pub fn parser<'tokens, 'src: 'tokens>(
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Self, RichErr<'src, 'tokens>> + Clone
    {
        let case = expression().map(CaseClause::Expression);

        let default = just(Token::Keyword(Keyword::Default)).to(CaseClause::Default);

        choice((case, default)).labelled("case clause")
    }
}

impl SwitchClause {
    pub fn parser<'tokens, 'src: 'tokens>(
        stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
            + Clone
            + 'tokens,
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Self, RichErr<'src, 'tokens>> + Clone
    {
        let case_clause = just(Token::Keyword(Keyword::Case))
            .ignore_then(
                CaseClause::parser()
                    .separated_by(just(Token::SyntaxToken(",")))
                    .allow_leading()
                    .at_least(1)
                    .collect(),
            )
            .then_ignore(just(Token::SyntaxToken(":")).or_not())
            .then(compound_statement(stmt.clone()))
            .map(|(cases, body)| SwitchClause::Case(cases, body));

        let default_clause = just(Token::Keyword(Keyword::Default))
            .ignore_then(just(Token::SyntaxToken(":")).or_not())
            .ignore_then(compound_statement(stmt.clone()))
            .map(SwitchClause::Default);

        choice((case_clause, default_clause)).labelled("switch clause")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentOperator {
    Simple,
    Compound(BinaryOperator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionallyTypedIdent(pub String, pub Option<Spanned<TemplateElaboratedIdent>>);

fn assignment_operator<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, AssignmentOperator, RichErr<'src, 'tokens>> + Clone
{
    select! {
        Token::SyntaxToken("=") => AssignmentOperator::Simple,
        Token::SyntaxToken("+=") => AssignmentOperator::Compound(BinaryOperator::Additive(AdditiveOperator::Plus)),
        Token::SyntaxToken("-=") => AssignmentOperator::Compound(BinaryOperator::Additive(AdditiveOperator::Minus)),
        Token::SyntaxToken("*=") => AssignmentOperator::Compound(BinaryOperator::Multiplicative(MultiplicativeOperator::Multiply)),
        Token::SyntaxToken("/=") => AssignmentOperator::Compound(BinaryOperator::Multiplicative(MultiplicativeOperator::Divide)),
        Token::SyntaxToken("%=") => AssignmentOperator::Compound(BinaryOperator::Multiplicative(MultiplicativeOperator::Modulo)),
        Token::SyntaxToken("&=") => AssignmentOperator::Compound(BinaryOperator::Bitwise(BitwiseOperator::And)),
        Token::SyntaxToken("|=") => AssignmentOperator::Compound(BinaryOperator::Bitwise(BitwiseOperator::Or)),
        Token::SyntaxToken("^=") => AssignmentOperator::Compound(BinaryOperator::Bitwise(BitwiseOperator::Xor)),
        Token::SyntaxToken(">>=") => AssignmentOperator::Compound(BinaryOperator::Shift(ShiftOperator::Right)),
        Token::SyntaxToken("<<=") => AssignmentOperator::Compound(BinaryOperator::Shift(ShiftOperator::Left)),
    }
}

fn assignment_statement<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    lhs_expression()
        .then(assignment_operator())
        .then(expression())
        .map(|((lhs, operator), rhs)| Statement::Assignment(lhs, operator, rhs))
        .labelled("assignment statement")
        .boxed()
}

fn inc_dec_statement<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    lhs_expression()
        .then(choice((
            just(Token::SyntaxToken("++")),
            just(Token::SyntaxToken("--")),
        )))
        .map(|(lhs, op)| match op {
            Token::SyntaxToken("++") => Statement::Increment(lhs),
            Token::SyntaxToken("--") => Statement::Decrement(lhs),
            _ => unreachable!(),
        })
        .labelled("increment/decrement statement")
        .boxed()
}

fn return_statement<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    just(Token::Keyword(Keyword::Return))
        .ignore_then(expression().or_not())
        .map(Statement::Return)
        .labelled("return statement")
        .boxed()
}

fn discard_statement<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    just(Token::Keyword(Keyword::Discard))
        .to(Statement::Discard)
        .labelled("discard statement")
        .boxed()
}

fn break_statement<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    let just_break = just(Token::Keyword(Keyword::Break)).to(Statement::Break);

    let break_if = just(Token::Keyword(Keyword::Break))
        .ignore_then(just(Token::Keyword(Keyword::If)))
        .ignore_then(expression())
        .map(Statement::BreakIf);

    choice((break_if, just_break))
        .labelled("break statement")
        .boxed()
}

fn continue_statement<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    just(Token::Keyword(Keyword::Continue))
        .to(Statement::Continue)
        .labelled("continue statement")
        .boxed()
}

fn compound_statement<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Vec<Statement>, RichErr<'src, 'tokens>> + Clone
{
    just(Token::Trivia)
        .or_not()
        .ignore_then(stmt)
        .then_ignore(just(Token::Trivia).or_not())
        .repeated()
        .collect()
        .delimited_by(just(Token::SyntaxToken("{")), just(Token::SyntaxToken("}")))
        .labelled("compound statement")
        .boxed()
}

fn loop_statement<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    let loop_attributes = Attribute::list_parser();
    let body_attributes = Attribute::list_parser();

    loop_attributes
        .then(
            just(Token::Keyword(Keyword::Loop))
                .ignore_then(body_attributes.then(compound_statement(stmt.clone()))),
        )
        .map(
            |(loop_attributes, (body_attributes, body))| Statement::Loop {
                loop_attributes,
                body_attributes,
                body,
            },
        )
        .labelled("loop statement")
        .boxed()
}

fn for_statement<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    let init = choice((
        variable_or_value_decl().map(Statement::Declaration),
        assignment_statement(),
        inc_dec_statement(),
        call_expression(expression()).map(Statement::FuncCall),
    ))
    .boxed();

    let update = choice((
        call_expression(expression()).map(Statement::FuncCall),
        assignment_statement(),
        inc_dec_statement(),
    ))
    .boxed();

    let header = init
        .or_not()
        .then_ignore(just(Token::SyntaxToken(";")))
        .then(expression().or_not())
        .then_ignore(just(Token::SyntaxToken(";")))
        .then(update.or_not())
        .map(|((a, b), c)| (a, b, c))
        .boxed();

    Attribute::list_parser()
        .then_ignore(just(Token::Keyword(Keyword::For)))
        .then(header.delimited_by(just(Token::SyntaxToken("(")), just(Token::SyntaxToken(")"))))
        .then(compound_statement(stmt))
        .map(
            |((attributes, (init, expression, update)), body)| Statement::For {
                attributes,
                init: Box::new(init),
                expression: Box::new(expression),
                update: Box::new(update),
                body,
            },
        )
        .labelled("for statement")
        .boxed()
}

fn while_statement<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    Attribute::list_parser()
        .then_ignore(just(Token::Keyword(Keyword::While)))
        .then(expression())
        .then(compound_statement(stmt.clone()))
        .map(|((attributes, expression), body)| Statement::While {
            attributes,
            expression,
            body,
        })
        .labelled("while statement")
        .boxed()
}

fn continuing_statement<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    Attribute::list_parser()
        .then_ignore(just(Token::Keyword(Keyword::Continuing)))
        .then(compound_statement(stmt.clone()))
        .map(|(attributes, body)| Statement::Continuing(attributes, body))
        .labelled("continuing statement")
        .boxed()
}

fn if_statement<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    let if_clause = just(Token::Keyword(Keyword::If))
        .ignore_then(expression())
        .then(compound_statement(stmt.clone()))
        .boxed();

    let else_if_clause = just(Token::Keyword(Keyword::Else))
        .ignore_then(just(Token::Keyword(Keyword::If)))
        .ignore_then(expression())
        .then(compound_statement(stmt.clone()))
        .boxed();

    let else_clause =
        just(Token::Keyword(Keyword::Else)).ignore_then(compound_statement(stmt.clone()));

    if_clause
        .then(else_if_clause.repeated().collect())
        .then(else_clause.or_not())
        .map(|((if_block, else_if_blocks), else_block)| Statement::If {
            if_block,
            else_if_blocks,
            else_block,
        })
        .labelled("if statement")
        .boxed()
}

fn switch_statement<'tokens, 'src: 'tokens>(
    stmt: impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Statement, RichErr<'src, 'tokens>> + Clone {
    Attribute::list_parser()
        .then_ignore(just(Token::Keyword(Keyword::Switch)))
        .then(expression())
        .then(
            SwitchClause::parser(stmt)
                .repeated()
                .collect()
                .delimited_by(just(Token::SyntaxToken("{")), just(Token::SyntaxToken("}"))),
        )
        .map(|((attributes, expression), body)| Statement::Switch {
            attributes,
            expression,
            body,
        })
        .labelled("switch statement")
        .boxed()
}

fn optionally_typed_ident<'tokens, 'src: 'tokens>(
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Expression, RichErr<'src, 'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<OptionallyTypedIdent>,
    RichErr<'src, 'tokens>,
> + Clone {
    let type_specifier = template_elaborated_ident(expr);

    select!(Token::Ident(ident) => ident.to_owned())
        .then(
            just(Token::SyntaxToken(":"))
                .ignore_then(type_specifier)
                .or_not(),
        )
        .map(|(ident, type_specifier)| OptionallyTypedIdent(ident, type_specifier))
        .map_with(map_span)
        .boxed()
}

pub fn statement<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Statement>, RichErr<'src, 'tokens>> + Clone
{
    recursive(|this| {
        choice((
            if_statement(this.clone()),
            loop_statement(this.clone()),
            for_statement(this.clone()),
            while_statement(this.clone()),
            declaration(this.clone()).map(Statement::Declaration),
            continuing_statement(this.clone()),
            switch_statement(this.clone()),
            compound_statement(this.clone()).map(Statement::Compound),
            choice((
                inc_dec_statement(),
                call_expression(expression()).map(Statement::FuncCall),
                assignment_statement(),
                return_statement(),
                discard_statement(),
                break_statement(),
                continue_statement(),
                break_statement(),
            ))
            .then_ignore(just(Token::SyntaxToken(";"))),
        ))
        .memoized()
    })
    .map_with(|stmt, e| stmt.with_span(e.span()))
    .labelled("statement")
    .boxed()
}
