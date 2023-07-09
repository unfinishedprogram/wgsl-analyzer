use std::ops::Index;

use naga::{Constant, Expression, Function, Handle, Type};

use super::{ErrorContext, LabelContext};

/*-----------
Error Context
-----------*/

impl Index<Handle<Function>> for ErrorContext<'_> {
    type Output = Function;

    fn index(&self, index: Handle<Function>) -> &Self::Output {
        &self.module.functions[index]
    }
}

impl Index<Handle<Type>> for ErrorContext<'_> {
    type Output = Type;

    fn index(&self, index: Handle<Type>) -> &Self::Output {
        &self.module.types[index]
    }
}

impl Index<Handle<Constant>> for ErrorContext<'_> {
    type Output = Constant;

    fn index(&self, index: Handle<Constant>) -> &Self::Output {
        &self.module.constants[index]
    }
}

/*-----------
Label Context
-----------*/

impl Index<Handle<Function>> for LabelContext<'_> {
    type Output = Function;

    fn index(&self, index: Handle<Function>) -> &Self::Output {
        &self.error_context[index]
    }
}

impl Index<Handle<Expression>> for LabelContext<'_> {
    type Output = Expression;

    fn index(&self, index: Handle<Expression>) -> &Self::Output {
        &self[self.function].expressions[index]
    }
}

impl Index<Handle<Type>> for LabelContext<'_> {
    type Output = Type;

    fn index(&self, index: Handle<Type>) -> &Self::Output {
        &self.error_context[index]
    }
}

impl Index<Handle<Constant>> for LabelContext<'_> {
    type Output = Constant;

    fn index(&self, index: Handle<Constant>) -> &Self::Output {
        &self.error_context[index]
    }
}
