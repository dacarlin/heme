use scoring::score;
use conformation::Pose;

// Sampling (smaller to larger, Move > Protocol)

pub fn get_protocol(protocol_name: &str) -> Protocol {
    // this can get a protocol from a dict of protocols defined in
    // protocols.rs. But, for now, we hard-code a protocol in
    let protocol = Protocol {
        components: vec![
            Box::new(ScoreOnlyMove {x: 1}),
            // Box::new(ScoreOnlyMove {x: 1}),
        ]
    };

    protocol
}

pub trait Move {
    fn apply(&self, pose: &mut Pose);
}

pub struct Protocol {
    pub components: Vec<Box<Move>>,
}

impl Protocol {
    pub fn run(&self, pose: &mut Pose) {
        for component in self.components.iter() {
            component.apply(pose);
        }
    }
}

pub struct ScoreOnlyMove {
    pub x: u32,
}

impl Move for ScoreOnlyMove {
    fn apply(&self, pose: &mut Pose) {
        // code to run when applying

        let result: f64 = score(pose);
        println!("ScoreOnlyMove: result is {}", result);
    }
}
