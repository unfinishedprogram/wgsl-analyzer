use lsp_types::{DocumentSymbol, SymbolKind};
use wgsl_ast::front::{
    ast::statement::declaration::{Declaration, StructMember},
    span::Spanned,
};

use crate::{document_tracker::TrackedDocument, range_tools::lsp_range_from_char_span};

pub trait SymbolProvider {
    fn get_symbols(&self) -> Vec<DocumentSymbol>;
    // fn document_symbol(&self, item: SymbolItem) -> DocumentSymbol;
}

impl SymbolProvider for TrackedDocument {
    #[allow(deprecated)]
    fn get_symbols(&self) -> Vec<DocumentSymbol> {
        let Some(Ok(module)) = &self.compilation_result else {
            return vec![];
        };

        module
            .declarations()
            .into_iter()
            .filter_map(|Spanned { span, inner }| match inner {
                Declaration::Variable { ident, .. } => Some(DocumentSymbol {
                    name: ident.inner.0,
                    kind: SymbolKind::Variable,
                    detail: None,
                    range: lsp_range_from_char_span(&self.content, span.into()),
                    selection_range: lsp_range_from_char_span(&self.content, ident.span.into()),
                    children: None,
                    deprecated: None,
                    tags: None,
                }),
                Declaration::Struct { ident, members, .. } => {
                    let children = Some(
                        members
                            .iter()
                            .map(
                                |StructMember {
                                     attributes,
                                     ident,
                                     value,
                                 }| DocumentSymbol {
                                    name: ident.inner.clone(),
                                    kind: SymbolKind::Field,
                                    detail: Some(format!("{value:?}")),
                                    range: lsp_range_from_char_span(
                                        &self.content,
                                        ident.span.into(),
                                    ),
                                    children: None,
                                    deprecated: None,
                                    tags: None,
                                    selection_range: lsp_range_from_char_span(
                                        &self.content,
                                        ident.span.into(),
                                    ),
                                },
                            )
                            .collect(),
                    );

                    Some(DocumentSymbol {
                        name: ident.inner,
                        kind: SymbolKind::Struct,
                        detail: None,
                        range: lsp_range_from_char_span(&self.content, span.into()),
                        selection_range: lsp_range_from_char_span(&self.content, ident.span.into()),
                        children,
                        deprecated: None,
                        tags: None,
                    })
                }
                Declaration::Function { ident, .. } => Some(DocumentSymbol {
                    name: ident.inner,
                    kind: SymbolKind::Function,
                    detail: None,
                    range: lsp_range_from_char_span(&self.content, span.into()),
                    selection_range: lsp_range_from_char_span(&self.content, ident.span.into()),
                    children: None,
                    deprecated: None,
                    tags: None,
                }),
                _ => None,
            })
            .collect()
    }
}
