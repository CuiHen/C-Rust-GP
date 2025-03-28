use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::utils::runner::ProgramState;

pub fn load_program_state_bool(path: &str) -> ProgramState<bool> {
    let file = File::open(Path::new(&path));
    let mut buf = vec![];
    file.unwrap().read_to_end(&mut buf).expect("TODO: panic message");
    
    serde_json::from_slice(&buf[..]).unwrap()
}

pub fn save_program_state_bool(path: &str, p_state: ProgramState<bool>) {
    let mut f = File::create(Path::new(&path)).unwrap();
    let buf = serde_json::to_vec(&p_state).unwrap();
    f.write_all(&buf[..]).unwrap();
    println!("Saving Successful");
}

pub fn load_program_state_f32(path: &str) -> ProgramState<f32> {
    let file = File::open(Path::new(&path));
    let mut buf = vec![];
    file.unwrap().read_to_end(&mut buf).expect("TODO: panic message");
    
    serde_json::from_slice(&buf[..]).unwrap()
}

pub fn save_program_state_f32(path: &str, p_state: ProgramState<f32>) {
    let mut f = File::create(Path::new(&path)).unwrap();
    let buf = serde_json::to_vec(&p_state).unwrap();
    f.write_all(&buf[..]).unwrap();
    println!("Saving Successful");
}