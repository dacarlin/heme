/*

Kinds of energies for atoms:

- two body
    - angle
    - dist
    - torsion
    - electrostatic
    - Lennard-Jones

Kinds of energies for residues:

- one body
    - depends only on amino acid
    - depends only on phi and psi angle
- two body

*/

use conformation::{Atom, Pose};

pub struct ScoreType {
    pub cutoff: i32,
    pub name: String,
    // evaluation_function: T,
}

impl ScoreType {
    pub fn from_name(name: &str) -> ScoreType {
        let name = String::from("lj");
        let cutoff = 6;

        ScoreType { cutoff, name }
    }

    pub fn score_atoms(atom1: &Atom, atom2: &Atom) -> f64 {
        // score a pair of atoms and return the total score

        55.5
    }
}



pub fn score(pose: &Pose) -> f64 {

    let mut total_score: f64 = 0.0;
    // Right now, we are only scoring atoms
    //let mut total_energy: f64 = 0.0;

    // electrostatics and van der Waals
    let mut coulomb_score: f64 = 0.0;
    let mut lennard_jones: f64 = 0.0;

    // calculate Lennard-Jones energy
    // from two atoms and the distance between them
    // which has already been computed
    pub fn calculate_lennard_jones_energy(a1: &Atom, a2: &Atom, r: f64) -> f64 {

        let r_m = 1.0; // sum of both radii
        let epsilon = 1.0;

        epsilon * ((r_m/r).powi(12) - 2.0 * (r_m/r).powi(6))
    }

    pub fn calculate_coulomb_energy(a1: &Atom, a2: &Atom, r: f64) -> f64 {

        let c = 1.0;

        c * a1.charge * a2.charge / r.powi(2)
    }

    for i in &pose.atoms {
        for j in &pose.atoms {
            let r: f64 = i.dist(j);
            if r == 0.0 {
                // these are the same atom
                // do nothing
            } else if r.max(6.0) == 6.0 {
                // these atoms are greater than 12 Å apart
                // ignore that they interact
            } else {
                total_score += calculate_lennard_jones_energy(i, j, r);
                total_score += calculate_coulomb_energy(i, j, r);
            }
        }
    }

    // solvation
    let mut solvation: f64 = 0.0;
    for i in &pose.atoms {
        let mut count_of_neighbors: f64 = 0.0;
        // count number of neighbors
        // penalize based on charge and number of neighbors
        for j in &pose.atoms {
            let r = i.dist(&j);
            if r.max(6.0) == 6.0 {
                // these atoms are greater than 6 Å apart
            } else {
                count_of_neighbors += 1.0;
            }
        }

        solvation += count_of_neighbors * i.charge;
    }

    (0.5 * coulomb_score) + (1.0 * lennard_jones) + (0.6 * solvation)
    // whoo, hard-coded weights!
}
