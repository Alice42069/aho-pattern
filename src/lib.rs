pub mod error;

use std::{fmt, str::FromStr /*, time::Instant*/};

use aho_corasick::AhoCorasick;

use crate::error::{Error, Result};

pub trait AhoPattern {
    fn find_patterns(haystack: &[u8], patterns: Vec<PatternBytes>) -> Result<Vec<Option<usize>>>;
}

impl AhoPattern for AhoCorasick {
    fn find_patterns(
        haystack: &[u8],
        mut patterns: Vec<PatternBytes>,
    ) -> Result<Vec<Option<usize>>> {
        // let pre_processing_start = Instant::now();

        // we want to get rid of them but store them for the return
        let starting_wildcards: Vec<usize> = patterns
            .iter_mut()
            .map(|p| {
                let count = p.0.iter().take_while(|b| b.is_none()).count();
                p.0.drain(..count);
                count
            })
            .collect();

        let mut output: Vec<Option<usize>> = vec![None; patterns.len()];

        // same length as input patterns, (largets sequence of Some values, index of that data from orginal input pattern, parsed patterns)
        let input_table: Vec<(Vec<u8>, usize, (Vec<u8>, Vec<bool>))> = {
            let mut table = Vec::new();

            for p in &patterns {
                let mut best_start = 0;
                let mut best_length = 0;

                let mut currrent_start = 0;
                let mut current_length = 0;

                for (i, b) in p.0.iter().enumerate() {
                    if b.is_some() {
                        if current_length == 0 {
                            currrent_start = i;
                        }

                        current_length += 1;

                        if current_length > best_length {
                            best_length = current_length;
                            best_start = currrent_start;
                        }
                    } else {
                        current_length = 0;
                    }
                }

                table.push((
                    p.0[best_start..best_start + best_length]
                        .iter()
                        .map(|x| x.unwrap())
                        .collect(),
                    best_start,
                    p.parse_pattern(),
                ));
            }

            table
        };

        // println!("Pre-Processing took {:?}", pre_processing_start.elapsed());

        // let ac_scan_time = Instant::now();

        let ac = AhoCorasick::new(input_table.iter().map(|p| &p.0))?;

        // println!("Aho-Corasick scan took {:?}", ac_scan_time.elapsed());

        // let post_processing_start = Instant::now();

        for m in ac.find_iter(haystack) {
            let index = m.pattern().as_usize();
            let table = &input_table[index];

            let offset = m.start();
            let pattern_offset = table.1;

            let pattern = &patterns[index];

            let match_buffer =
                &haystack[offset - pattern_offset..offset - pattern_offset + pattern.0.len()];

            let parsed_pattern = &table.2;

            if PatternBytes::matches_pattern(match_buffer, &parsed_pattern.0, &parsed_pattern.1) {
                output[index] = Some(offset - pattern_offset - starting_wildcards[index]);
            }
        }

        // println!("Post-Processing took {:?}", post_processing_start.elapsed());
        // assert!(output.iter().all(|m| m.is_some()));

        //println!("{:?}", output);

        Ok(output)
    }
}

/// A sequence of bytes where unknown values are represented as `None`
///
/// For example, the byte pattern `48 ? 2E` would be represented as:
/// `[Some(0x48), None, Some(0x2E)]`
#[derive(Debug, Clone)]
pub struct PatternBytes(pub Vec<Option<u8>>);

impl PatternBytes {
    pub fn patterns_from_strs(strs: &[&str]) -> Result<Vec<Self>> {
        strs.iter().map(|&s| Self::from_str(s)).collect()
    }

    pub fn patterns_from_bytes(bytes: &[Vec<u8>]) -> Vec<Self> {
        bytes.iter().map(|b| Self::from(b.clone())).collect()
    }

    fn parse_pattern(&self) -> (Vec<u8>, Vec<bool>) {
        self.0
            .clone()
            .into_iter()
            .map(|b| (b.unwrap_or(0), b.is_none()))
            .unzip()
    }

    fn matches_pattern(buffer: &[u8], pattern: &[u8], mask: &[bool]) -> bool {
        return buffer
            .iter()
            .zip(pattern.iter())
            .zip(mask.iter())
            .all(|((&b, &p), &m)| m || b == p);
    }
}

impl FromStr for PatternBytes {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes = s
            .split_whitespace()
            .map(|t| match t {
                "?" | "??" => Ok(None),
                _ => u8::from_str_radix(t, 16)
                    .map(Some)
                    .map_err(|_| Error::Parsing),
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(PatternBytes(bytes))
    }
}

impl From<Vec<u8>> for PatternBytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value.iter().map(|&b| Some(b)).collect())
    }
}

impl fmt::Display for PatternBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self
            .0
            .iter()
            .map(|opt_byte| match opt_byte {
                Some(b) => format!("{:02X}", b),
                None => "?".to_string(),
            })
            .collect();

        write!(f, "{}", parts.join(" "))
    }
}
