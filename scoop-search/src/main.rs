use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
    {
        "version": "5.17.2.534",
        "description": "SUMo. Detecting and updating outdated software",
        "homepage": "https://www.kcsoftwares.com/?sumo",
        "license": {
            "identifier": "Freeware",
            "url": "http://www.kcsoftwares.com/legal/ToU.pdf"
        },
        "url": "https://www.kcsoftwares.com/files/sumo.zip",
        "hash": "5c1768c9a1b9d5b6ade6a370895c46a2c79b3feab025b57ad586d838150f7306",
        "extract_dir": "sumo",
        "pre_install": "if (!(Test-Path \"$persist_dir\\settings.ini\")) { Set-Content \"$dir\\settings.ini\" '[Settings]', 'AutoUpdate=0' -Encoding Ascii }",
        "bin": "SUMo.exe",
        "shortcuts": [
            [
                "SUMo.exe",
                "SUMo"
            ]
        ],
        "persist": "settings.ini",
        "checkver": {
            "url": "https://www.kcsoftwares.com/?download",
            "regex": "(?sm)SUMo</h.*?Version\\s+:\\s*<code>([\\d.]+)</"
        },
        "autoupdate": {
            "url": "https://www.kcsoftwares.com/files/sumo.zip"
        }
    }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Version: {}\nDescription: {}\nLicense: {}\nShortcuts: {}", v["version"], v["description"], v["license"]["identifier"], v["shortcuts"][0]);

    Ok(())
}

fn print_if_contains_query(name: &str, bucket: &str, json: &str, query: &str) -> Result<()>{
    let v: Value = serde_json::from_str(json)?;
    if query != "" && v["description"].to_string().contains(query) {
        println!("{}\t{}\t{}", name, v["version"], bucket)
    }
    Ok(())
}

fn main() {
    untyped_example().unwrap();
}
