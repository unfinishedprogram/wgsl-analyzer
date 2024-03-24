use crate::range_tools::lsp_range_from_char_span;

pub struct FindDefinitionResult {
    pub selection_range: std::ops::Range<usize>,
    pub definition_location: DefinitionLocation,
}

pub struct DefinitionLocation {
    pub range: std::ops::Range<usize>,
    pub ident_range: std::ops::Range<usize>,
}

impl FindDefinitionResult {
    pub fn new(
        selection_range: std::ops::Range<usize>,
        definition_location: DefinitionLocation,
    ) -> FindDefinitionResult {
        FindDefinitionResult {
            selection_range,
            definition_location,
        }
    }

    pub fn into_lsp_location_link(
        self,
        source: &str,
        uri: &lsp_types::Url,
    ) -> lsp_types::LocationLink {
        let target_range = lsp_range_from_char_span(source, &self.definition_location.range);
        let target_selection_range =
            lsp_range_from_char_span(source, &self.definition_location.ident_range);
        let selection_range = lsp_range_from_char_span(source, &self.selection_range);

        lsp_types::LocationLink {
            target_uri: uri.clone(),
            target_range,
            target_selection_range,
            origin_selection_range: Some(selection_range),
        }
    }
}

impl DefinitionLocation {
    pub fn new(
        ident_range: std::ops::Range<usize>,
        range: std::ops::Range<usize>,
    ) -> DefinitionLocation {
        DefinitionLocation { range, ident_range }
    }

    pub fn from_range(range: std::ops::Range<usize>) -> DefinitionLocation {
        DefinitionLocation {
            range: range.clone(),
            ident_range: range,
        }
    }
}
