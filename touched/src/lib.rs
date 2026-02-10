#![no_std]

mod primitive;
mod slice;
mod util;

pub trait Touched {
    fn touch(&self);
}

impl Touched for usize {
    fn touch(&self) {
        let _ = core::hint::black_box::<usize>(*self);
    }
}

impl<T: Touched> Touched for &T {
    fn touch(&self) {
        touching::<T>(*self)
    }
}

pub fn touching<T: Touched + ?Sized>(t: &T) {
    <T as Touched>::touch(t);
}
