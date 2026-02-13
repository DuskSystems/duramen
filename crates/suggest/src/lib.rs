#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::vec::Vec;

#[must_use]
pub fn suggest<'a>(query: &str, candidates: &[&'a str]) -> Option<&'a str> {
    let len = query.chars().count();
    let mut threshold = len.max(3) / 3;

    let mut winner: Option<&str> = None;

    for candidate in candidates {
        let distance = damerau_levenshtein_distance(query, candidate);
        if distance <= threshold {
            threshold = distance.saturating_sub(1);
            winner = Some(candidate);
        }
    }

    winner
}

/// <https://en.wikipedia.org/wiki/Damerau-Levenshtein_distance>.
fn damerau_levenshtein_distance(source: &str, target: &str) -> usize {
    let source: Vec<char> = source.chars().collect();
    let target: Vec<char> = target.chars().collect();

    let prefix = source
        .iter()
        .zip(&target)
        .take_while(|(source_char, target_char)| source_char == target_char)
        .count();

    let suffix = source[prefix..]
        .iter()
        .rev()
        .zip(target[prefix..].iter().rev())
        .take_while(|(source_char, target_char)| source_char == target_char)
        .count();

    let source = &source[prefix..source.len() - suffix];
    let target = &target[prefix..target.len() - suffix];

    if source.is_empty() {
        return target.len();
    }

    if target.is_empty() {
        return source.len();
    }

    let source_len = source.len();
    let target_len = target.len();

    let mut window: [Vec<usize>; 3] = [
        Vec::with_capacity(target_len + 1),
        (0..=target_len).collect(),
        Vec::with_capacity(target_len + 1),
    ];

    for row in 1..=source_len {
        window[2].clear();
        window[2].push(row);

        for column in 1..=target_len {
            let cost = usize::from(source[row - 1] != target[column - 1]);

            let deletion = window[1][column] + 1;
            let insertion = window[2][column - 1] + 1;
            let substitution = window[1][column - 1] + cost;

            let mut distance = deletion.min(insertion).min(substitution);

            if row > 1
                && column > 1
                && source[row - 1] == target[column - 2]
                && source[row - 2] == target[column - 1]
            {
                let transposition = window[0][column - 2] + cost;
                distance = distance.min(transposition);
            }

            window[2].push(distance);
        }

        window.rotate_left(1);
    }

    window[1][target_len]
}

// Tests sourced from: https://github.com/rapidfuzz/strsim-rs/blob/v0.11.1/src/lib.rs
#[cfg(test)]
mod tests {
    use super::damerau_levenshtein_distance;

    #[test]
    fn empty() {
        assert_eq!(damerau_levenshtein_distance("", ""), 0);
    }

    #[test]
    fn same() {
        assert_eq!(damerau_levenshtein_distance("damerau", "damerau"), 0);
    }

    #[test]
    fn first_empty() {
        assert_eq!(damerau_levenshtein_distance("", "damerau"), 7);
    }

    #[test]
    fn second_empty() {
        assert_eq!(damerau_levenshtein_distance("damerau", ""), 7);
    }

    #[test]
    fn diff() {
        assert_eq!(damerau_levenshtein_distance("ca", "abc"), 3);
    }

    #[test]
    fn diff_short() {
        assert_eq!(damerau_levenshtein_distance("damerau", "aderua"), 3);
    }

    #[test]
    fn diff_reversed() {
        assert_eq!(damerau_levenshtein_distance("aderua", "damerau"), 3);
    }

    #[test]
    fn diff_multibyte() {
        assert_eq!(damerau_levenshtein_distance("öঙ香", "abc"), 3);
        assert_eq!(damerau_levenshtein_distance("abc", "öঙ香"), 3);
    }

    #[test]
    fn diff_unequal_length() {
        assert_eq!(damerau_levenshtein_distance("damerau", "aderuaxyz"), 6);
    }

    #[test]
    fn diff_unequal_length_reversed() {
        assert_eq!(damerau_levenshtein_distance("aderuaxyz", "damerau"), 6);
    }

    #[test]
    fn diff_comedians() {
        assert_eq!(damerau_levenshtein_distance("Stewart", "Colbert"), 5);
    }

    #[test]
    fn many_transpositions() {
        assert_eq!(
            damerau_levenshtein_distance("abcdefghijkl", "bacedfgihjlk"),
            4
        );
    }

    #[test]
    fn diff_longer() {
        assert_eq!(
            damerau_levenshtein_distance(
                "The quick brown fox jumped over the angry dog.",
                "Lehem ipsum dolor sit amet, dicta latine an eam.",
            ),
            36,
        );
    }

    #[test]
    fn beginning_transposition() {
        assert_eq!(damerau_levenshtein_distance("foobar", "ofobar"), 1);
    }

    #[test]
    fn end_transposition() {
        assert_eq!(damerau_levenshtein_distance("specter", "spectre"), 1);
    }

    #[test]
    fn restricted_edit() {
        assert_eq!(damerau_levenshtein_distance("a cat", "an abct"), 4);
    }
}
