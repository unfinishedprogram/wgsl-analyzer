use std::sync::LazyLock;
use lsp_types::Position;
use naga::TypeInner;
use regex::Regex;

use crate::pretty_error::error_context::{FunctionContext, ModuleContext};

static RE_ACCESS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(^\w+|\[[^\]]+\]|\.\w+)").unwrap());

#[derive(Debug)]
pub enum PropertyAccess<'a> {
    Field(&'a str),
    Index,
}

fn parse_property_accesses(line:&str) -> Vec<PropertyAccess> {
    let line = line.trim();
    let mut props = vec![];

    for cap in RE_ACCESS.captures_iter(line) {
        if let Some(prop) = cap.get(1) {
            let prop = prop.as_str();
            if prop.starts_with('[') {
                props.push(PropertyAccess::Index);
            } else {
                let prop = prop.strip_prefix('.').unwrap_or(prop);
                props.push(PropertyAccess::Field(prop));
            }
        }
    }

    props
}

impl FunctionContext<'_> {
    fn evaluate_property_access_type(&self, props: &[PropertyAccess]) -> Option<TypeInner> {
        let mut current_type = None;

        for prop in props {
            match prop {
                PropertyAccess::Field(name) => {
                    if let Some(base) = &current_type {
                        if let TypeInner::Struct { members, .. } = base {
                            for member in members {
                                if member.name.as_ref().is_some_and(|it| it == name) {
                                    current_type = Some(self.module().types[member.ty].inner.clone());
                                    break;
                                }
                            }
                        }
                    } else {
                        current_type = self.get_type_by_name(name).map(|ty|ty.inner);
                    }
                }
                PropertyAccess::Index => {
                    match current_type {
                        Some(TypeInner::Array { base, .. }|TypeInner::BindingArray { base, .. }) => {
                            current_type = Some(self.module().types[base].inner.clone());
                        }
                        _ => break,
                    }
                }
            }
        }
        

        current_type
    }
}


impl ModuleContext<'_> {
    pub fn get_proceeding_popery_access_type(
        &self,
        position: &Position,
        function: &naga::Function,
    ) -> Option<TypeInner> {
        // TODO: Extract only the part of the line before the cursor
        let line = self.code
            .split('\n')
            .nth(position.line as usize)
            .unwrap();

        let accesses = parse_property_accesses(line);

        self.function_ctx(function).evaluate_property_access_type(&accesses)
    }

}
