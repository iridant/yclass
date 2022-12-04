//! This example exists for the purpose of testing the main application.

use std::{collections::LinkedList, iter::repeat_with, thread::park};

#[repr(C)]
struct Foo {
    values: [u16; 10],
    p1: Box<[u32; 10]>,
    p2: Box<[f32; 10]>,
    p3: LinkedList<u32>,
}

fn main() {
    let foo = Foo {
        values: repeat_with(|| fastrand::u16(..255))
            .take(10)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        p1: Box::new(
            repeat_with(|| fastrand::u32(..255))
                .take(10)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ),
        p2: Box::new(
            repeat_with(fastrand::f32)
                .take(10)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ),
        p3: LinkedList::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
    };
    println!("Address: {:p}", &foo);

    park();
}
