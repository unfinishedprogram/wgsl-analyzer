use naga::{Expression, Handle, ScalarKind, Type, TypeInner};

use super::LabelContext;

pub trait AsType {
    fn as_type(&self, context: &LabelContext) -> Type;
}

impl AsType for Handle<Expression> {
    fn as_type(&self, context: &LabelContext) -> Type {
        fn scalar(kind: ScalarKind, width: u8) -> Type {
            Type {
                name: None,
                inner: TypeInner::Scalar { kind, width },
            }
        }

        fn from_inner(inner: TypeInner) -> Type {
            Type { name: None, inner }
        }

        let expression = &context[*self];
        match expression {
            Expression::Literal(lit) => match lit {
                naga::Literal::F64(_) => scalar(ScalarKind::Float, 8),
                naga::Literal::F32(_) => scalar(ScalarKind::Float, 4),
                naga::Literal::U32(_) => scalar(ScalarKind::Uint, 4),
                naga::Literal::I32(_) => scalar(ScalarKind::Sint, 4),
                naga::Literal::Bool(_) => scalar(ScalarKind::Bool, 0),
            },
            Expression::Constant(handle) => match context[*handle].inner.resolve_type() {
                naga::proc::TypeResolution::Handle(handle) => context[handle].clone(),
                naga::proc::TypeResolution::Value(inner) => from_inner(inner),
            },
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

            // TODO:
            // Expression::FunctionArgument(_) => todo!(),
            // Expression::GlobalVariable(_) => todo!(),
            // Expression::LocalVariable(_) => todo!(),
            // Expression::Access { base, index } => todo!(),
            // Expression::AccessIndex { base, index } => todo!(),
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
            _ => Type {
                name: Some("*UNKNOWN*".into()),
                inner: TypeInner::Scalar {
                    kind: ScalarKind::Uint,
                    width: 0,
                },
            },
        }
    }
}
