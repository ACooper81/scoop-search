use serde_json::{Result, Value};

fn search_query(v: &mut Vec<Manifest>, query: &str) -> Result<()>{
    let input_path: String = "C:\\Users\\Adrian\\scoop\\buckets\\Scoop-Apps\\bucket\\SUMo-Portable.json".to_string();
    let text = std::fs::read_to_string(&input_path).unwrap();
    let val: Value = serde_json::from_str(&text).unwrap();

    if query == "" {
        
    } else {
        v.push(Manifest {
            name: val["name"].to_string(), 
            version: val["version"].to_string().replace("\"", ""), 
            source: val["source"].to_string(), 
            binaries: val["bin"].to_string().replace("\"", ""),
        });
    }
    Ok(())
}

#[derive(Debug)]
struct Manifest {
    name: String,
    version: String,
    source: String,
    binaries: String,
}

fn main() {
    let mut _name_count: u8 = 45;
    let mut _version_count: u8 = 20;
    let mut _source_count: u8  = 20;
    let mut _binaries_count: u8 = 25;
    let mut v: Vec<Manifest> = vec![];
    let query: String = std::env::args().nth(0).unwrap();

    
    // let input_path: String = "C:\\Users\\Adrian\\scoop\\buckets\\Scoop-Apps\\bucket\\SUMo-Portable.json".to_string();
    // let text = std::fs::read_to_string(&input_path).unwrap();
    // let val: Value = serde_json::from_str(&text).unwrap();

    // if query == "" {
        
    // } else {
    //     v.push(Manifest {
    //         name: val["name"].to_string(), 
    //         version: val["version"].to_string().replace("\"", ""), 
    //         source: val["source"].to_string(), 
    //         binaries: val["bin"].to_string().replace("\"", ""),
    //     });
    // }

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
    
    // untyped_example().unwrap();
}
