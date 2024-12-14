/// This module contains the implementation of the Enhanced Surveillance (EHS) decoder.
/// The EHS decoder consists of several sub-modules, each responsible for decoding a specific version of the EHS data.
mod base;
mod bds_4_0;
mod bds_5_0;
mod bds_6_0;

pub(crate) use base::*;
pub(crate) use bds_4_0::*;
pub(crate) use bds_5_0::*;
pub(crate) use bds_6_0::*;
