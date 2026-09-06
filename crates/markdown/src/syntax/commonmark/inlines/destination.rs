use super::entities::unescape;
use crate::engine::{Budget, ParseError};

type Destination = Option<(usize, String, Option<String>)>;
pub(crate) fn destination(
    source: &str,
    pos: usize,
    budget: &mut Budget,
    normalize: fn(&str) -> Option<String>,
) -> Result<Destination, ParseError> {
    let bytes = source.as_bytes();
    if bytes.get(pos) != Some(&b'(') {
        return Ok(None);
    }
    let mut i = pos + 1;
    while matches!(bytes.get(i), Some(b' ' | b'\t' | b'\n')) {
        budget.spend(1)?;
        i += 1;
    }
    let angle = bytes.get(i) == Some(&b'<');
    if angle {
        i += 1;
    }
    let start = i;
    let mut depth = 0;
    while i < bytes.len() {
        budget.spend(1)?;
        let b = bytes[i];
        if b == b'\\' && bytes.get(i + 1).is_some_and(u8::is_ascii_punctuation) {
            i += 2;
            continue;
        }
        if angle {
            if b == b'>' {
                break;
            }
            if b == b'<' || b == b'\n' {
                return Ok(None);
            }
        } else {
            if b <= 32 || b == b')' && depth == 0 {
                break;
            }
            if b == b'(' {
                depth += 1;
                budget.depth(depth)?;
            }
            if b == b')' {
                depth -= 1;
            }
        }
        i += 1;
    }
    if depth != 0 || angle && bytes.get(i) != Some(&b'>') {
        return Ok(None);
    }
    let Some(url) = normalize(&unescape(&source[start..i])) else {
        return Ok(None);
    };
    if angle {
        i += 1;
    }
    let before_space = i;
    while matches!(bytes.get(i), Some(b' ' | b'\t' | b'\n')) {
        budget.spend(1)?;
        i += 1;
    }
    let mut title = None;
    if i > before_space && matches!(bytes.get(i), Some(b'"' | b'\'' | b'(')) {
        let closing = if bytes[i] == b'(' { b')' } else { bytes[i] };
        i += 1;
        let start = i;
        while i < bytes.len() && bytes[i] != closing {
            budget.spend(1)?;
            if bytes[i] == b'\\' && bytes.get(i + 1).is_some_and(u8::is_ascii_punctuation) {
                i += 1;
            }
            i += 1;
        }
        if bytes.get(i) != Some(&closing) {
            return Ok(None);
        }
        title = Some(unescape(&source[start..i]));
        i += 1;
        while matches!(bytes.get(i), Some(b' ' | b'\t' | b'\n')) {
            budget.spend(1)?;
            i += 1;
        }
    }
    if bytes.get(i) != Some(&b')') {
        return Ok(None);
    }
    Ok(Some((i + 1, url, title)))
}
