//! This modules contains all of medblock things regarding data security (encryption, decryption, vetkds).
//!
//! ### DISCLAIMER
//!
//!
//! ***THIS MODULES IS NOT YET IN ANYWAY HAS BEEN AUDITED FOR ITS SECURITY AND USAGE.
//! THIS MODULE CURRENTLY IS MEANT TO BE A PROTOTYPE OF USAGE ON HOW VETKDS MAYBE USED TO SECURE
//! PATIENT EMR.***
//!
//! ***THIS MODULE DOES NOT CONCERN WITH THE HANDLING OF ACCESS CONTROL.***
//!
//! ***THIS MODULE IS MEANT TO BE USED ONLY FOR PUBLIC API THAT CANISTER EXPOSES THAT DEALS WITH DATA ENCRYPTION AND DECRPYTION. 
//! NO OTHER MODULE INSIDE THIS CANISTER CRATES ARE ALLOWED TO USE ANYHTING FROM THIS MODULE!***
//!
//!
//! we intentionally seperate the handling of data security in an effort to make integration easy. And also because we expect this modules
//! to change frequently due to audits(later date) and potential adjustments for vetkds integration.
//! for now, this module only provide basic symmetric key like usage taken directly from icp default vetkd [example](https://github.com/dfinity/examples/blob/master/rust/vetkd/src/app_backend/src/lib.rs).
//! although we're planning to aim for public api similar to the example as it suits our usecase, we acknowledge that this implementation is unsecure and thus, require
//! further audits and adjustments.  
//!

/// vetkd abstraction api
pub mod vetkd;
