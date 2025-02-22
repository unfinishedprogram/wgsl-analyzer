#[derive(Debug, Clone, PartialEq, Default)]
pub enum IdentError {
    #[default]
    Unknown,
    SingleUnderscore,
    DoubleLeadingUnderscore,
    ReservedKeyword,
}

pub fn parse_ident(ident: &str) -> Result<&str, IdentError> {
    if ident.starts_with("__") {
        return Err(IdentError::DoubleLeadingUnderscore);
    }

    if ident == "_" {
        return Err(IdentError::SingleUnderscore);
    }

    if is_reserved_word(ident) {
        return Err(IdentError::ReservedKeyword);
    }

    Ok(ident)
}

fn is_reserved_word(ident: &str) -> bool {
    matches!(
        ident,
        "NULL"
            | "Self"
            | "abstract"
            | "active"
            | "alignas"
            | "alignof"
            | "as"
            | "asm"
            | "asm_fragment"
            | "async"
            | "attribute"
            | "auto"
            | "await"
            | "become"
            | "binding_array"
            | "cast"
            | "catch"
            | "class"
            | "co_await"
            | "co_return"
            | "co_yield"
            | "coherent"
            | "column_major"
            | "common"
            | "compile"
            | "compile_fragment"
            | "concept"
            | "const_cast"
            | "consteval"
            | "constexpr"
            | "constinit"
            | "crate"
            | "debugger"
            | "decltype"
            | "delete"
            | "demote"
            | "demote_to_helper"
            | "do"
            | "dynamic_cast"
            | "enum"
            | "explicit"
            | "export"
            | "extends"
            | "extern"
            | "external"
            | "fallthrough"
            | "filter"
            | "final"
            | "finally"
            | "friend"
            | "from"
            | "fxgroup"
            | "get"
            | "goto"
            | "groupshared"
            | "highp"
            | "impl"
            | "implements"
            | "import"
            | "inline"
            | "instanceof"
            | "interface"
            | "layout"
            | "lowp"
            | "macro"
            | "macro_rules"
            | "match"
            | "mediump"
            | "meta"
            | "mod"
            | "module"
            | "move"
            | "mut"
            | "mutable"
            | "namespace"
            | "new"
            | "nil"
            | "noexcept"
            | "noinline"
            | "nointerpolation"
            | "noperspective"
            | "null"
            | "nullptr"
            | "of"
            | "operator"
            | "package"
            | "packoffset"
            | "partition"
            | "pass"
            | "patch"
            | "pixelfragment"
            | "precise"
            | "precision"
            | "premerge"
            | "priv"
            | "protected"
            | "pub"
            | "public"
            | "readonly"
            | "ref"
            | "regardless"
            | "register"
            | "reinterpret_cast"
            | "require"
            | "resource"
            | "restrict"
            | "self"
            | "set"
            | "shared"
            | "sizeof"
            | "smooth"
            | "snorm"
            | "static"
            | "static_assert"
            | "static_cast"
            | "std"
            | "subroutine"
            | "super"
            | "target"
            | "template"
            | "this"
            | "thread_local"
            | "throw"
            | "trait"
            | "try"
            | "type"
            | "typedef"
            | "typeid"
            | "typename"
            | "typeof"
            | "union"
            | "unless"
            | "unorm"
            | "unsafe"
            | "unsized"
            | "use"
            | "using"
            | "varying"
            | "virtual"
            | "volatile"
            | "wgsl"
            | "where"
            | "with"
            | "writeonly"
            | "yield"
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
    Alias,
    Break,
    Case,
    Const,
    ConstAssert,
    Continue,
    Continuing,
    Default,
    Diagnostic,
    Discard,
    Else,
    Enable,
    Fn,
    For,
    If,
    Let,
    Loop,
    Override,
    Requires,
    Return,
    Struct,
    Switch,
    Var,
    While,
}
