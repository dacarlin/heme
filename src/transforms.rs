use conformation::{Atom, Pose};

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

pub fn score(pose: &Pose) -> f64 {

    let lennard_jones_weight: f64 = 1.0;
    let coulomb_weight: f64 = 1.0;

    let mut total_score: f64 = 0.0;
    for i in &pose.atoms {
        for j in &pose.atoms {
            let r: f64 = i.dist(j);
            if r == 0.0 {
                // these are the same atom // do nothing
            } else if r.max(6.0) == 6.0 {
                // these atoms are greater than 12 Å apart, ignore that they interact
            } else {
                total_score += lennard_jones_weight * calculate_lennard_jones_energy(i, j, r);
                total_score += coulomb_weight * calculate_coulomb_energy(i, j, r);
            }
        }
    }

    // It would be better to have a NxN matrix, where N = number of atoms
    // where each entry is the last known distance, and to only operate on
    // a subset of that matrix

    total_score
}

pub fn pdb_info(pose: &Pose) -> String {

    String::from("hello")
}

pub fn featurize(pose: &Pose) -> String {

    let mut count = 0;
    for i in &pose.atoms {
        count += 1; 
    }

    String::from(format!("PDB info\n----------\nNumber of atoms: {}\nNumber of snatoms:{}", count, count))
}
