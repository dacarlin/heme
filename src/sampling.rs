// Sampling (smaller to larger, Move > Protocol)

use scoring::score;
use conformation::{Atom, Pose};

pub struct Move {
    pub name: String,
}

impl Move {
    pub fn new() -> Move {
        let name =  String::from("dummy move");
        Move { name }
    }
    pub fn apply(&self, pose: &Pose) {
        println!("Applied move {}", self.name)
    }
}

pub struct Protocol {
    pub name: String,
    pub steps: Vec<Move>,
}

impl Protocol {
    pub fn apply(&self, pose: Pose) -> f64 {
        for step in &self.steps {
            step.apply(&pose);
            let local_score = score(&pose);
        }
        let total_score = score(&pose);

        total_score
    }
}

pub fn get_protocol(protocol_name: &str) -> Protocol {
    let name = String::new();
    let steps = vec![Move::new(), Move::new()];
    Protocol { name, steps }
}
