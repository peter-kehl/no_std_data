//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

use core::fmt::{self, Debug, Formatter};
use utils::{checks, DnaTrait, OurResult, RnaTrait};

/// DNA (DNA nucleotide sequence).
///
/// Implementing [`Eq`] is not necessary for our purpose, but valid.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

#[derive(Clone, Copy)]
pub enum Rna<'a> {
    GivenNucleotides(&'a str),
    DnaBased(&'a str),
}

impl<'a> DnaTrait<'a, Rna<'a>> for Dna<'a> {
    fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    fn into_rna(&self) -> Rna<'a> {
        match self {
            Dna(dna) => Rna::DnaBased(dna),
        }
    }
}

impl<'a> RnaTrait<'a> for Rna<'a> {
    /// Create a new [`Rna`] instance with given RNA nucleotides -[`Rna::GivenNucleotides`] variant.
    /// If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(rna: &'a str) -> OurResult<Self> {
        checks::check_rna_str(rna)?;
        Ok(Self::GivenNucleotides(rna))
    }
}

impl<'a> Rna<'a> {
    /// Get an [`Iterator`] over `self`'s RNA nucleotides (chars), and call `closure` with that
    /// (`self`'s) iterator and `other_rna_chars`. For  
    /// [RNA-based variant](Rna::GivenNucleotides) this iterates over the given nucleotides. For  
    /// [DNA-based variant](Rna::DnaBased) this translates the DNA nucleotides to RNA ones on the
    /// fly (without storing them anywhere).
    fn with_chars<R, C>(&self, other_rna_chars: &mut dyn Iterator<Item = char>, closure: C) -> R
    where
        C: Fn(&mut dyn Iterator<Item = char>, &mut dyn Iterator<Item = char>) -> R,
    {
        match self {
            Rna::GivenNucleotides(rna) => closure(&mut rna.chars(), other_rna_chars),
            Rna::DnaBased(dna) => closure(&mut dna.chars().map(utils::dna_to_rna), other_rna_chars),
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        fn inner(
            iter_one: &mut dyn Iterator<Item = char>,
            iter_two: &mut dyn Iterator<Item = char>,
        ) -> bool {
            iter_one.eq(iter_two)
        }

        match self {
            Self::GivenNucleotides(rna) => other.with_chars(&mut rna.chars(), inner),
            Self::DnaBased(dna) => other.with_chars(&mut dna.chars().map(utils::dna_to_rna), inner),
        }
    }
}
/// Not necessary, but valid.
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    /// Compared to [../../no_heap-slices-iterator]([../../no_heap-slices-iterator),
    /// [Self::DnaBased] variant here doesn't have `self.iter()`. So we map DNA to RNA chars here.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Rna(\"")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "{rna}")?;
            }
            Rna::DnaBased(dna) => {
                dna.chars()
                    .map(utils::dna_to_rna)
                    .try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "\")")
    }
}
