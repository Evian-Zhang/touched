mod primitive;
mod util;

pub trait Touched {
    fn touch(&self);
}

pub fn touching<T: Touched>(t: T) {
    t.touch();
}
