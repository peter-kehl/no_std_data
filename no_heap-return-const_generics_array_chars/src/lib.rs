//! no_std heapless (bare metal/embedded-friendly)
#![no_std]
#![allow(unused)] //@TODO remove

use core::fmt::{self, Debug, Formatter};
use core::ops::Deref;
use core::str::Chars;

/// DNA (DNA nucleotide sequence).
/// 
/// `const N` parameter does not affect storage of this type. It's used only to infer respective
/// ['Rna`] size when calling [`Dna::into_rna`].
#[derive(Debug, PartialEq, Eq)]
pub struct Dna<'a, const N: usize>(&'a str);

/// RNA (RNA nucleotide sequence).
/// 
/// Usable only if the required `const N` parameter is known in compile time.
pub struct Rna<const N: usize>([char; N]);

impl<const L: usize, const R: usize> PartialEq<Rna<R>> for Rna<L> {
    fn eq(&self, other: &Rna<R>) -> bool {
        todo!()
    }
}

impl<'a, const N: usize> Dna<'a, N> {
    /// Create a new [`Dna`] instance with given DNA nucleotides. If `dna` is valid, return  
    /// [`Some(Dna)`](Some<Dna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(dna: &'a str) -> Result<Self, usize> {
        shared::check_dna(dna)?;
        Ok(Self(dna))
    }

    /// Create a [DNA-based variant of `Rna`](Rna::GivenNucleotides) instance, based on `self`. No
    /// transformation/iteration is done yet - see [`Rna::DnaBased`].
    pub fn into_rna(self) -> Rna<N> {
        todo!()
    }
}

/// Iterator over RNA nucleotides. This iterates over either:
/// - given RNA ones (for [RnaIterator::GivenNucleotides]), or
/// - translated on the fly from DNA ones (for [RnaIterator::DnaBased]).
enum RnaIterator<'a> {
    GivenNucleotides(Chars<'a>),
    DnaBased(Chars<'a>),
}

impl<'a> Rna<'a> {
    /// Create a new [`Rna`] instance with given RNA nucleotides -[`Rna::GivenNucleotides`] variant.
    /// If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    pub fn new(rna: &'a str) -> Result<Self, usize> {
        match shared::check_rna_str(rna) {
            Ok(()) => Ok(Self::GivenNucleotides(rna)),
            Err(i) => Err(i),
        }
    }

    /// Create an [`RnaIterator`] over `self`'s RNA nucleotides (chars). For  
    /// [RNA-based variant](Rna::GivenNucleotides) this iterates over the given nucleotides. For  
    /// [DNA-based variant](Rna::DnaBased) this translates the DNA nucleotides to RNA ones on the
    /// fly (without storing them anywhere).
    ///
    /// We can't declare return type here as `impl Iterator<Item = char>` if we return a different
    /// expression for each `match *self` branch here. Why? Such alternative results would be two
    /// different implementations of [`Iterator`]. Hence we have our own type: [`RnaIterator`].
    fn iter(&self) -> RnaIterator<'a> {
        match *self {
            Rna::GivenNucleotides(rna) => RnaIterator::GivenNucleotides(rna.chars()),

            Rna::DnaBased(dna) => RnaIterator::DnaBased(dna.chars()),
        }
    }
}

impl<'a> Iterator for RnaIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RnaIterator::DnaBased(chars) => {
                let dna = chars.next();
                match dna {
                    Some(nucl) => Some(shared::dna_to_rna(nucl)),
                    None => None,
                }
            }
            RnaIterator::GivenNucleotides(chars) => chars.next(),
        }
    }
}

impl<'a> PartialEq for Rna<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}
/// Not necessary for our purpose, but valid.
impl<'a> Eq for Rna<'a> {}

impl<'a> Debug for Rna<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "RNA {{")?;
        match self {
            Rna::GivenNucleotides(rna) => {
                write!(f, "GivenNucleotides {{{rna}}}")?;
            }
            Rna::DnaBased(dna) => {
                write!(f, "DnaBased {{{dna}}} which translates to ")?;
                self.iter().try_for_each(|c| write!(f, "{c}"))?;
            }
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
pub mod test {
    //! Test(s) on top of Exercism's tests (which are in `../tests/`).

    // Unit tests of a `no_std` crate can't use `std` either. However, they can use heap (even if
    // the crate being tested doesn't have access to heap).
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
