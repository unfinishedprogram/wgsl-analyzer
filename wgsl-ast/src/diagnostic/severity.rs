// Corresponds 1:1 to the diagnostic severity in the LSP spec
// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnostic

#[derive(Clone, Copy)]
pub enum Severity {
    Hint = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
}

impl From<Severity> for ariadne::Color {
    fn from(val: Severity) -> Self {
        match val {
            Severity::Error => ariadne::Color::Red,
            Severity::Warning => ariadne::Color::Yellow,
            Severity::Info => ariadne::Color::Blue,
            Severity::Hint => ariadne::Color::Green,
        }
    }
}
