// This time, and this one time *only*
// we are going to implement a PDB parser.
// Literally as soon as we have Internet, we
// will look for an existing PDB parser.
// I promise. -- Alex (Sun, Jul 30 at 7:30 PM)

use conformation::{Atom, Pose};
use sampling::{Move, Protocol, get_protocol};
use scoring::score;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_atom_from_string() {
        let atom = Atom::new("ATOM      7  OD1 ASP A   1      27.457 -10.165  18.817  1.00  0.00           O");
        let xyz = XYZ::new(27.457, -10.165, 18.817);
        println!("{:?}", xyz)
        // assert_eq!()
    }

    #[test]
    fn test_pdb_parsing() {
        let contents = "ATOM      1  N   ASP A   1      27.405  -7.086  18.389  1.00  0.00           N
ATOM      2  CA  ASP A   1      26.221  -7.728  18.989  1.00  0.00           C
ATOM      3  C   ASP A   1      25.150  -6.679  19.328  1.00  0.00           C
ATOM      4  O   ASP A   1      24.099  -6.710  18.722  1.00  0.00           O
ATOM      5  CB  ASP A   1      26.607  -8.504  20.250  1.00  0.00           C
ATOM      6  CG  ASP A   1      27.438  -9.745  19.951  1.00  0.00           C
ATOM      7  OD1 ASP A   1      27.457 -10.165  18.817  1.00  0.00           O
ATOM      8  OD2 ASP A   1      28.042 -10.262  20.858  1.00  0.00           O
ATOM      9 1H   ASP A   1      28.091  -7.781  18.175  1.00  0.00           H
ATOM     10 2H   ASP A   1      27.138  -6.611  17.550  1.00  0.00           H
";
        let atoms = parse_pdb(&contents);
        let pose = Pose::from_atoms(atoms);

    }

}
