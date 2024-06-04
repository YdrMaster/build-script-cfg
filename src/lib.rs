#![doc = include_str!("../README.md")]
#![deny(warnings, missing_docs)]

/// The type that represents a configuration option.
#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Cfg<S>(S);

impl<S: std::fmt::Display> Cfg<S> {
    /// Creates a new configuration option with the given name.
    #[inline]
    pub fn new(name: S) -> Self {
        println!("cargo::rustc-check-cfg=cfg({name})");
        Self(name)
    }

    /// Sets the corresponding `rustc-cfg` flag.
    #[inline]
    pub fn define(self) {
        println!("cargo:rustc-cfg={}", self.0);
    }
}
