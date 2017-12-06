#![no_std]

pub enum RE<P, R, Rs> {
    Empty,
    Group(R),
    Satisfy(P),
    Concatenate(Rs),
    Alternate(Rs),
    Repeat {
        x: R,
        bounds: (u32, Option<u32>),
        greedy: bool,
    },
}
