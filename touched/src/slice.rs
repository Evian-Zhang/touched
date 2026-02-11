use crate::Touchable;

impl<T: Touchable> Touchable for [T] {
    fn touch(&self) {
        self.iter().for_each(crate::touching);
    }
}

impl<T: Touchable, const N: usize> Touchable for [T; N] {
    fn touch(&self) {
        crate::touching(self.as_slice());
    }
}

impl Touchable for str {
    fn touch(&self) {
        crate::touching(self.as_bytes());
    }
}

impl Touchable for core::ffi::CStr {
    fn touch(&self) {
        crate::touching(self.to_bytes_with_nul());
    }
}
