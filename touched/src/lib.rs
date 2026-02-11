#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod primitive;
mod slice;
mod tuple;
mod util;

#[cfg(feature = "derive")]
pub use touched_derive::Touchable;

/// Trait for accessing each memory unit of a target.
///
/// You can use `#[derive(Touchable)]` for common structures.
pub trait Touchable {
    /// Take the reference of a target, access its each memory unit
    fn touch(&self);
}

impl<T: Touchable + ?Sized> Touchable for &T {
    fn touch(&self) {
        touching::<T>(*self)
    }
}

impl<T: Touchable + ?Sized> Touchable for &mut T {
    fn touch(&self) {
        touching::<T>(*self)
    }
}

/// Access a target's each memory unit.
///
/// The target should implement [`Touchable`].
pub fn touching<T: Touchable + ?Sized>(t: &T) {
    <T as Touchable>::touch(t);
}
