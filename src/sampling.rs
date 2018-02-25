use scoring::score;
use conformation::Pose;

// Sampling (smaller to larger, Move > Protocol)

// available_protocols?

pub fn get_protocol(protocol_name: &str) -> Protocol {
    // this can get a protocol from a dict of protocols defined in
    // protocols.rs. But, for now, we hard-code a protocol in
    let protocol = Protocol {
        components: vec![
            // go ahread, try it, it's fun!
            Box::new(ScoreOnlyMove {}),
            Box::new(GridMove {}),
            Box::new(ScoreOnlyMove {}),
        ]
    };

    protocol
}

// protocol object
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

// mover trait
pub trait Move {
    fn apply(&self, pose: &mut Pose);
}

// list of movers
pub struct ScoreOnlyMove {}
pub struct GridMove {}

// implementations for movers
impl Move for ScoreOnlyMove {
    fn apply(&self, pose: &mut Pose) {
        // code to run when applying

        let result: f64 = score(pose);
        println!("ScoreOnlyMove: result is {}", result);
    }
}

impl Move for GridMove {
    fn apply(&self, pose: &mut Pose) {
        // get extents of the active site
        // or can read in from a argument the 6 XYZ objects at the corners

        // construct the grid
        // this will be like the Submaranian paper
        // we will evaluate hydro and electrostatic terms on each grid point
        // this makes it sequence-independent (and structure-independent)
        // just like our PLOS paper 

        println!("GridMove: activating grid")

    }
}
