use std::ops::Index;

use naga::{Constant, Expression, Function, Handle, Type};

use super::{DiagnosticContext, FunctionErrorContext};

/*-----------
Error Context
-----------*/

impl Index<Handle<Function>> for DiagnosticContext<'_> {
    type Output = Function;

    fn index(&self, index: Handle<Function>) -> &Self::Output {
        &self.module.functions[index]
    }
}

impl Index<Handle<Type>> for DiagnosticContext<'_> {
    type Output = Type;

    fn index(&self, index: Handle<Type>) -> &Self::Output {
        &self.module.types[index]
    }
}

impl Index<Handle<Constant>> for DiagnosticContext<'_> {
    type Output = Constant;

    fn index(&self, index: Handle<Constant>) -> &Self::Output {
        &self.module.constants[index]
    }
}

/*-----------
Label Context
-----------*/

impl Index<Handle<Function>> for FunctionErrorContext<'_> {
    type Output = Function;

    fn index(&self, index: Handle<Function>) -> &Self::Output {
        &self.module().functions[index]
    }
}

impl Index<Handle<Expression>> for FunctionErrorContext<'_> {
    type Output = Expression;

    fn index(&self, index: Handle<Expression>) -> &Self::Output {
        &self[self.function].expressions[index]
    }
}

impl Index<Handle<Type>> for FunctionErrorContext<'_> {
    type Output = Type;

    fn index(&self, index: Handle<Type>) -> &Self::Output {
        &self.module().types[index]
    }
}

impl Index<Handle<Constant>> for FunctionErrorContext<'_> {
    type Output = Constant;

    fn index(&self, index: Handle<Constant>) -> &Self::Output {
        &self.module().constants[index]
    }
}
