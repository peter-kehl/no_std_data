use utils::{DnaTrait, OurResult, RnaTrait};

/// Most test_* functions are a verbatim copy from original Exercism's tests from
/// https://github.com/exercism/rust/blob/main/exercises/practice/rna-transcription/tests/rna-transcription.rs
pub trait Tests {
    type Dna<'a>: DnaTrait<'a, Self::Rna<'a>>;
    type Rna<'a>: RnaTrait<'a> + 'a;

    // ------ Start end functions from Exercism
    fn test_valid_self_input() {
        assert!(Self::Dna::new("GCTA").is_ok());
    }

    fn test_valid_rna_input() {
        assert!(Self::Rna::new("CGAU").is_ok());
    }

    fn test_invalid_self_input() {
        // Invalid character
        assert_eq!(Self::Dna::new("X").err(), Some(0));
        // Valid nucleotide, but invalid in context
        assert_eq!(Self::Dna::new("U").err(), Some(0));
        // Longer string with contained errors
        assert_eq!(Self::Dna::new("ACGTUXXCTTAA").err(), Some(4));
    }

    fn test_invalid_rna_input() {
        // Invalid character
        assert_eq!(Self::Rna::new("X").unwrap_err(), 0);
        // Valid nucleotide, but invalid in context
        assert_eq!(Self::Rna::new("T").unwrap_err(), 0);
        // Longer string with contained errors
        assert_eq!(Self::Rna::new("ACGUTTXCUUAA").unwrap_err(), 4);
    }

    fn test_acid_equals_acid() {
        assert_eq!(
            Self::Dna::new("CGA").unwrap(),
            Self::Dna::new("CGA").unwrap()
        );
        assert_ne!(
            Self::Dna::new("CGA").unwrap(),
            Self::Dna::new("AGC").unwrap()
        );
        assert_eq!(
            Self::Rna::new("CGA").unwrap(),
            Self::Rna::new("CGA").unwrap()
        );
        assert_ne!(
            Self::Rna::new("CGA").unwrap(),
            Self::Rna::new("AGC").unwrap()
        );
    }

    fn test_transcribes_cytosine_guanine() {
        assert_eq!(
            Self::Rna::new("G").unwrap(),
            Self::Dna::new("C").unwrap().into_rna()
        );
    }

    fn test_transcribes_guanine_cytosine() {
        assert_eq!(
            Self::Rna::new("C").unwrap(),
            Self::Dna::new("G").unwrap().into_rna()
        );
    }

    fn test_transcribes_adenine_uracil() {
        assert_eq!(
            Self::Rna::new("U").unwrap(),
            Self::Dna::new("A").unwrap().into_rna()
        );
    }

    fn test_transcribes_thymine_to_adenine() {
        assert_eq!(
            Self::Rna::new("A").unwrap(),
            Self::Dna::new("T").unwrap().into_rna()
        );
    }

    fn test_transcribes_all_self_to_rna() {
        assert_eq!(
            Self::Rna::new("UGCACCAGAAUU").unwrap(),
            Self::Dna::new("ACGTGGTCTTAA").unwrap().into_rna()
        )
    }
    // ------ End test functions from Exercism

    // ------- Tests on top of Exercism's tests:

    /// Honoring default derived format of a newtype-based implementation. Any other implementations
    /// to conform.
    fn test_rna_given_nucleotides_debug() -> OurResult<()> {
        let rna = Self::Rna::new("CGAU")?;
        let rna_dbg = format!("{:?}", rna);
        assert_eq!("Rna(\"CGAU\")", rna_dbg);
        Ok(())
    }

    /// Honoring default derived format of a newtype-based implementation.
    fn test_rna_from_dna_debug() -> OurResult<()> {
        let dna = Self::Dna::new("GCTA")?;
        let rna = dna.into_rna();
        let rna_dbg = format!("{:?}", rna);
        assert_eq!("Rna(\"CGAU\")", rna_dbg);
        Ok(())
    }

    fn all_tests() {
        Self::test_valid_self_input();
        Self::test_valid_rna_input();
        Self::test_invalid_self_input();
        Self::test_invalid_rna_input();
        Self::test_acid_equals_acid();
        Self::test_transcribes_cytosine_guanine();
        Self::test_transcribes_guanine_cytosine();
        Self::test_transcribes_adenine_uracil();
        Self::test_transcribes_thymine_to_adenine();
        Self::test_transcribes_all_self_to_rna();
        assert!(Self::test_rna_given_nucleotides_debug().is_ok());
        assert!(Self::test_rna_from_dna_debug().is_ok());
    }
}
