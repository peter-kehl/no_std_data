//! no_std heapless (bare metal/embedded-friendly)
#![no_std]

use core::fmt::{self, Debug, Formatter};
use utils::{checks, DnaTrait, OurResult, RnaTrait};

const MAX_NUM_RNA_NUCLEOTIDES: usize = 12;

// @TODO Others: Derive/impl Clone.

/// DNA (DNA nucleotide sequence). `Dna` itself is `&str` slice-based. (Sufficient for our purpose.)
/// Only `Rna` is array-based.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dna<'a>(&'a str);

/// RNA (RNA nucleotide sequence). Storing RNA nucleotides.
///
/// We don't derive [`PartialEq`] or [`Debug`] or [`Clone`] or [`Copy`]. If we were using
/// [Serde](https://docs.rs/serde/latest/serde/), we wouln't derive its `Serialize` either. Why?
/// Because an [`Rna`] instance may contain leftover nucleotides.
///
/// Let's say we have derived [`PartialEq`] or [`Debug`] or [`Clone`] or [`Copy`]. Then (possibly
/// later) we add modification methods, but we'd forget to wipe out any unused characters after any
/// modification that shortens `len`. If we left in the derived [`PartialEq`] and [`Clone`] or
/// `Serialize`:
/// - Two instances with the same `len` and `rna[..len]` would be treated as unequal if their unused
///   characters (left from before the modification) would differ. That's a incorrect behavior, and
///   insecure, too (because it reveals some information about the past content). And,
/// - Formatting or serializing an instance after a modification could make (potentially
///   confidential) previous characters leak out!
///
/// Security and mutation: Properly implementing similar types is difficult. Otherwise they may leak
/// older data. (Mutation methods and related wiping out such data is not in our scope.)
///
/// Alternatively, we could derive all the above mentioned traits, if we wipe out any unused `rna`
/// slots after any modification.
///
/// Deriving [`Default`] makes the new instance valid, because it sets `len` to 0. However, this
/// works for [`MAX_NUM_RNA_NUCLEOTIDES`] being not more than 32. Otherwise we'd need to initialize
/// the array ourselves with [`core::array::from_fn`].
#[derive(Default)]
pub struct Rna {
    rna: [char; MAX_NUM_RNA_NUCLEOTIDES],
    len: usize,
}

impl<'a> DnaTrait<'a, Rna> for Dna<'a> {
    fn new(dna: &'a str) -> OurResult<Self> {
        checks::check_dna(dna)?;
        Ok(Self(dna))
    }

    fn into_rna(&self) -> Rna {
        Rna::new_from_iter(self.0.chars().map(utils::dna_to_rna)).expect("RNA")
    }
}

impl<'a> RnaTrait<'a> for Rna {
    /// Create a new [`Rna`] instance with given RNA nucleotides -[`Rna::GivenNucleotides`] variant.
    /// If `rna` is valid, return  
    /// [`Some(Rna)`](Some<Rna>) containing the new instance. On error return [`Err`] with a 0-based
    /// index of the first incorrect character.
    fn new(rna: &'a str) -> OurResult<Self> {
        Self::new_from_iter(rna.chars())
    }
}
impl Rna {
    fn new_from_iter(rna_iter: impl Iterator<Item = char>) -> OurResult<Self> {
        //@TODO check if Default works for MAX_NUM_RNA_NUCLEOTIDES over 25
        let mut result = Rna::default();
        /*let mut result = Rna {
            rna: [char::default(); MAX_NUM_RNA_NUCLEOTIDES],
            len: 0
        };*/
        for c in rna_iter {
            result.rna[result.len] = c;
            result.len += 1;
        }
        checks::check_rna_chars(result.chars())?;
        Ok(result)
    }

    fn chars(&self) -> &[char] {
        &self.rna[..self.len]
    }
}

impl PartialEq for Rna {
    fn eq(&self, other: &Self) -> bool {
        self.chars() == other.chars()
    }
}
/// Not necessary, but valid.
impl Eq for Rna {}

impl Debug for Rna {
    /// Compared to [../../no_heap-slices-iterator]([../../no_heap-slices-iterator),
    /// [Self::DnaBased] variant here doesn't have `self.iter()`. So we map DNA to RNA chars here.
    /// Honoring default derived format of a newtype-based implementation, so we can re-use same tests.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        // In `no_std` with heap we could have:
        #[cfg(feature = "with_heap")]
        {
            extern crate alloc;
            use alloc::string::String;
            write!(f, "RNA({})", self.chars().iter().collect::<String>())
        }
        // But to make this heapless-compatible, we iterate over characters instead:
        #[cfg(not(feature = "with_heap"))]
        {
            write!(f, "Rna(\"")?;
            self.chars().iter().try_for_each(|&c| write!(f, "{}", c))?;
            write!(f, "\")")
        }
    }
}

impl Clone for Rna {
    fn clone(&self) -> Self {
        let mut rna = [char::default(); MAX_NUM_RNA_NUCLEOTIDES];
        rna[..self.len].copy_from_slice(&self.rna[..self.len]);
        Self { rna, len: self.len }
    }
}