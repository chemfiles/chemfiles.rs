/* File rmsd.rs, example for the Chemharp library
 * Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */
extern crate chemharp;
use chemharp::*;

fn main() {
    let mut traj = Trajectory::open("filename.nc").unwrap();
    let mut frame = Frame::new(0).unwrap();
    let mut distances = Vec::new();

    // Accumulate the distances to the origin of the 10th atom throughtout the
    // trajectory
    for _ in 0..traj.nsteps().unwrap() {
        traj.read(&mut frame).unwrap();
        // Position of the 10th atom
        let position = frame.positions().unwrap()[9];
        let distance = f32::sqrt(position[0]*position[0] +
                                 position[1]*position[1] +
                                 position[2]*position[2]);
        distances.push(distance);
    }

    let mean = distances.iter().fold(0.0, |acc, &item| acc + item);
    let mut rmsd = 0.0;

    for distance in distances.iter() {
        rmsd += (mean - distance) * (mean - distance);
    }
    rmsd /= distances.len() as f32;
    rmsd = f32::sqrt(rmsd);

    println!("Root-mean square displacement is: {}", rmsd);
}
