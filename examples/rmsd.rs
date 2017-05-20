// File rmsd.rs, example for the chemfiles library
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
extern crate chemfiles;
use chemfiles::{Trajectory, Frame};

fn main() {
    let mut trajectory = Trajectory::open("filename.nc", 'r').unwrap();
    let mut frame = Frame::new().unwrap();
    let mut distances = Vec::new();

    // Accumulate the distances to the origin of the 10th atom throughtout the
    // trajectory
    for _ in 0..trajectory.nsteps().unwrap() {
        trajectory.read(&mut frame).unwrap();
        // Position of the 10th atom
        let position = frame.positions().unwrap()[9];
        let distance = f64::sqrt(
            position[0] * position[0] +
            position[1] * position[1] +
            position[2] * position[2]
        );
        distances.push(distance);
    }

    let mean = distances.iter().fold(0.0, |acc, &item| acc + item);
    let mut rmsd = 0.0;

    for distance in &distances {
        rmsd += (mean - distance) * (mean - distance);
    }
    rmsd /= distances.len() as f64;
    rmsd = f64::sqrt(rmsd);

    println!("Root-mean square displacement is: {}", rmsd);
}
