use lsp_types::{CompletionItem, CompletionItemKind, Position};

use crate::{
    document_tracker::TrackedDocument,
    parser::matching_bracket_index,
    range_tools::{source_location_to_range, span_to_range, RangeTools},
};

pub trait CompletionProvider {
    fn get_completion(&self, position: &Position) -> Vec<CompletionItem>;

    fn get_functions(&self, position: &Position) -> Vec<CompletionItem>;
    fn get_locals(
        &self,
        position: &Position,
        function: naga::Handle<naga::Function>,
    ) -> Vec<CompletionItem>;
    fn get_types(&self, position: &Position) -> Vec<CompletionItem>;
    fn get_constants(&self, position: &Position) -> Vec<CompletionItem>;
}

impl CompletionProvider for TrackedDocument {
    fn get_completion(&self, position: &Position) -> Vec<CompletionItem> {
        let mut res = vec![];
        res.extend(self.get_functions(position));
        res.extend(self.get_types(position));
        res.extend(self.get_constants(position));
        res
    }
    fn get_functions(&self, position: &Position) -> Vec<CompletionItem> {
        let Some(module) = &self.module else {
            return vec![];
        };

        let mut res = vec![];

        for (handle, func) in module.functions.iter() {
            if let Some(name) = func.name.clone() {
                let mut location = module.functions.get_span(handle).location(&self.content);

                if let Some(close) = matching_bracket_index(&self.content, location.offset as usize)
                {
                    location.length = close as u32 - location.offset;
                }

                let range = source_location_to_range(Some(location), &self.content).unwrap();

                if range.contains_line(position) {
                    res.extend(self.get_locals(position, handle));
                }

                res.push(new_completion_item(name, CompletionItemKind::Function))
            }
        }

        res
    }
    fn get_locals(
        &self,
        position: &Position,
        function: naga::Handle<naga::Function>,
    ) -> Vec<CompletionItem> {
        let Some(module) = &self.module else {
            return vec![];
        };
        let function = &module.functions[function];
        let mut res = vec![];

        for (handle, name) in function.named_expressions.iter() {
            let range = span_to_range(function.expressions.get_span(*handle), &self.content);
            res.push(detailed_completion_item(
                name.to_owned(),
                CompletionItemKind::Variable,
                &format!("{:?}", range),
            ))
        }

        for (handle, variable) in function.local_variables.iter() {
            let range = span_to_range(function.local_variables.get_span(handle), &self.content);

            if range.start < *position {
                if let Some(name) = &variable.name {
                    res.push(new_completion_item(
                        name.to_owned(),
                        CompletionItemKind::Variable,
                    ))
                }
            } else if let Some(name) = &variable.name {
                res.push(new_completion_item(
                    name.to_owned(),
                    CompletionItemKind::Variable,
                ))
            }
        }

        res
    }
    fn get_types(&self, position: &Position) -> Vec<CompletionItem> {
        let Some(module) = &self.module else {
            return vec![];
        };

        let mut res = vec![];

        for (_, ty) in module.types.iter() {
            if let Some(name) = &ty.name {
                res.push(detailed_completion_item(
                    name.to_owned(),
                    CompletionItemKind::Class,
                    &format!("{:?}", ty.inner),
                ))
            }
        }
        res
    }
    fn get_constants(&self, position: &Position) -> Vec<CompletionItem> {
        let Some(module) = &self.module else {
            return vec![];
        };

        let mut res = vec![];

        for (_, constant) in module.constants.iter() {
            if let Some(name) = &constant.name {
                res.push(new_completion_item(
                    name.to_owned(),
                    CompletionItemKind::Constant,
                ))
            }
        }

        res
    }
}

pub fn new_completion_item(symbol: String, kind: CompletionItemKind) -> CompletionItem {
    CompletionItem {
        label: symbol,
        kind: Some(kind),
        ..Default::default()
    }
}

pub fn detailed_completion_item(
    symbol: String,
    kind: CompletionItemKind,
    detail: &str,
) -> CompletionItem {
    CompletionItem {
        label: symbol,
        kind: Some(kind),
        detail: Some(detail.to_owned()),
        ..Default::default()
    }
}
