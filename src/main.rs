extern crate uom;

use uom::si::f32::*;
use uom::si::length::kilometer;
use uom::si::time::second;

fn main() {
    let length = Length::new::<kilometer>(5.0);
    let time = Time::new::<second>(15.0);
    let velocity = speed(length, time);
    println!("Average speed was: {:?}", velocity);
}

fn speed(length: Length, time: Time) -> Velocity {
    // Explicit type on return value here as the inferred type differs from Velocity
    let v: Velocity = length / time;
    v
}
