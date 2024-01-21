mod hamming_distance;
mod edit_distance;
mod trie;

pub use self::hamming_distance::hamming_distance1;
pub use self::hamming_distance::hamming_distance2;
pub use self::hamming_distance::hamming_distance_str;

pub use self::edit_distance::edit_distance1;
pub use self::edit_distance::edit_distance2;

pub use self::trie::Trie;