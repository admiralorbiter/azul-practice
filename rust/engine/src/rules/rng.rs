use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};

/// Create a seeded RNG from a u64 seed
///
/// This provides deterministic random number generation for reproducible scenarios.
///
/// # Example
///
/// ```
/// use engine::create_rng_from_seed;
/// let mut rng1 = create_rng_from_seed(12345);
/// let mut rng2 = create_rng_from_seed(12345);
/// // Both RNGs will produce identical sequences
/// ```
pub fn create_rng_from_seed(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}

/// Generate a random seed string for display purposes
///
/// Returns a string representation of a random u64 seed.
/// This can be shown to users and later parsed back for reproducibility.
///
/// # Example
///
/// ```
/// use engine::generate_seed_string;
/// let seed = generate_seed_string();
/// println!("Scenario seed: {}", seed);
/// ```
pub fn generate_seed_string() -> String {
    let mut rng = rand::thread_rng();
    let seed: u64 = rng.gen();
    seed.to_string()
}

/// Parse a seed string into a u64
///
/// # Arguments
///
/// * `s` - String representation of a u64 seed
///
/// # Returns
///
/// * `Ok(u64)` - Successfully parsed seed
/// * `Err(String)` - Parse error with message
///
/// # Example
///
/// ```
/// use engine::parse_seed_string;
/// let seed = parse_seed_string("12345").unwrap();
/// assert_eq!(seed, 12345);
/// ```
pub fn parse_seed_string(s: &str) -> Result<u64, String> {
    s.parse::<u64>()
        .map_err(|e| format!("Invalid seed string '{}': {}", s, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeded_rng_deterministic() {
        let mut rng1 = create_rng_from_seed(12345);
        let mut rng2 = create_rng_from_seed(12345);
        
        // Generate 10 random numbers from each RNG
        for _ in 0..10 {
            let n1: u32 = rng1.gen();
            let n2: u32 = rng2.gen();
            assert_eq!(n1, n2, "Seeded RNGs should produce identical sequences");
        }
    }

    #[test]
    fn test_different_seeds_produce_different_sequences() {
        let mut rng1 = create_rng_from_seed(12345);
        let mut rng2 = create_rng_from_seed(54321);
        
        let n1: u32 = rng1.gen();
        let n2: u32 = rng2.gen();
        
        assert_ne!(n1, n2, "Different seeds should produce different sequences");
    }

    #[test]
    fn test_generate_seed_string() {
        let seed1 = generate_seed_string();
        let seed2 = generate_seed_string();
        
        // Both should be valid u64 strings
        assert!(seed1.parse::<u64>().is_ok());
        assert!(seed2.parse::<u64>().is_ok());
        
        // Very likely to be different (not guaranteed, but probability is high)
        // We won't assert inequality since it's technically possible to be the same
    }

    #[test]
    fn test_parse_seed_string_valid() {
        let result = parse_seed_string("12345");
        assert_eq!(result, Ok(12345));
        
        let result = parse_seed_string("0");
        assert_eq!(result, Ok(0));
        
        let result = parse_seed_string("18446744073709551615"); // u64::MAX
        assert_eq!(result, Ok(u64::MAX));
    }

    #[test]
    fn test_parse_seed_string_invalid() {
        let result = parse_seed_string("not a number");
        assert!(result.is_err());
        
        let result = parse_seed_string("-123");
        assert!(result.is_err());
        
        let result = parse_seed_string("12.34");
        assert!(result.is_err());
    }

    #[test]
    fn test_seed_round_trip() {
        let original_seed = 987654321u64;
        let seed_string = original_seed.to_string();
        let parsed_seed = parse_seed_string(&seed_string).unwrap();
        assert_eq!(original_seed, parsed_seed);
    }
}
