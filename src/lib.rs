//! Raw FFI bindings to the Rockchip Rockit MPI SDK (RV1103/RV1106, Luckfox Pico).
//!
//! Bindings are pregenerated with bindgen 0.71 for
//! `armv7-unknown-linux-uclibceabihf` (32-bit ARM, hard-float).
//!
//! You must provide `librockit.so` from the Luckfox SDK at build time
//! (via `ROCKIT_LIB`) and at runtime (`LD_LIBRARY_PATH`).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

include!("bindings.rs");