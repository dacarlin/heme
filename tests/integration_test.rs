extern crate trust;

#[test]
fn it_works() {
    assert_eq!(4, 4);
}

#[test]
fn test_main() {
    // let args = vec!("score", "demo/5_res_peptide.pdb");
    // let config = trust::io::Config::new([&args).unwrap();
    // trust::io::run(config);
}

// extern crate trust;
//
// use std::env;
// use std::process;
//
// use trust::io::Config;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let config = Config::new(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });
//
//     println!("Running protocol: {}", config.protocol);
//     println!("Input file name: {}", config.filename);
//
//     if let Err(e) = trust::io::run(config) {
//         println!("Application error: {}", e);
//         process::exit(1);
//     }
// }
