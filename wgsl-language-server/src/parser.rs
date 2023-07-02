pub fn matching_bracket_index(source: &str, open: usize) -> Option<usize> {
    let chars = source[open..].char_indices();
    let mut depth = 0;
    for (index, c) in chars {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index + open);
                }
            }
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn bracket_matcher_trivial() {
        assert_eq!(matching_bracket_index("{}", 0), Some(1));
    }

    #[test]
    pub fn bracket_matcher_nested() {
        assert_eq!(matching_bracket_index("{{}}", 0), Some(3));
    }

    #[test]
    pub fn bracket_matcher_nested_inside() {
        assert_eq!(matching_bracket_index("{{}}", 1), Some(2));
    }

    #[test]
    pub fn practical() {
        let src = "pub fn function_name(param:Type) { if (condition) { expr } }";
        assert_eq!(matching_bracket_index(src, 0), Some(src.len() - 1));
    }
}
