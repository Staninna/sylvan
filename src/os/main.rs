#![no_std]
#![no_main]
// Needed for testing
#![feature(custom_test_frameworks)]
#![test_runner(sylvan_shared::test::test_runner)]
#![reexport_test_harness_main = "test"]

use arrayvec::ArrayVec;
use rand::{rngs::SmallRng, SeedableRng};
use sylvan_shared::{
    os::{
        init::init,
        rng::SEED,
        vga::{clear_screen, BUFFER_HEIGHT, BUFFER_WIDTH},
    },
    println,
    verlet::{
        blob::Blob,
        constraint::Constraint,
        solver::{Solver, MAX_BLOBS},
        vec2::Vec2,
    },
};

mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    println!("Type a seed for the random number generator must be a u64");
    for _ in 0..600_000_000 {
        x86_64::instructions::nop()
    }
    x86_64::instructions::interrupts::disable();

    let constraint = Constraint {
        position: Vec2::new(BUFFER_WIDTH as f32 / 2.0, BUFFER_HEIGHT as f32 / 2.0),
        radius: (BUFFER_HEIGHT * 5) as f32,
    };

    let mut rng = SmallRng::seed_from_u64(unsafe { SEED });
    let mut blobs: ArrayVec<Blob, MAX_BLOBS> = ArrayVec::new();
    for _ in 0..MAX_BLOBS {
        let blob = Blob::random(&mut rng, constraint);
        blobs.push(blob);
    }

    let gravity = Vec2::new(0.0, 9.81);

    let iterations = 1;
    let time = 0.00000000001;

    let mut solver = Solver::new(blobs, gravity, constraint, iterations, time);

    loop {
        solver.update();
        clear_screen(&constraint);
        solver.draw();

        for _ in 0..5_000_000 {
            x86_64::instructions::nop()
        }
    }
}
