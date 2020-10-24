use std::marker::PhantomData;

pub enum Literal {
    Bln(Bln),
    Str(Str),
    Num(Num),
    Emp(Emp),
}

// TODO: Implement `Str` and `Num` properly

pub struct Bln(bool);

pub struct Str(String);

pub struct Num(f64);

pub struct Emp(PhantomData<()>);
