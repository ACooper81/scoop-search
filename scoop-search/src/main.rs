use serde_json::{Result, Value};
use std::{fs, env, io};

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

    let args: Vec<String> = env::args().collect();
    let query: String;
    match args.len() {
        1 => query = "".to_string().to_lowercase(),
        2 => query = args[1].parse().unwrap(),
        _ => query = "".to_string().to_lowercase(),
    }

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

    let buckets_path = env::home_dir().unwrap().display().to_string() + "\\scoop\\buckets\\";
    let buckets = fs::read_dir(buckets_path).unwrap();

    for path in buckets {
        let manifest_path = path.unwrap().path().display().to_string() + "\\bucket";
        let manifest_paths = fs::read_dir(manifest_path).unwrap();
        for manifest in manifest_paths {
            let path = manifest.unwrap().path().display().to_string();
            // println!("Buckets: {}", path);
            search_query(&mut v, &path, &query).unwrap();
        }
    }

    // search_query(&mut v, &"C:/Users/Adrian/scoop/buckets/Scoop-Apps/bucket/SUMo-Portable.json".to_string(), &query).unwrap();
    print!("Results from local buckets...\n\n");
    for m in &v {
        println!("{: <45} {: <20} {: <20} {: <25}", m.name, m.version, m.source, m.binaries);
        // let stdin = io::stdin();
        // let mut user_input: String = "".to_string();
        // stdin.read_line(&mut user_input).unwrap();
    }

    // println!("{}", query);

    Ok(())
}

fn search_query(v: &mut Vec<Manifest>, input_path: &String, query: &str) -> Result<()>{
    // let input_path: String = "C:/Users/Adrian/scoop/buckets/Scoop-Apps/bucket/SUMo-Portable.json".to_string();
    let text = fs::read_to_string(&input_path).unwrap();
    let val: Value = serde_json::from_str(&text).unwrap();

    // let _path_split = input_path.split('/');
    let mut manifest_name = "".to_string();
    let mut bucket = "".to_string();

    let mut last_split_item: String = "".to_string();
    for substring in input_path.split("\\") {
        if last_split_item == "buckets" {
            bucket = substring.to_string();
        }
        if substring.contains(".json") {
            manifest_name = substring.replace(".json", "");
        }
        last_split_item = substring.to_string();
    }
    
    // let file_stem = path::Path::new(&input_path).file_stem().unwrap().to_str();
    // let file_name = &file_stem.expect("no file stem found").to_string();
    // println!("{}", &file_stem.expect("no file stem found").to_string());
    if query != "" {
        if manifest_name.to_lowercase().contains(query) || val["bin"].to_string().to_lowercase().contains(query) {
            v.push(Manifest {
                name: manifest_name,
                version: val["version"].to_string().replace("\"", ""), 
                source: bucket, 
                binaries: val["bin"].to_string().replace("\"", ""),
            });
        }
    } else {
        v.push(Manifest {
            name: manifest_name, 
            version: val["version"].to_string().replace("\"", ""), 
            source: bucket, 
            binaries: "".to_string(),
        });
    }
    Ok(())
}