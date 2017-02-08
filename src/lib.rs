//! Get the index of a subslice into its parent slice.
//!
//! ## Example
//!
//! ```rust
//! use subslice_index::subslice_index;
//!
//! let arr = [1, 2, 3, 4, 5, 6];
//!
//! let slice = &arr[..];
//! let subslice = &slice[2..4];
//!
//! assert_eq!(subslice, [3, 4]);
//! assert_eq!(subslice_index(slice, subslice), 2);
//! ```

/// Get the index into the given slice of the first item in the given subslice.
///
/// The subslice must come from the slice, otherwise the returned index is invalid.
pub fn subslice_index<'a, T>(slice: &'a [T], subslice: &'a [T]) -> usize {
    let diff = subslice.as_ptr() as usize - slice.as_ptr() as usize;
    let idx = diff / std::mem::size_of::<T>();

    debug_assert!(idx <= slice.len());

    idx
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slice_index() {
        let m = [0u8; 42];

        assert_eq!(subslice_index(&m[..], &m[..]), 0);
        assert_eq!(subslice_index(&m[..], &m[13..]), 13);
        assert_eq!(subslice_index(&m[..], &m[13..20]), 13);
        assert_eq!(subslice_index(&m[..], &m[42..]), 42);

        let m = [0u32; 42];

        assert_eq!(subslice_index(&m[..], &m[..]), 0);
        assert_eq!(subslice_index(&m[..], &m[13..]), 13);
        assert_eq!(subslice_index(&m[..], &m[13..20]), 13);
        assert_eq!(subslice_index(&m[..], &m[42..]), 42);
    }
}
