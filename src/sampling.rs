use crate::transforms::{score, pdb_info, featurize};
use crate::conformation::Pose;

pub fn get_protocol(protocol_name: &str) -> Protocol {
    // this can get a protocol from a dict of protocols defined in
    // protocols.rs. But, for now, we hard-code a protocol in

    println!("running protocol {}", protocol_name); 

    let protocol: Protocol = match protocol_name {
        "score" => Protocol {
            components: vec![
                // go ahread, try it, it's fun!
                Box::new(ScoreOnlyTransform {}),
                Box::new(ScoreOnlyTransform {}),
            ]
        },
        "featurize" => Protocol {
            components: vec![
                // go ahread, try it, it's fun!
                
                Box::new(PDBInfoTransform {}),
                Box::new(FeaturizeTransform {}),
                
            ]
        },
        _ => Protocol {
            components: vec![
                
                Box::new(PDBInfoTransform {}), 
            ]
        }
    }; 

    protocol
}

pub struct Protocol {
    pub components: Vec<Box<dyn Transform>>,
}

impl Protocol {
    pub fn run(&self, pose: &mut Pose) {
        for component in self.components.iter() {
            component.apply(pose);
        }
    }
}

pub trait Transform {
    fn apply(&self, pose: &mut Pose);
}

// Manifest: available Transformrs 
pub struct ScoreOnlyTransform {}
pub struct PDBInfoTransform {}
pub struct FeaturizeTransform {} 

// implementations for Transformrs
impl Transform for ScoreOnlyTransform {
    fn apply(&self, pose: &mut Pose) {
        // code to run when applying

        let result: f64 = score(pose);
        println!("ScoreOnlyTransform: result is {}", result);
    }
}

impl Transform for PDBInfoTransform {
    fn apply(&self, pose: &mut Pose) {
        let result: String = pdb_info(pose); 

        println!("{}", result)
    }
}

impl Transform for FeaturizeTransform {
    fn apply(&self, pose: &mut Pose) {
        let result: String = featurize(pose); 

        println!("{}", result)
    }
}
