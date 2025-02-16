use lsp_types::{CompletionItem, CompletionItemKind, Position};
use naga::{Function, Span};

use super::{
    completion_provider::{detailed_completion_item, new_completion_item},
    CompletionProvider,
};

use crate::{
    completions::swizzle::SWIZZLES,
    document_tracker::TrackedDocument,
    parser::matching_bracket_index,
    pretty_error::error_context::{type_print::TypePrintable, ModuleContext},
    range_tools::{source_location_to_range, span_to_lsp_range, string_offset, RangeTools},
};

impl TrackedDocument {
    fn get_containing_function(&self, position: &Position) -> Option<&Function> {
        let Some(module) = &self.last_valid_module else {
            return None;
        };

        for (handle, function) in module.functions.iter() {
            let mut location = module.functions.get_span(handle).location(&self.content);

            if let Some(close) = matching_bracket_index(&self.content, location.offset as usize) {
                location.length = close as u32 - location.offset;
            }

            let range = source_location_to_range(Some(location), &self.content).unwrap();

            if range.contains_line(position) {
                return Some(function);
            }
        }

        for entry_point in &module.entry_points {
            let entry_point_location =
                Span::total_span(entry_point.function.body.span_iter().map(|(_, span)| *span))
                    .location(&self.content);

            let range =
                source_location_to_range(Some(entry_point_location), &self.content).unwrap();

            if range.contains_line(position) {
                return Some(&entry_point.function);
            }
        }

        None
    }

    fn get_functions(&self, _position: &Position) -> Vec<CompletionItem> {
        let Some(module) = &self.last_valid_module else {
            return vec![];
        };

        let mut res = vec![];

        for (_, func) in module.functions.iter() {
            if let Some(name) = func.name.clone() {
                res.push(new_completion_item(name, CompletionItemKind::FUNCTION))
            }
        }

        res
    }

    fn get_locals(&self, position: &Position, function: &naga::Function) -> Vec<CompletionItem> {
        let mut res = vec![];

        for (handle, name) in function.named_expressions.iter() {
            let range = function
                .expressions
                .get_span(*handle)
                .to_range()
                .unwrap_or_default();

            if let Some(range_content) = self.content.get(range) {
                res.push(detailed_completion_item(
                    name.to_owned(),
                    CompletionItemKind::VARIABLE,
                    range_content,
                ))
            }
        }

        for (handle, variable) in function.local_variables.iter() {
            let range = span_to_lsp_range(function.local_variables.get_span(handle), &self.content);
            let span = function
                .local_variables
                .get_span(handle)
                .to_range()
                .unwrap_or_default();

            if range.start < *position {
                if let Some(name) = &variable.name {
                    res.push(detailed_completion_item(
                        name.to_owned(),
                        CompletionItemKind::VARIABLE,
                        &self.content[span],
                    ))
                }
            }
        }

        res
    }

    fn get_types(&self, _position: &Position) -> Vec<CompletionItem> {
        let Some(module) = &self.last_valid_module else {
            return vec![];
        };

        let mut res = vec![];

        for (_, ty) in module.types.iter() {
            if let Some(name) = &ty.name {
                let err_ctx = ModuleContext::new(module, &self.content);

                res.push(detailed_completion_item(
                    name.to_owned(),
                    CompletionItemKind::CLASS,
                    ty.print_type(&err_ctx),
                ))
            }
        }
        res
    }

    fn get_globals(&self, _position: &Position) -> Vec<CompletionItem> {
        let Some(module) = &self.last_valid_module else {
            return vec![];
        };

        let mut res = vec![];

        for (handle, constant) in module.constants.iter() {
            if let Some(name) = &constant.name {
                let span = module.constants.get_span(handle);
                let detail = &self.content[span.to_range().unwrap_or_default()];

                res.push(detailed_completion_item(
                    name.to_owned(),
                    CompletionItemKind::CONSTANT,
                    detail,
                ))
            }
        }

        for (handle, global) in module.global_variables.iter() {
            if let Some(name) = &global.name {
                let span = module.global_variables.get_span(handle);
                let detail = &self.content[span.to_range().unwrap_or_default()];

                res.push(detailed_completion_item(
                    name.to_owned(),
                    CompletionItemKind::VARIABLE,
                    detail,
                ))
            }
        }

        res
    }

    fn get_property_access(&self, position: &Position) -> Vec<CompletionItem> {
        let Some(ctx) = self.module_context() else {
            return vec![];
        };

        let Some(function) = self.get_containing_function(position) else {
            return vec![];
        };

        let Some(expr_type) = ctx.get_proceeding_popery_access_type(position, function) else {
            return vec![];
        };

        match expr_type {
            naga::TypeInner::Scalar(_) => {}
            naga::TypeInner::Image { .. } => {}
            naga::TypeInner::Sampler { .. } => {}
            naga::TypeInner::AccelerationStructure => {}
            naga::TypeInner::RayQuery => {}
            naga::TypeInner::BindingArray { .. } => {}
            naga::TypeInner::Atomic(_) => {}
            naga::TypeInner::Pointer { .. } => {}
            naga::TypeInner::Array { .. } => {}
            naga::TypeInner::ValuePointer { .. } => {}
            naga::TypeInner::Vector { size, .. } | naga::TypeInner::Matrix { rows: size, .. } => {
                return match size {
                    naga::VectorSize::Bi => &SWIZZLES[0],
                    naga::VectorSize::Tri => &SWIZZLES[1],
                    naga::VectorSize::Quad => &SWIZZLES[2],
                }
                .iter()
                .map(|s| new_completion_item(s, CompletionItemKind::FIELD))
                .collect();
            }
            naga::TypeInner::Struct { members, .. } => {
                return members
                    .iter()
                    .flat_map(|member| member.name.clone())
                    .map(|name| new_completion_item(name, CompletionItemKind::FIELD))
                    .collect()
            }
        }

        vec![]
    }
}

impl CompletionProvider for &TrackedDocument {
    fn get_completions(&self, position: &Position) -> Vec<CompletionItem> {
        let text_offset = string_offset(&self.content, position);
        let last_char = &self.content[text_offset - 1..text_offset];
        let is_access_completion = last_char.starts_with('.');

        let mut completions = vec![];

        if is_access_completion {
            completions.extend(self.get_property_access(position));
        } else {
            if let Some(current_function) = self.get_containing_function(position) {
                completions.extend(self.get_locals(position, current_function));
            }
            completions.extend(self.get_functions(position));
            completions.extend(self.get_types(position));
            completions.extend(self.get_globals(position));
            completions.extend(crate::completions::KeywordCompletions.get_completions(position));
            completions.extend(crate::completions::BuiltinCompletions.get_completions(position));
        }

        completions
    }
}
