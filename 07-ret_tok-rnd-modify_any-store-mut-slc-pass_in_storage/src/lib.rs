//! This crate doesn't implement utils::{DnaTrait, RnaTrait}, because the function signature of
//! [`Dna::into_rna`] here is different - it needs an extra parameter (storage slice).
#![no_std]

use core::fmt::{self, Debug, Formatter};
use core::str;
use utils::{checks, OurResult};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dna<'a>(&'a str);

/// This can't derive, neither implement, [`Clone`]. Why? Because a mutable reference (`rna` field)
/// can't be cloned.
/// -- TODO change that for the immutable version (once implemented)
///
/// New to Rust? We can't just clone the referenced data and use a new reference, because any data
/// in Rust has to be owned from exactly one place. However,  the goal of this implementation is not
/// to own the data,  but to (mutably) refer to it instead.
pub enum Rna<'a> {
    GivenNucleotides(&'a str),
    /// The characters in the byte slice represent, or will represent, RNA.
    MutableNucleotides {
        /// The whole/available storage.
        /// TODO implement mutable operations. Panic on the first invariant.
        rna: &'a mut [u8],
        /// Length of the valid subslice (used storage).
        len: usize,
    },
}

impl<'a> Dna<'a> {
    pub fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    pub fn into_rna<'s>(&self, storage: &'s mut [u8]) -> Rna
    where
        's: 'a,
    {
        Rna::new_from_iter_and_storage(self.0.chars().map(utils::dna_to_rna), storage).expect("RNA")
    }
}

impl<'a> Rna<'a> {
    pub fn new(rna: &'a str) -> OurResult<Self> {
        checks::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }

    fn new_from_iter_and_storage<'s>(
        rna_iter: impl Iterator<Item = char>,
        storage: &'s mut [u8],
    ) -> OurResult<Self>
    where
        's: 'a,
    {
        let len = utils::char_iter_to_bytes(storage, rna_iter);
        let result = Self::MutableNucleotides { rna: storage, len };
        checks::check_rna_str(result.as_str())?;
        Ok(result)
    }

    fn as_str(&self) -> &str {
        match self {
            Self::GivenNucleotides(rna) => rna,
            Self::MutableNucleotides { rna, len } => {
                str::from_utf8(&rna[..*len]).expect("UTF-8 encoded string of RNA nucleotides")
            }
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
/// Not necessary, but valid.
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Rna(\"{}\")", self.as_str())
    }
}

impl<'l, 'r> PartialEq<&Rna<'r>> for Rna<'l> {
    fn eq(&self, other: &&Rna<'r>) -> bool {
        self.as_str() == other.as_str()
    }
}
impl<'l, 'r> PartialEq<Rna<'r>> for &Rna<'l> {
    fn eq(&self, other: &Rna<'r>) -> bool {
        self.as_str() == other.as_str()
    }
}

#[cfg(test)]
pub mod test {
    use super::{Dna, Rna};

    /// Testing that equality is defined for references - because we can't share instances of this
    /// type in any other way.
    #[test]
    fn test_rna_shared() {
        let rna = Rna::new("CGAU").unwrap();

        let dna = Dna::new("GCTA").unwrap();
        let mut dna_transformed_storage = [0u8; 4];
        let dna_transformed = dna.into_rna(&mut dna_transformed_storage);

        assert_eq!(rna, dna_transformed);
        assert_eq!(dna_transformed, rna);

        let rna_ref = &rna;
        assert_eq!(rna, rna_ref);
        assert_eq!(rna_ref, rna);

        assert_eq!(rna_ref, dna_transformed);
        assert_eq!(dna_transformed, rna_ref);

        let dna_transformed_ref = &dna_transformed;
        assert_eq!(rna, dna_transformed_ref);
        assert_eq!(dna_transformed_ref, rna);

        assert_eq!(rna_ref, dna_transformed_ref);
        assert_eq!(dna_transformed_ref, rna_ref);
    }
}
