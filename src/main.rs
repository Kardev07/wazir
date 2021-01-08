use serde_json::{from_str, Value};

fn read_index() -> Result<Value, serde_json::Error> {
    let temp = std::fs::read_to_string("index.json").unwrap();
    Ok(from_str(temp.as_str()).unwrap())
}

fn install(name: &str) {
    // TODO: Installation Code
    // Pull the code from the respective URL, and extract if a zip file.
}

fn main() {
    let index: Value = read_index().unwrap();
    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 {
        // TODO: Print help.
        return;
    }
    match argv[1].as_str() {
        "install" => {
            if argv.len() > 2 {
                install(argv[2].as_str());
            }
        }
        _ => {}
    }

    println!("{}", index[argv[1].as_str()]);
}
