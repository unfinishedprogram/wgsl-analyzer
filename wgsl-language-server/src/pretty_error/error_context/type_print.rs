use naga::{FunctionResult, Handle, Scalar, ScalarKind, Type, TypeInner};

use super::ModuleContext;

pub trait TypePrintable {
    fn print_type(&self, context: &ModuleContext) -> String;
}

impl TypePrintable for FunctionResult {
    fn print_type(&self, context: &ModuleContext) -> String {
        self.ty.print_type(context)
    }
}

impl TypePrintable for Handle<Type> {
    fn print_type(&self, context: &ModuleContext) -> String {
        context.module.types[*self].print_type(context)
    }
}

impl TypePrintable for Type {
    fn print_type(&self, context: &ModuleContext) -> String {
        if let Some(name) = &self.name {
            format!("{name} {}", self.inner.print_type(context))
        } else {
            self.inner.print_type(context)
        }
    }
}

impl TypePrintable for TypeInner {
    fn print_type(&self, context: &ModuleContext) -> String {
        fn print_scalar(kind: &ScalarKind, width: u8) -> String {
            match kind {
                ScalarKind::Sint => format!("i{}", width * 8),
                ScalarKind::Uint => format!("u{}", width * 8),
                ScalarKind::Float => format!("f{}", width * 8),
                ScalarKind::Bool => "bool".into(),
                ScalarKind::AbstractFloat => "AbstractFloat".into(),
                ScalarKind::AbstractInt => "AbstractInt".into(),
            }
        }
        match self {
            TypeInner::Scalar(Scalar { kind, width }) => print_scalar(kind, *width),
            TypeInner::Vector {
                size,
                scalar: Scalar { kind, width },
            } => {
                format!("vec{}<{}>", *size as u8, print_scalar(kind, *width))
            }
            TypeInner::Matrix {
                columns,
                rows,
                scalar: Scalar { kind: _, width },
            } => {
                format!("mat{}x{}<{}>", *columns as u8, *rows as u8, width * 8)
            }
            TypeInner::Atomic(Scalar { kind, width }) => {
                format!("Atomic<{}>", print_scalar(kind, *width))
            }
            TypeInner::Pointer { base, space } => {
                format!("ptr<{space:?}, {}>", base.print_type(context))
            }
            TypeInner::ValuePointer {
                size: _,
                space,
                scalar: Scalar { kind, width },
            } => format!("ptr<{space:?}, {}>", print_scalar(kind, *width)),
            TypeInner::Array {
                base,
                size,
                stride: _,
            } => {
                let base_type = base.print_type(context);
                match size {
                    naga::ArraySize::Constant(s) => format!("Array<{base_type}, {s}>"),
                    naga::ArraySize::Dynamic => format!("Array<{base_type}>"),
                    naga::ArraySize::Pending(s) => format!("Array<{base_type}, {s:?}>"),
                }
            }

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
            TypeInner::BindingArray { base, size } => {
                let base_type = base.print_type(context);

                match size {
                    naga::ArraySize::Constant(s) => format!("BindingArray<{base_type}, {s}>"),
                    naga::ArraySize::Dynamic => format!("BindingArray<{base_type}>"),
                    naga::ArraySize::Pending(s) => format!("BindingArray<{base_type}, {s:?}>"),
                }
            }
            TypeInner::Struct { members, span: _ } => {
                let res: String = members
                    .iter()
                    .map(|member| match &member.name {
                        Some(name) => format!("  {name}: {}", member.ty.print_type(context)),
                        None => member.ty.print_type(context),
                    })
                    .collect::<Vec<String>>()
                    .join(",\n");
                ["{", &res, "}"].join("\n")
            }
        }
    }
}
