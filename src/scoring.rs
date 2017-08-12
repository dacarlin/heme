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

pub fn score(pose: &Pose) -> f64 {
    // Right now, we are only scoring atoms
    //let mut total_energy: f64 = 0.0;

    // electrostatics and van der Waals
    let mut coulomb_score: f64 = 0.0;
    let mut lennard_jones: f64 = 0.0;

    for i in &pose.atoms {
        for j in &pose.atoms {
            let r: f64 = i.dist(j);
            if r == 0.0 {
                // these are the same atom
                // do nothing
            } else if r.max(12.0) == 12.0 {
                // these atoms are greater than 12 Å apart
                // ignore that they interact
            } else {
                let epsilon: f64 = 1.0;
                let C: f64 = 1.0;
                let cou = i.charge * j.charge / r.powi(2);
                coulomb_score += C * cou;

                let rm: f64 = 1.0; // minimum distance is actually going to depend on both atoms' radii
                let lj = (rm/r).powi(12) - 2.0 * (rm/r).powi(6);
                lennard_jones += epsilon * lj;
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
