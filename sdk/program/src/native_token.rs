//! Definitions for the native TRZ token and its fractional lamports.

#![allow(clippy::arithmetic_side_effects)]

/// There are 10^9 lamports in one TRZ
pub const LAMPORTS_PER_TRZ: u64 = 1_000_000_000;

/// Approximately convert fractional native tokens (lamports) into native tokens (TRZ)
pub fn lamports_to_trz(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_TRZ as f64
}

/// Approximately convert native tokens (TRZ) into fractional native tokens (lamports)
pub fn trz_to_lamports(trz: f64) -> u64 {
    (trz * LAMPORTS_PER_TRZ as f64) as u64
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct Trz(pub u64);

impl Trz {
    fn write_in_trz(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "◎{}.{:09}",
            self.0 / LAMPORTS_PER_TRZ,
            self.0 % LAMPORTS_PER_TRZ
        )
    }
}

impl Display for Trz {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_trz(f)
    }
}

impl Debug for Trz {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_trz(f)
    }
}
