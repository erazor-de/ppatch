mod masked_byte;
mod opt_fifo;
mod pattern;
mod pattern_replace_iterator;
mod pattern_replace_result_iterator;
mod pattern_search_iterator;
mod pattern_search_ref_iterator;
mod pattern_search_result_iterator;
mod pattern_skip_iterator;
mod pattern_skip_result_iterator;
mod pattern_take_iterator;
mod pattern_take_result_iterator;
mod replacer;
mod searcher;
mod skipper;
mod taker;

pub mod prelude;

use crate::opt_fifo::OptFifo;
use crate::replacer::Replacer;
use crate::searcher::Searcher;
use crate::skipper::Skipper;
use crate::taker::Taker;

pub use crate::masked_byte::MaskedByte;
pub use crate::pattern::Pattern;
pub use crate::pattern_replace_iterator::{PatternReplaceExt, PatternReplaceIterator};
pub use crate::pattern_replace_result_iterator::{
    PatternReplaceResultExt, PatternReplaceResultIterator,
};
pub use crate::pattern_search_iterator::{PatternSearchExt, PatternSearchIterator};
pub use crate::pattern_search_ref_iterator::{PatternSearchRefExt, PatternSearchRefIterator};
pub use crate::pattern_search_result_iterator::{
    PatternSearchResultExt, PatternSearchResultIterator,
};
pub use crate::pattern_skip_iterator::{PatternSkipExt, PatternSkipIterator};
pub use crate::pattern_skip_result_iterator::{PatternSkipResultExt, PatternSkipResultIterator};
pub use crate::pattern_take_iterator::{PatternTakeExt, PatternTakeIterator};
pub use crate::pattern_take_result_iterator::{PatternTakeResultExt, PatternTakeResultIterator};

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not parse {}: {}", string, source))]
    ParseError {
        string: String,
        source: crate::masked_byte::Error,
    },

    #[snafu(display("Overhanging replace pattern is not fully defined"))]
    ReplaceNotDefined,

    #[snafu(display(""))]
    IteratorError { source: Box<dyn std::error::Error> },
}

pub type Result<T, E = crate::Error> = std::result::Result<T, E>;

#[derive(PartialEq, Debug)]
pub enum PatternSearchType<T> {
    Match { data: Vec<T>, index: usize },
    NonMatch(T),
}
