// Pairing follows markdown-it 14.3.0; see THIRD_PARTY_NOTICES.md.
use super::token::{Delimiter, Token, TokenKind};
use crate::{ParseError, engine::Budget};

pub(super) fn balance(
    tokens: &mut [Token],
    ds: &mut [Delimiter],
    budget: &mut Budget,
) -> Result<(), ParseError> {
    let mut bottoms = std::collections::HashMap::<(usize, u8), [isize; 6]>::new();
    let mut jumps = vec![0; ds.len()];
    let (mut header, mut last_token) = (0, None);
    for closer in 0..ds.len() {
        budget.spend(1)?;
        if ds[header].key != ds[closer].key || last_token != ds[closer].token.checked_sub(1) {
            header = closer;
        }
        last_token = Some(ds[closer].token);
        if !ds[closer].close {
            continue;
        }
        let bottom = bottoms.entry(ds[closer].key).or_insert([-1; 6]);
        let parameter = usize::from(ds[closer].open) * 3 + ds[closer].length % 3;
        let min = bottom[parameter];
        let mut opener = header as isize - jumps[header] as isize - 1;
        let mut new_min = opener;
        while opener > min {
            budget.spend(1)?;
            let o = opener as usize;
            let a = ds[o];
            let b = ds[closer];
            let odd = (a.close || b.open)
                && (a.length + b.length).is_multiple_of(3)
                && (!a.length.is_multiple_of(3) || !b.length.is_multiple_of(3));
            if a.key == b.key && a.open && a.end.is_none() && !odd {
                let jump = if o > 0 && !ds[o - 1].open {
                    jumps[o - 1] + 1
                } else {
                    0
                };
                jumps[closer] = closer - o + jump;
                jumps[o] = jump;
                ds[closer].open = false;
                ds[o].end = Some(closer);
                ds[o].close = false;
                new_min = -1;
                last_token = None;
                break;
            }
            opener -= jumps[o] as isize + 1;
        }
        if new_min != -1 {
            bottom[parameter] = new_min;
        }
    }

    let mut i = ds.len();
    while i > 0 {
        budget.spend(1)?;
        i -= 1;
        let Some(end) = ds[i].end else {
            continue;
        };
        let strong = ds[i].pairing.combine_pairs
            && i > 0
            && end + 1 < ds.len()
            && ds[i - 1].end == Some(end + 1)
            && ds[i - 1].key == ds[i].key
            && ds[i - 1].token + 1 == ds[i].token
            && ds[end].token + 1 == ds[end + 1].token;
        let open_token = ds[i].token;
        let close_token = ds[end].token;
        tokens[open_token].kind = TokenKind::Open(ds[i].pairing.make, strong);
        tokens[close_token].kind = TokenKind::Close;
        if strong {
            tokens[open_token].start = tokens[ds[i - 1].token].start;
            tokens[close_token].end = tokens[ds[end + 1].token].end;
            tokens[ds[i - 1].token].kind = TokenKind::Empty;
            tokens[ds[end + 1].token].kind = TokenKind::Empty;
            i -= 1;
        }
    }
    Ok(())
}
