//#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod instruction;
mod state;
mod tests;

pinocchio_pubkey::declare_id!("E6UcK3dSFc2yaFtEb35pc1WsBVcrPhEbnB87YoNDXhqy");
