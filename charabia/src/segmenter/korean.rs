use std::borrow::Cow;
use std::sync::LazyLock;

use lindera::dictionary::{load_embedded_dictionary, DictionaryKind};
use lindera::mode::{Mode, Penalty};
use lindera::segmenter::Segmenter as LinderaSegmenter;

use crate::segmenter::Segmenter;

/// Korean specialized [`Segmenter`].
///
/// This Segmenter uses lindera internally to segment the provided text.
pub struct KoreanSegmenter;

static LINDERA: LazyLock<LinderaSegmenter> = LazyLock::new(|| {
    let dictionary = load_embedded_dictionary(DictionaryKind::KoDic).unwrap();
    LinderaSegmenter::new(Mode::Decompose(Penalty::default()), dictionary, None)
});

impl Segmenter for KoreanSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let tokens = LINDERA.segment(Cow::Borrowed(to_segment)).unwrap();

        let result: Vec<&'o str> = tokens
            .into_iter()
            .map(|token| {
                let start = token.byte_start;
                let end = token.byte_end;
                &to_segment[start..end]
            })
            .collect();

        Box::new(result.into_iter())
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "한국어의형태해석을실시할수있습니다 123 456.";

    const SEGMENTED: &[&str] = &[
        "한국어",
        "의",
        "형태",
        "해석",
        "을",
        "실시",
        "할",
        "수",
        "있",
        "습니다",
        " ",
        "123",
        " ",
        "456",
        ".",
    ];

    const TOKENIZED: &[&str] = &[
        "한국어",
        "의",
        "형태",
        "해석",
        "을",
        "실시",
        "할",
        "수",
        "있",
        "습니다",
        " ",
        "123",
        " ",
        "456",
        ".",
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(KoreanSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Hangul, Language::Kor);
}
