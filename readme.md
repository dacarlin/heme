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

```bash
heme featurize path/to/pdbs --output-format json --output-file dataset.json 
```


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

- Heme 0.3.2
- Heme 0.2.0 
- Heme 0.1.0 
  - basic PDB parsing 
  - `Atom` and `Record` objects 
  




