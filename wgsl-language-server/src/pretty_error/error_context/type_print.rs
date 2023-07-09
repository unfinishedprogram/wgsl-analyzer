use naga::{FunctionResult, Handle, ScalarKind, Type, TypeInner};

use super::LabelContext;

pub trait TypePrintable {
    fn print_type(&self, context: &LabelContext) -> String;
}

impl TypePrintable for FunctionResult {
    fn print_type(&self, context: &LabelContext) -> String {
        self.ty.print_type(context)
    }
}

impl TypePrintable for Handle<Type> {
    fn print_type(&self, context: &LabelContext) -> String {
        context.error_context.module.types[*self].print_type(context)
    }
}

impl TypePrintable for Type {
    fn print_type(&self, context: &LabelContext) -> String {
        if let Some(name) = &self.name {
            format!("{name}:{}", self.inner.print_type(context))
        } else {
            self.inner.print_type(context)
        }
    }
}

impl TypePrintable for TypeInner {
    fn print_type(&self, context: &LabelContext) -> String {
        fn print_scalar(kind: &ScalarKind, width: u8) -> String {
            match kind {
                ScalarKind::Sint => format!("i{}", width * 8),
                ScalarKind::Uint => format!("u{}", width * 8),
                ScalarKind::Float => format!("f{}", width * 8),
                ScalarKind::Bool => "bool".into(),
            }
        }
        match self {
            TypeInner::Scalar { kind, width } => print_scalar(kind, *width),
            TypeInner::Vector { size, kind, width } => {
                format!("vec{}<{}>", *size as u8, print_scalar(kind, *width))
            }
            TypeInner::Matrix {
                columns,
                rows,
                width,
            } => {
                format!("mat{}x{}<{}>", *columns as u8, *rows as u8, width * 8)
            }
            TypeInner::Atomic { kind, width } => format!("Atomic<{}>", print_scalar(kind, *width)),
            TypeInner::Pointer { base, space } => {
                format!("ptr<{space:?}, {}>", base.print_type(context))
            }
            TypeInner::ValuePointer {
                size,
                kind,
                width,
                space,
            } => format!("ptr<{space:?}, {}>", print_scalar(kind, *width)),
            TypeInner::Array { base, size, stride } => match size {
                naga::ArraySize::Constant(s) => format!("Array<{}, {s}>", base.print_type(context)),
                naga::ArraySize::Dynamic => format!("Array<{}>", base.print_type(context)),
            },

            // TODO: Improve type printing
            TypeInner::Image {
                dim,
                arrayed,
                class,
            } => format!("{dim:?}:{arrayed:?}:{class:?}"),
            TypeInner::Sampler { comparison } => if *comparison {
                "sampler_comparison"
            } else {
                "sampler"
            }
            .into(),
            TypeInner::AccelerationStructure => "AccelerationStructure".into(),
            TypeInner::RayQuery => "RayQuery".into(),
            TypeInner::BindingArray { base, size } => match size {
                naga::ArraySize::Constant(s) => {
                    format!("BindingArray<{}, {s}>", base.print_type(context))
                }
                naga::ArraySize::Dynamic => format!("BindingArray<{}>", base.print_type(context)),
            },
            TypeInner::Struct { members, span } => {
                let res: String = members
                    .iter()
                    .map(|member| match &member.name {
                        Some(name) => format!("\t{name}: {},\n", member.ty.print_type(context)),
                        None => member.ty.print_type(context),
                    })
                    .collect();
                ["{", &res, "}"].join("\n")
            }
        }
    }
}
