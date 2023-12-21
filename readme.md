# Heme

Heme is a Rust library for working with proteins. Heme handles two 
major aspects of proteins: their 3-D structure and their 1-D primary
sequences. Use Heme for fast featurization of protein structures 
for use with machine learning models 



## Features 


### Read and write common formats for protein data 

Heme can read FASTAs and PDB files 


### Fast featurization for transformer models 

Heme is a fast way to featurize protein structures for use with
machine learning models 


### Molecular modeling 

Heme includes an implementation of force-field based molecular modeling 
in the Rust language. It's a work in progress that's helping me 
learn Rust. I hope that it will eventually be useful, but right now
it's not. (For good implementations of force-field based molecular
modeling, see the OpenMM or Lumol projects.)


## Features 

Heme is centered around proteins, so right now 

- parse PDB structures 
- `Pose` object representing a protein structure
- `Atom` object that represents a single atom 
- both `Pose` and `Atom` have methods that are useful for calculating 
  structural features, such as the distance between two atoms, or the
  residues within some cutoff of a provided atom 


## Roadmap 

- I intend to keep Heme a simple library for reading PDBs and working with
  protein structures as I continue to learn Rust 
- Simple molecular modeling is possible, it would be interesting to learn
  about implementing a force field
- I'd like to add some nice features like 
    - get the sequence of a `Pose` object 
    - a `Residue` object 




