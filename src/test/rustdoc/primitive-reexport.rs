// aux-build:primitive.rs

extern crate primitive;

pub mod bar {
    #[doc(no_inline)]
    pub use primitive::*;
}
