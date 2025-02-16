use std::sync::LazyLock;

const RGBA: &[char; 4] = &['r', 'g', 'b', 'a'];
const XYZW: &[char; 4] = &['x', 'y', 'z', 'w'];

pub static SWIZZLES: LazyLock<[Vec<String>; 3]> = LazyLock::new(|| {
    let mut length_2 = swizzles_of_length(4, &XYZW[0..2]);
    length_2.extend(swizzles_of_length(4, &RGBA[0..2]));

    let mut length_3 = swizzles_of_length(4, &XYZW[0..3]);
    length_3.extend(swizzles_of_length(4, &RGBA[0..3]));

    let mut length_4 = swizzles_of_length(4, XYZW);
    length_4.extend(swizzles_of_length(4, RGBA));

    [length_2, length_3, length_4]
});

fn swizzles_of_length(length: usize, parts: &[char]) -> Vec<String> {
    swizzles_of_length_inner(
        length,
        parts,
        parts.iter().map(|it| it.to_string()).collect(),
    )
}

fn swizzles_of_length_inner(
    length: usize,
    parts: &[char],
    mut previous: Vec<String>,
) -> Vec<String> {
    if length == 1 {
        return previous;
    }

    let len = previous.len();
    for i in 0..len {
        for p in parts.iter() {
            previous.push(format!("{}{p}", previous[i]));
        }
    }

    swizzles_of_length_inner(length - 1, parts, previous)
}
