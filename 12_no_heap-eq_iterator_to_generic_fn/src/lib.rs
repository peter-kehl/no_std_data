#![allow(unused)]
//! no_std heapless (bare metal/embedded-friendly) implementation
#![no_std]

use core::fmt::{self, Debug, Formatter};
use utils::OurResult;

#[derive(Debug, PartialEq)]
pub struct Dna<'a>(&'a str);

pub enum Rna<'a> {
    GivenNucleotides(&'a str), // RNA nucleotides
    // Original DNA nucleotides, but *not* transformed. Instead, it will
    // generate RNA nucleotides on the fly by iterating when the consumer calls
    // `PartialEq::eq(...)` on `self`.
    DnaBased(&'a str),
}

impl<'a> Dna<'a> {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(dna: &'a str) -> OurResult<Self> {
        match utils::check_dna(dna) {
            Ok(()) => Ok(Self(dna)),
            Err(i) => Err(i),
        }
    }

    pub fn into_rna(self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl<'a> Rna<'a> {
    /** On error return Err with a 0-based index of the first incorrect character. */
    pub fn new(rna: &'a str) -> OurResult<Self> {
        match utils::check_rna_str(rna) {
            Ok(()) => Ok(Self::GivenNucleotides(rna)),
            Err(i) => Err(i),
        }
    }

    fn eq_iterate_other<I>(&self, other_rna_chars: I) -> bool
    where
        I: Iterator<Item = char>,
    {
        match self {
            Rna::GivenNucleotides(rna) => rna.chars().eq(other_rna_chars),
            Rna::DnaBased(dna) => dna.chars().map(utils::dna_to_rna).eq(other_rna_chars),
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::GivenNucleotides(rna) => other.eq_iterate_other(rna.chars()),
            Self::DnaBased(dna) => other.eq_iterate_other(dna.chars().map(utils::dna_to_rna)),
        }
    }
}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "GivenNucleotides {{{rna}}}")?;
            }
            Rna::DnaBased(dna) => {
                write!(f, "DnaBased {{{dna}}} which translates to ")?;
                // Compared to ../../no_std-no_heap-slices-iterator here we
                // don't have self.iter(). So we map dna to rna here:
                dna.chars()
                    .map(utils::dna_to_rna)
                    .try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
pub mod test {
    // TODO bookmark & Embed Presentation: `arrform` is an heapless no_std
    // alternative to format!(...). New to Rust? Exclamation mark indicates a
    // macro invocation

    // Emb. Presentation: However, your unit tests can use full `std`:
    extern crate alloc;
    use alloc::format;

    #[test]
    #[allow(unused_must_use)]
    fn test_rna_given_nucleotides_debug() {
        super::Dna::new("GCTA").map(|dna| {
            let rna = dna.into_rna();
            let rna_dbg = format!("{:?}", rna);
            assert_eq!(
                "RNA {DnaBased {GCTA} which translates to CGAU}",
                rna_dbg.as_str()
            );
        });
    }
}
