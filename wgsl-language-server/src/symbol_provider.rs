use std::ops::{Index, Range};

use lsp_types::{DocumentSymbol, SymbolKind};
use wgsl_ast::front::{
    ast::statement::declaration::{self, Declaration, Struct, StructMember},
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
                Declaration::Variable(v) => {
                    Some(v.into_document_symbol(&self.content, span.into()))
                }
                Declaration::Struct(s) => Some(s.into_document_symbol(&self.content, span.into())),
                Declaration::Function(f) => {
                    Some(f.into_document_symbol(&self.content, span.into()))
                }
                _ => None,
            })
            .collect()
    }
}

pub trait IntoDocumentSymbol {
    fn into_document_symbol(self, source: &str, span: Range<usize>) -> DocumentSymbol;
}

impl IntoDocumentSymbol for declaration::Function {
    #[allow(deprecated)]
    fn into_document_symbol(self, source: &str, span: Range<usize>) -> DocumentSymbol {
        DocumentSymbol {
            name: self.ident.inner,
            kind: SymbolKind::Function,
            detail: None,
            range: lsp_range_from_char_span(source, span),
            selection_range: lsp_range_from_char_span(source, self.ident.span.into()),
            children: None,
            tags: None,
            deprecated: None,
        }
    }
}

impl IntoDocumentSymbol for StructMember {
    #[allow(deprecated)]
    fn into_document_symbol(self, source: &str, span: Range<usize>) -> DocumentSymbol {
        DocumentSymbol {
            name: self.ident.inner,
            kind: SymbolKind::Field,
            detail: Some(source[self.value].to_owned()),
            range: lsp_range_from_char_span(source, span),
            children: None,
            deprecated: None,
            tags: None,
            selection_range: lsp_range_from_char_span(source, self.ident.span.into()),
        }
    }
}

impl IntoDocumentSymbol for Struct {
    #[allow(deprecated)]
    fn into_document_symbol(self, source: &str, span: Range<usize>) -> DocumentSymbol {
        let members = self.members.clone();

        let children = Some(
            members
                .into_iter()
                .map(|s| s.inner.into_document_symbol(source, s.span.into()))
                .collect(),
        );

        DocumentSymbol {
            name: self.ident.inner,
            kind: SymbolKind::Struct,
            detail: None,
            range: lsp_range_from_char_span(source, span),
            selection_range: lsp_range_from_char_span(source, self.ident.span.into()),
            children,
            deprecated: None,
            tags: None,
        }
    }
}

impl IntoDocumentSymbol for declaration::Variable {
    #[allow(deprecated)]
    fn into_document_symbol(self, source: &str, span: Range<usize>) -> DocumentSymbol {
        DocumentSymbol {
            name: self.ident.inner.0.clone(),
            kind: SymbolKind::Variable,
            detail: self.ident.1.as_ref().map(|ty| source[ty].to_owned()),
            range: lsp_range_from_char_span(source, span),
            selection_range: lsp_range_from_char_span(source, self.ident.span.into()),
            children: None,
            deprecated: None,
            tags: None,
        }
    }
}
