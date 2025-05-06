#!/usr/bin/env rust-script

#[derive(Clone, Debug, Eq, PartialEq)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct Sprite {
    name: String,
    position: Coord,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        // when there is a local variable with the same name as the
        // structure field then it is used if we omit the initializer
        Coord { x, y }
    }

    pub fn add(&self, other: &Coord) -> Coord {
        // the member access operator ('.') supports "deref coercion" meaning the compiler will
        // automatically dereference the l.h.s. until the types match. This means that, in this
        // case (*self).x and self.x are equivalent.
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn add_inplace(&mut self, other: &Coord) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() {
    let s = Sprite {
        name: "Example".to_string(),
        position: Coord { x: 320, y: 200 },
    };

    println!("{s:?}");
    println!("{} at {:?}", s.name, s.position);

    let c1 = Coord::new(3, 4);
    let mut c2 = Coord { x: 1, y: 10 }.add(&c1);
    c2.add_inplace(&c1);

    println!("c2 = {c2:?}");
}
