use serde_json::{Result, Value};
use std::{fs, env};
extern crate termsize;


#[derive(Debug)]
struct Manifest {
    name: String,
    version: String,
    source: String,
    binaries: String,
}

fn main() -> Result<()> {
    let mut name_count: usize = 4;
    let mut version_count: usize = 7;
    let mut source_count: usize  = 6;
    let mut binaries_count: usize = 8;
    let mut terminal_width: usize = 0;
    let mut v: Vec<Manifest> = vec![];

    termsize::get().map(|size| {
        // println!("rows {} cols {}", size.rows, size.cols);
        terminal_width = size.cols as usize;
    });
    

    let args: Vec<String> = env::args().collect();
    let query: String;

    let help_message = "Usage: scoop search <query>\n\nSearches for apps that are available to install.\n\nIf used with [query], shows app names that match the query.\nWithout [query], shows all the available apps.";
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" || args[1] == "/?" {
            println!("{help_message}");
            std::process::exit(exitcode::OK);
        }
    }
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
    // let buckets_path = env::home_dir().unwrap().display().to_string() + "\\scoop\\buckets\\";
    let buckets_path = env::var("SCOOP").unwrap_or(env::home_dir().unwrap().display().to_string() + "\\scoop") + "\\buckets\\";
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

    for m in &mut v {
        if m.name.len() > name_count {
            name_count = m.name.len();
        }
        if m.version.len() > version_count {
            version_count = m.version.len();
        }
        if m.source.len() > source_count {
            source_count = m.source.len();
        }
        if m.binaries.len() > binaries_count {
            let widths = name_count + version_count + source_count + 30;
            binaries_count = terminal_width - widths;
        }
        // println!("{} {}",terminal_width, binaries_count);
    }

    print!("Results from local buckets...\n\n");
    for m in &mut v {
        if m.binaries.len() > binaries_count {
            let string: String = m.binaries.chars().take(binaries_count - 3).collect();
            m.binaries = string + "...";
        }
        println!("{: <width$} {: <width2$} {: <width3$} {: <width4$}"
        , m.name, m.version, m.source, m.binaries
        , width = name_count, width2 = version_count, width3 = source_count, width4 = binaries_count);
    }

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

    let version_string = val["version"].to_string().replace("\"", "");
    if query != "" {
        if manifest_name.to_lowercase().contains(query) || val["bin"].to_string().to_lowercase().contains(query) {
            let mut binaries_string = val["bin"].to_string().replace("\"", "");
            if binaries_string == "null".to_string() {
                binaries_string = "".to_string();
            }
            if binaries_string.contains("[") {
                binaries_string = binaries_string.replace("[", "").replace("]", "").replace(",", " | ");
            }
            
            // binaries_string = "".to_string();
            // for item in binaries_list {
            //     if item.contains('.') {
                    
            //         binaries_string = binaries_string + item;
            //     }
            // }
            v.push(Manifest {
                name: manifest_name,
                version: version_string,
                source: bucket, 
                binaries: binaries_string,
            });
        }
    } else {
        v.push(Manifest {
            name: manifest_name, 
            version: version_string, 
            source: bucket, 
            binaries: "".to_string(),
        });
    }
    Ok(())
}