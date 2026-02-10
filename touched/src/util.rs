macro_rules! impl_for_primitives {
    ($ty:ty) => {
        impl $crate::Touched for $ty {
            fn touch(&self) {
                let _ = core::hint::black_box::<$ty>(*self);
            }
        }
    };
}

pub(crate) use impl_for_primitives;
