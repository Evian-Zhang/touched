use crate::Touched;

impl<T: Touched> Touched for [T] {
    fn touch(&self) {
        self.iter().for_each(crate::touching);
    }
}

impl Touched for str {
    fn touch(&self) {
        crate::touching(self.as_bytes());
    }
}

impl Touched for core::ffi::CStr {
    fn touch(&self) {
        crate::touching(self.to_bytes_with_nul());
    }
}
