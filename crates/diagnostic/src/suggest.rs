/// Maximum string length supported.
const MAX_LEN: usize = 64;

/// Returns the closest match from `candidates` to `input`.
#[must_use]
pub fn suggest<'a>(input: &str, candidates: &[&'a str]) -> Option<&'a str> {
    if input.is_empty() {
        return None;
    }

    let threshold = (input.len() / 3).clamp(1, 3);

    let mut best = None;
    let mut best_distance = threshold + 1;

    for &candidate in candidates {
        let distance = damerau_levenshtein(input, candidate);
        if distance < best_distance {
            best_distance = distance;
            best = Some(candidate);

            if distance == 0 {
                break;
            }
        }
    }

    best
}

fn damerau_levenshtein(source: &str, target: &str) -> usize {
    let source = source.as_bytes();
    let target = target.as_bytes();

    if source.len() > MAX_LEN || target.len() > MAX_LEN {
        return usize::MAX;
    }

    let prefix = source
        .iter()
        .zip(target)
        .take_while(|(source_byte, target_byte)| source_byte == target_byte)
        .count();

    let suffix = source[prefix..]
        .iter()
        .rev()
        .zip(target[prefix..].iter().rev())
        .take_while(|(source_byte, target_byte)| source_byte == target_byte)
        .count();

    let source = &source[prefix..source.len() - suffix];
    let target = &target[prefix..target.len() - suffix];

    if source.is_empty() {
        return target.len();
    }

    if target.is_empty() {
        return source.len();
    }

    let mut window = [[0_usize; MAX_LEN + 1]; 3];

    let columns = target.len() + 1;
    for (column, slot) in window[0][..columns].iter_mut().enumerate() {
        *slot = column;
    }

    for (row, &source_byte) in source.iter().enumerate() {
        let current = (row + 1) % 3;
        let previous = row % 3;

        window[current][0] = row + 1;

        for (column, &target_byte) in target.iter().enumerate() {
            let cost = usize::from(source_byte != target_byte);

            let deletion = window[previous][column + 1] + 1;
            let insertion = window[current][column] + 1;
            let substitution = window[previous][column] + cost;
            window[current][column + 1] = deletion.min(insertion).min(substitution);

            if row > 0
                && column > 0
                && source_byte == target[column - 1]
                && source[row - 1] == target_byte
            {
                let transposition = window[(row + 2) % 3][column - 1] + 1;
                window[current][column + 1] = window[current][column + 1].min(transposition);
            }
        }
    }

    window[source.len() % 3][target.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact() {
        assert_eq!(
            suggest("contains", &["contains", "containsAll"]),
            Some("contains")
        );
    }

    #[test]
    fn close() {
        assert_eq!(
            suggest("contians", &["contains", "containsAll"]),
            Some("contains")
        );
    }

    #[test]
    fn transposition() {
        assert_eq!(damerau_levenshtein("ab", "ba"), 1);
    }

    #[test]
    fn none() {
        assert_eq!(suggest("foobar", &["contains", "containsAll"]), None);
    }

    #[test]
    fn empty() {
        assert_eq!(suggest("foobar", &[""]), None);
        assert_eq!(suggest("", &["contains"]), None);
        assert_eq!(suggest("", &[""]), None);
    }
}
