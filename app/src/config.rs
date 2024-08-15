use std::{collections::HashMap, io::Read};
use std::env;
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

pub struct Config {
    pub map: Yaml
}

impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];
        
        let mut config = String::new();
        let _ = std::fs::File::open(file_path).unwrap().read_to_string(&mut config);
        let map = YamlLoader::load_from_str(&config).unwrap();

        Config { map: map[0].clone() }
    }
}
