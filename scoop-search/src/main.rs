use serde_json::{Result, Value};
use std::{env, fs, path};

#[derive(Debug)]
struct Manifest {
    name: String,
    version: String,
    source: String,
    binaries: String,
}

fn main() -> Result<()> {
    let mut _name_count: u8 = 45;
    let mut _version_count: u8 = 20;
    let mut _source_count: u8  = 20;
    let mut _binaries_count: u8 = 25;
    let mut v: Vec<Manifest> = vec![];
    let query = "sumo".to_string().to_lowercase();
    // let query: String = std::env::args().nth(1).unwrap();
    // let query: String = "sumo".to_string();

    v.push(Manifest {
        name: "Name".to_string(), 
        version: "Version".to_string(), 
        source: "Source".to_string(), 
        binaries: "Binaries".to_string()
    });

    v.push(Manifest {
        name: "----".to_string(), 
        version: "-------".to_string(), 
        source: "------".to_string(), 
        binaries: "--------".to_string()
    });

    search_query(&mut v, &query).unwrap();

    for m in &v {
        println!("{: <45} {: <20} {: <20} {: <25}", m.name, m.version, m.source, m.binaries);
    }

    // println!("{}", query);

    Ok(())
}

fn search_query(v: &mut Vec<Manifest>, query: &str) -> Result<()>{
    let input_path: String = "C:/Users/Adrian/scoop/buckets/Scoop-Apps/bucket/SUMo-Portable.json".to_string();
    let text = fs::read_to_string(&input_path).unwrap();
    let val: Value = serde_json::from_str(&text).unwrap();

    let file_stem = path::Path::new(&input_path).file_stem().unwrap().to_str();
    let file_name = &file_stem.expect("no file stem found").to_string();
    // println!("{}", &file_stem.expect("no file stem found").to_string());
    if query != "" {
        if file_name.to_lowercase().contains(query) || val["bin"].to_string().to_lowercase().contains(query) {
            v.push(Manifest {
                name: path::Path::new(&input_path).file_stem().unwrap().to_str().expect("no file stem found").to_string(), 
                version: val["version"].to_string().replace("\"", ""), 
                source: val["source"].to_string(), 
                binaries: val["bin"].to_string().replace("\"", ""),
            });
        }
    } else {
        v.push(Manifest {
            name: path::Path::new(&input_path).file_stem().unwrap().to_str().expect("no file stem found").to_string(), 
            version: val["version"].to_string().replace("\"", ""), 
            source: val["source"].to_string(), 
            binaries: "".to_string(),
        });
    }
    Ok(())
}