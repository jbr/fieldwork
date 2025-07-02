use std::fmt::{self, Display, Formatter};

use proc_macro2::Span;
use syn::Error;

fn damerau_levenshtein(a: &str, b: &str) -> usize {
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();
    let len_a = a.len();
    let len_b = b.len();
    let mut dist = vec![vec![0; len_b + 1]; len_a + 1];

    for (i, dist_i) in dist.iter_mut().enumerate().take(len_a + 1) {
        dist_i[0] = i;
    }
    for j in 0..=len_b {
        dist[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            dist[i][j] = *[
                dist[i - 1][j] + 1,        // deletion
                dist[i][j - 1] + 1,        // insertion
                dist[i - 1][j - 1] + cost, // substitution
            ]
            .iter()
            .min()
            .unwrap();

            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                dist[i][j] = dist[i][j].min(dist[i - 2][j - 2] + cost); // transposition
            }
        }
    }

    dist[len_a][len_b]
}

pub(crate) fn invalid_key(span: Span, key: &str, valid_keys: &[&str]) -> Error {
    if valid_keys.contains(&key) {
        return Error::new(span, format!("invalid value for key `{key}`"));
    }

    let suggestion = valid_keys
        .iter()
        .map(|s| (*s, damerau_levenshtein(s, key)))
        .min_by_key(|(_, distance)| *distance);

    let keys = Keys(valid_keys);

    match suggestion {
        Some((suggestion, distance)) if distance <= 2 => Error::new(
            span,
            format!(
                "unknown configuration `{key}`; did you mean `{suggestion}`?\n\n\
                 In this position, fieldwork recognizes:\n{keys}",
            ),
        ),

        _ => Error::new(
            span,
            format!(
                "unknown configuration `{key}`\n\n\
                 In this position, fieldwork recognizes:\n{keys}",
            ),
        ),
    }
}

struct Keys<'a>(&'a [&'a str]);

impl Display for Keys<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_width = 93; // rustc indents by 7, so this wraps us at 100
        let len = self.0.len();
        let mut line_len = 0;
        for (n, key) in self.0.iter().enumerate() {
            let is_last = n == len - 1;
            let is_first = n == 0;
            let chunk_len = if is_first {
                2 + key.len() // `key`
            } else if is_last {
                8 + key.len() // ", and `key`"
            } else {
                4 + key.len() // ", `key`"
            };

            if line_len + chunk_len > max_width && line_len > 0 {
                f.write_str(",\n")?;
                line_len = 0;
                if is_last {
                    f.write_fmt(format_args!("and `{key}`"))?;
                    line_len += 5 + key.len();
                } else {
                    f.write_fmt(format_args!("`{key}`"))?;
                    line_len += 2 + key.len();
                }
            } else {
                line_len += chunk_len;
                if is_first {
                    f.write_fmt(format_args!("`{key}`"))?;
                } else if is_last {
                    f.write_fmt(format_args!(", and `{key}`"))?;
                } else {
                    f.write_fmt(format_args!(", `{key}`"))?;
                }
            }
        }
        Ok(())
    }
}
