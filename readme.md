# Heme

Heme is a Rust library for working with proteins


## Features 


### Read and write common formats for protein data 

Heme can read PDB files and construct a `Pose` object (named after 
the similar concept from Rosetta).

```rust 
use io::parse_pdb; 
use conformation::Pose; 

let atoms = parse_pdb(&contents);
let pose = Pose::from_atoms(atoms);
```

The `pose` object has some useful functions, aside from being a container 

- `Pose` object representing a protein structure
- `Atom` object that represents a single atom 
- both `Pose` and `Atom` have methods that are useful for calculating 
  structural features, such as the distance between two atoms, or the
  residues within some cutoff of a provided atom 


### Fast featurization for graph-structured generative models 

Heme is a fast way to featurize protein structures for use with
machine learning models 

```bash
heme featurize path/to/pdbs --output-format json --output-file dataset.json 
```


### Molecular modeling 

Heme includes an implementation of force-field based molecular modeling 
in the Rust language. It's a work in progress that's helping me 
learn Rust. I hope that it will eventually be useful, but right now
it's not. (For good implementations of force-field based molecular
modeling, see the OpenMM or Lumol projects.)


## Roadmap 

- Heme 0.4.0 (planned) 
    - implement fast featurization for structure transformer 
    - featurization for ProteinMPNN/LigandMPNN compatibility 
- Heme 0.3.2
    - handle ligands, metal ions, etc 
- Heme 0.2.0 
    - basic geometric operations 
    - basic force field implementation 
- Heme 0.1.0 
  - basic PDB parsing 
  - `Atom` and `Record` objects 
  

## Development 

Heme is a hybrid Python-Rust library with bindings provided by PyO3


