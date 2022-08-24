//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

// @TODO Others: remove import of Debug - where it's derived only

use core::fmt::{self, Debug, Formatter};
use core::str;

const MAX_NUM_RNA_NUCLEOTIDES: usize = 12;

// @TODO Others: Derive/impl Clone.

/// DNA (DNA nucleotide sequence). `Dna` itself is `&str` slice-based. (Sufficient for our purpose.)
/// Only `Rna` is array-based.
///
/// Implementing [`Eq`] is not necessary, but valid.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dna<'a>(&'a str);

/// RNA (RNA nucleotide sequence). Storing RNA nucleotides.
///
/// We can't derive [`PartialEq`] or [`Debug`]. Why? Because an `Rna` instance may contain leftover
/// nucleotides.
///
/// Security: Properly implementing similar types is difficult. Otherwise they may leak older data.
/// (Wiping out such data is not in our scope.)
///
/// Deriving [`Default`] makes the new instance valid, because it sets `len` to 0. However, this
/// works only up to a fixed limit (25?). Otherwise we'd need to initialize the array ourselves with
/// [`core::array::from_fn`].
#[derive(Default, Clone)]
pub struct Rna {
    rna: [u8; MAX_NUM_RNA_NUCLEOTIDES],
    len: usize,
}

impl<'a> Dna<'a> {
    pub fn new(dna: &'a str) -> utils::Result<Self> {
        // @TODO in other projects: use ? op, and add a link
        utils::check_dna(dna)?;
        Ok(Self(dna))
    }

    pub fn into_rna(self) -> Rna {
        Rna::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA")
    }
}

impl Rna {
    /// Create a new [`Rna`] instance with given RNA nucleotides. If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new<'a>(rna: &'a str) -> utils::Result<Self> {
        Self::new_from_iter(rna.chars())
    }

    fn new_from_iter(rna_iter: impl Iterator<Item = char>) -> utils::Result<Self> {
        let mut result = Rna::default();
        for c in rna_iter {
            result.rna[result.len] = c as u8;
            result.len += 1;
        }
        // This would not work for Unicode in general.
        utils::check_rna_char_iter(result.bytes().iter().map(|&b| b as char))?;
        Ok(result)
    }

    fn bytes(&self) -> &[u8] {
        &self.rna[..self.len]
    }
}

impl PartialEq for Rna {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}
/// Not necessary, but valid.
impl Eq for Rna {}

impl Debug for Rna {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "RNA {{{}}}",
            str::from_utf8(self.bytes()).expect("UTF-8 encoded string")
        )
    }
}

#[cfg(test)]
pub mod test {
    extern crate alloc;
    use alloc::format;

    #[test]
    #[allow(unused_must_use)]
    fn test_rna_given_nucleotides_debug() {
        super::Dna::new("GCTA").map(|dna| {
            let rna = dna.into_rna();
            let rna_dbg = format!("{:?}", rna);
            assert_eq!("RNA {CGAU}", rna_dbg.as_str());
        });
    }
}
