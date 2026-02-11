#![allow(unused)]

use touched::Touchable;

#[derive(Touchable)]
struct Normal {
    a: u32,
    b: u64,
}

#[derive(Touchable)]
struct WithGenric<T: Touchable, F: Touchable> {
    a: T,
    b: F,
}

#[derive(Touchable)]
struct WithSlice<'a> {
    a: &'a [u8],
    b: &'a mut [u8],
    c: [u8; 64],
}

#[derive(Touchable)]
struct WithSkipped<T> {
    a: u8,
    #[touched(skip)]
    b: core::marker::PhantomData<T>,
}

#[derive(Touchable)]
struct WithTuple {
    a: u8,
    b: (u8, u32),
    c: (),
}

#[derive(Touchable)]
struct EmptyStruct;

#[derive(Touchable)]
struct Unnamed(u32, u8);

#[derive(Touchable)]
struct UnnamedSkipped(u32, #[touched(skip)] core::marker::PhantomData<()>);
