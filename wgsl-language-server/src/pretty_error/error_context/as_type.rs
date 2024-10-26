use super::FunctionErrorContext;
use naga::{Expression, Handle, Scalar, ScalarKind, Type, TypeInner};

pub trait AsType {
    fn as_type(&self, context: &FunctionErrorContext) -> Type;
}

impl AsType for Handle<Expression> {
    fn as_type(&self, context: &FunctionErrorContext) -> Type {
        fn scalar(kind: ScalarKind, width: u8) -> Type {
            Type {
                name: None,
                inner: TypeInner::Scalar(Scalar { kind, width }),
            }
        }

        let expression = &context[*self];
        match expression {
            Expression::Literal(lit) => match lit {
                naga::Literal::F64(_) => scalar(ScalarKind::Float, 8),
                naga::Literal::F32(_) => scalar(ScalarKind::Float, 4),
                naga::Literal::U32(_) => scalar(ScalarKind::Uint, 4),
                naga::Literal::I32(_) => scalar(ScalarKind::Sint, 4),
                naga::Literal::Bool(_) => scalar(ScalarKind::Bool, 0),
                naga::Literal::U64(_) => scalar(ScalarKind::Uint, 8),
                naga::Literal::I64(_) => scalar(ScalarKind::Sint, 8),
                naga::Literal::AbstractInt(_) => scalar(ScalarKind::AbstractInt, 8),
                naga::Literal::AbstractFloat(_) => scalar(ScalarKind::AbstractFloat, 8),
            },
            Expression::Constant(handle) => context[context[*handle].ty].clone(),
            Expression::ZeroValue(handle) => context[*handle].clone(),
            Expression::Swizzle {
                size: _,
                vector,
                pattern: _,
            } => vector.as_type(context),
            Expression::Compose { ty, components: _ } => context[*ty].clone(),
            Expression::Load { pointer } => pointer.as_type(context),
            Expression::Unary { op: _, expr } => expr.as_type(context),
            Expression::Binary { op, left, right: _ } => match op {
                naga::BinaryOperator::Add
                | naga::BinaryOperator::Subtract
                | naga::BinaryOperator::Multiply
                | naga::BinaryOperator::Divide
                | naga::BinaryOperator::Modulo
                | naga::BinaryOperator::ExclusiveOr
                | naga::BinaryOperator::InclusiveOr
                | naga::BinaryOperator::LogicalAnd
                | naga::BinaryOperator::LogicalOr
                | naga::BinaryOperator::ShiftLeft
                | naga::BinaryOperator::ShiftRight => left.as_type(context),

                naga::BinaryOperator::Equal
                | naga::BinaryOperator::NotEqual
                | naga::BinaryOperator::Less
                | naga::BinaryOperator::LessEqual
                | naga::BinaryOperator::Greater
                | naga::BinaryOperator::GreaterEqual
                | naga::BinaryOperator::And => scalar(ScalarKind::Bool, 0),
            },
            Expression::Select {
                condition: _,
                accept,
                reject: _,
            } => accept.as_type(context),
            Expression::Derivative {
                axis: _,
                ctrl: _,
                expr,
            } => expr.as_type(context),
            Expression::Relational {
                fun: _,
                argument: _,
            } => scalar(ScalarKind::Bool, 0),

            Expression::AtomicResult { ty, comparison: _ } => context[*ty].clone(),
            Expression::WorkGroupUniformLoadResult { ty } => context[*ty].clone(),
            Expression::ArrayLength(_) => scalar(ScalarKind::Uint, 4),
            Expression::FunctionArgument(index) => {
                let ty_handle =
                    context.module.functions[context.function].arguments[*index as usize].ty;
                context.module.types[ty_handle].clone()
            }
            Expression::GlobalVariable(handle) => {
                context.module.types[context.module.global_variables[*handle].ty].clone()
            }
            Expression::AccessIndex { base, index } => {
                let base_ty = base.as_type(context);
                match base_ty.inner {
                    TypeInner::Struct { members, .. } => {
                        context.module.types[members[*index as usize].ty].clone()
                    }
                    TypeInner::Array { base, .. } => context.module.types[base].clone(),
                    _ => base_ty,
                }
            }

            // TODO:
            // Expression::LocalVariable(_) => todo!(),
            // Expression::Access { base, index } => todo!(),
            // Expression::Splat { size, value } => todo!(),
            // Expression::ImageSample {
            //     image,
            //     sampler,
            //     gather,
            //     coordinate,
            //     array_index,
            //     offset,
            //     level,
            //     depth_ref,
            // } => todo!(),
            // Expression::ImageLoad {
            //     image,
            //     coordinate,
            //     array_index,
            //     sample,
            //     level,
            // } => todo!(),
            // Expression::CallResult(_) => todo!(),
            // Expression::Math {
            //     fun,
            //     arg,
            //     arg1,
            //     arg2,
            //     arg3,
            // } => todo!(),
            // Expression::As {
            //     expr,
            //     kind,
            //     convert,
            // } => todo!(),
            // Expression::ImageQuery { image, query } => todo!(),
            other => Type {
                name: Some(format!("UNHANDLED : {other:?}")),
                inner: TypeInner::Scalar(Scalar {
                    kind: ScalarKind::Uint,
                    width: 0,
                }),
            },
        }
    }
}
