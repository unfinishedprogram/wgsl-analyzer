use naga::{Constant, Expression, GlobalVariable, Handle, LocalVariable, Type};

use super::{FunctionContext, ModuleContext, as_type::AsType};

pub enum GlobalIdentQueryResult {
    GlobalVariable(Handle<GlobalVariable>),
    Constant(Handle<Constant>),
}

pub enum FunctionIdentQueryResult {
    LocalVariable(Handle<LocalVariable>),
    NamedExpression(Handle<Expression>),
    Global(GlobalIdentQueryResult),
}

impl ModuleContext<'_> {
    fn get_ident_by_name(&self, name: &str) -> Option<GlobalIdentQueryResult> {
        if let Some((handle, _)) = &self
            .module
            .constants
            .iter()
            .find(|(_, constant)| constant.name.as_ref().is_some_and(|it| it == name))
        {
            return Some(GlobalIdentQueryResult::Constant(*handle));
        }

        if let Some((handle, _)) = self
            .module
            .global_variables
            .iter()
            .find(|(_, global)| global.name.as_ref().is_some_and(|it| it == name))
        {
            return Some(GlobalIdentQueryResult::GlobalVariable(handle));
        }

        None
    }

    fn get_type_of_ident(&self, ident: &GlobalIdentQueryResult) -> Type {
        match ident {
            GlobalIdentQueryResult::Constant(handle) => {
                self.module.types[self.module.constants[*handle].ty].clone()
            }
            GlobalIdentQueryResult::GlobalVariable(handle) => {
                self.module.types[self.module.global_variables[*handle].ty].clone()
            }
        }
    }
}

impl FunctionContext<'_> {
    pub fn get_type_by_name(&self, name: &str) -> Option<Type> {
        self.get_ident_by_name(name)
            .map(|it| self.get_type_of_ident(it))
    }

    fn get_ident_by_name(&self, name: &str) -> Option<FunctionIdentQueryResult> {
        if let Some((handle, _)) = self
            .function
            .local_variables
            .iter()
            .find(|(_, local)| local.name.as_ref().is_some_and(|it| it == name))
        {
            return Some(FunctionIdentQueryResult::LocalVariable(handle));
        }

        if let Some((handle, _)) = self
            .function
            .named_expressions
            .iter()
            .find(|(_, expr_name)| *expr_name == name)
        {
            return Some(FunctionIdentQueryResult::NamedExpression(*handle));
        }

        self.error_ctx
            .get_ident_by_name(name)
            .map(FunctionIdentQueryResult::Global)
    }

    pub fn get_type_of_ident(&self, query_result: FunctionIdentQueryResult) -> Type {
        match query_result {
            FunctionIdentQueryResult::LocalVariable(handle) => {
                self.module().types[self.function.local_variables[handle].ty].clone()
            }
            FunctionIdentQueryResult::NamedExpression(handle) => handle.as_type(self),
            FunctionIdentQueryResult::Global(global_ident_query_result) => {
                self.error_ctx.get_type_of_ident(&global_ident_query_result)
            }
        }
    }
}
