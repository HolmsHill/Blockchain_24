mod macros;

mod ethereum;
pub use ethereum::EthereumHardfork;

mod optimism;
pub use optimism::OptimismHardfork;

mod dev;
pub use dev::DEV_HARDFORKS;

use core::{
    any::Any,
    hash::{Hash, Hasher},
};
use dyn_clone::DynClone;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

/// Generic hardfork trait.
/// ///
/// This trait defines common behavior for hardforks in various blockchains or similar systems.
/// It provides methods to retrieve the name of the hardfork and supports cloning and hashing.
#[auto_impl::auto_impl(&, Box)]
pub trait Hardfork: Any + DynClone + Send + Sync + 'static {
    /// Fork name.
    fn name(&self) -> &'static str;
}

dyn_clone::clone_trait_object!(Hardfork);

impl core::fmt::Debug for dyn Hardfork + 'static {
    /// Implements the `Debug` trait for `dyn Hardfork`.
    ///
    /// Formats the debug output as the name of the hardfork.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(self.name()).finish()
    }
}

impl PartialEq for dyn Hardfork + 'static {
    // Implements the `PartialEq` trait for `dyn Hardfork`.
    ///
    /// Compares two hardforks based on their names.
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for dyn Hardfork + 'static {}

impl Hash for dyn Hardfork + 'static {
    /// Implements the `Hash` trait for `dyn Hardfork`.
    ///
    /// Hashes the hardfork based on its name.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardfork::optimism::OptimismHardfork;
    use std::str::FromStr;

    #[test]
    fn check_hardfork_from_str() {

        /// Test data for Ethereum hardforks
        let hardfork_str = [
            "frOntier",
            "homEstead",
            "dao",
            "tAngerIne",
            "spurIousdrAgon",
            "byzAntium",
            "constantinople",
            "petersburg",
            "istanbul",
            "muirglacier",
            "bErlin",
            "lonDon",
            "arrowglacier",
            "grayglacier",
            "PARIS",
            "ShAnGhAI",
            "CaNcUn",
            "PrAguE",
        ];
        let expected_hardforks = [
            EthereumHardfork::Frontier,
            EthereumHardfork::Homestead,
            EthereumHardfork::Dao,
            EthereumHardfork::Tangerine,
            EthereumHardfork::SpuriousDragon,
            EthereumHardfork::Byzantium,
            EthereumHardfork::Constantinople,
            EthereumHardfork::Petersburg,
            EthereumHardfork::Istanbul,
            EthereumHardfork::MuirGlacier,
            EthereumHardfork::Berlin,
            EthereumHardfork::London,
            EthereumHardfork::ArrowGlacier,
            EthereumHardfork::GrayGlacier,
            EthereumHardfork::Paris,
            EthereumHardfork::Shanghai,
            EthereumHardfork::Cancun,
            EthereumHardfork::Prague,
        ];

        /// Parse strings into EthereumHardfork variants
        let hardforks: Vec<EthereumHardfork> =
            hardfork_str.iter().map(|h| EthereumHardfork::from_str(h).unwrap()).collect();

        /// Assert that parsed hardforks match the expected variants
        assert_eq!(hardforks, expected_hardforks);
    }

    #[test]
    fn check_op_hardfork_from_str() {
        /// Test data for Optimism hardforks
        let hardfork_str = ["beDrOck", "rEgOlITH", "cAnYoN", "eCoToNe", "FJorD"];
        let expected_hardforks = [
            OptimismHardfork::Bedrock,
            OptimismHardfork::Regolith,
            OptimismHardfork::Canyon,
            OptimismHardfork::Ecotone,
            OptimismHardfork::Fjord,
        ];

        /// Parse strings into OptimismHardfork variants
        let hardforks: Vec<OptimismHardfork> =
            hardfork_str.iter().map(|h| OptimismHardfork::from_str(h).unwrap()).collect();

        /// Assert that parsed hardforks match the expected variants
        assert_eq!(hardforks, expected_hardforks);
    }

    #[test]
    fn check_nonexistent_hardfork_from_str() {
        /// Test for a non-existent hardfork name
        assert!(EthereumHardfork::from_str("not a hardfork").is_err());
    }
}
