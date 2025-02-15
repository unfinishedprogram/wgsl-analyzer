use std::ops::Index;

use naga::{Constant, Expression, Function, Handle, Type};

use super::{FunctionContext, ModuleContext};

/*-----------
Error Context
-----------*/

impl Index<Handle<Function>> for ModuleContext<'_> {
    type Output = Function;

    fn index(&self, index: Handle<Function>) -> &Self::Output {
        &self.module.functions[index]
    }
}

impl Index<Handle<Type>> for ModuleContext<'_> {
    type Output = Type;

    fn index(&self, index: Handle<Type>) -> &Self::Output {
        &self.module.types[index]
    }
}

impl Index<Handle<Constant>> for ModuleContext<'_> {
    type Output = Constant;

    fn index(&self, index: Handle<Constant>) -> &Self::Output {
        &self.module.constants[index]
    }
}

/*-----------
Label Context
-----------*/

impl Index<Handle<Function>> for FunctionContext<'_> {
    type Output = Function;

    fn index(&self, index: Handle<Function>) -> &Self::Output {
        &self.module().functions[index]
    }
}

impl Index<Handle<Expression>> for FunctionContext<'_> {
    type Output = Expression;

    fn index(&self, index: Handle<Expression>) -> &Self::Output {
        &self.function.expressions[index]
    }
}

impl Index<Handle<Type>> for FunctionContext<'_> {
    type Output = Type;

    fn index(&self, index: Handle<Type>) -> &Self::Output {
        &self.module().types[index]
    }
}

impl Index<Handle<Constant>> for FunctionContext<'_> {
    type Output = Constant;

    fn index(&self, index: Handle<Constant>) -> &Self::Output {
        &self.module().constants[index]
    }
}
