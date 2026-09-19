use super::*;

pub(crate) const LTR_ISOLATE_START: char = '\u{2066}';
const RTL_ISOLATE_START: char = '\u{2067}';
pub(crate) const ISOLATE_END: char = '\u{2069}';

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TextBaseDirection {
    Ltr,
    Rtl,
}

pub(crate) fn text_char_boundaries(text: &str) -> Vec<usize> {
    let mut boundaries = Vec::with_capacity(text.graphemes(true).count() + 1);
    boundaries.push(0);
    for (idx, grapheme) in text.grapheme_indices(true) {
        boundaries.push(idx + grapheme.len());
    }
    boundaries
}

pub(crate) fn base_direction_for_text(text: &str) -> TextBaseDirection {
    let bidi = BidiInfo::new(text, None);
    bidi.paragraphs
        .first()
        .map(|paragraph| {
            if paragraph.level.is_rtl() {
                TextBaseDirection::Rtl
            } else {
                TextBaseDirection::Ltr
            }
        })
        .unwrap_or(TextBaseDirection::Ltr)
}

pub(crate) fn resolve_truncate_side(
    truncate_side: UTextTruncateSide,
    _base_direction: TextBaseDirection,
) -> UTextTruncateSide {
    match truncate_side {
        UTextTruncateSide::Auto => UTextTruncateSide::End,
        side => side,
    }
}

fn isolate_for_direction(segment: &str, base_direction: TextBaseDirection) -> String {
    if segment.is_empty() {
        return String::new();
    }

    let isolate_start = match base_direction {
        TextBaseDirection::Ltr => LTR_ISOLATE_START,
        TextBaseDirection::Rtl => RTL_ISOLATE_START,
    };

    let mut isolated = String::with_capacity(segment.len() + 2);
    isolated.push(isolate_start);
    isolated.push_str(segment);
    isolated.push(ISOLATE_END);
    isolated
}

pub(crate) fn build_truncate_candidate(
    text: &str,
    boundaries: &[usize],
    truncate_side: UTextTruncateSide,
    kept_graphemes: usize,
    base_direction: TextBaseDirection,
) -> String {
    let total_graphemes = boundaries.len().saturating_sub(1);
    if total_graphemes == 0 {
        return super::truncation::DEFAULT_ELLIPSIS.to_string();
    }

    match truncate_side {
        UTextTruncateSide::End | UTextTruncateSide::Auto => {
            let keep = kept_graphemes.min(total_graphemes);
            let prefix = &text[..boundaries[keep]];
            let isolated_prefix = isolate_for_direction(prefix, base_direction);
            let mut candidate = String::with_capacity(
                isolated_prefix.len() + super::truncation::DEFAULT_ELLIPSIS.len(),
            );
            candidate.push_str(&isolated_prefix);
            candidate.push_str(super::truncation::DEFAULT_ELLIPSIS);
            candidate
        }
        UTextTruncateSide::Start => {
            let keep = kept_graphemes.min(total_graphemes);
            let suffix = &text[boundaries[total_graphemes - keep]..];
            let isolated_suffix = isolate_for_direction(suffix, base_direction);
            let mut candidate = String::with_capacity(
                super::truncation::DEFAULT_ELLIPSIS.len() + isolated_suffix.len(),
            );
            candidate.push_str(super::truncation::DEFAULT_ELLIPSIS);
            candidate.push_str(&isolated_suffix);
            candidate
        }
        UTextTruncateSide::Middle => {
            let keep = kept_graphemes.min(total_graphemes);
            let keep_prefix = keep.div_ceil(2);
            let keep_suffix = keep / 2;
            let prefix = &text[..boundaries[keep_prefix]];
            let suffix = &text[boundaries[total_graphemes - keep_suffix]..];
            let isolated_prefix = isolate_for_direction(prefix, base_direction);
            let isolated_suffix = isolate_for_direction(suffix, base_direction);
            let mut candidate = String::with_capacity(
                isolated_prefix.len()
                    + super::truncation::DEFAULT_ELLIPSIS.len()
                    + isolated_suffix.len(),
            );
            candidate.push_str(&isolated_prefix);
            candidate.push_str(super::truncation::DEFAULT_ELLIPSIS);
            candidate.push_str(&isolated_suffix);
            candidate
        }
    }
}
