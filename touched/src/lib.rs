#![no_std]

mod primitive;
mod slice;
mod util;

#[cfg(feature = "derive")]
pub use touched_derive::Touchable;

pub trait Touchable {
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

pub fn touching<T: Touchable + ?Sized>(t: &T) {
    <T as Touchable>::touch(t);
}
