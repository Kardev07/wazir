use serde_json::{from_str, Value};

fn read_index() -> Result<Value, serde_json::Error> {
    let temp = std::fs::read_to_string("index.json").unwrap();
    Ok(from_str(temp.as_str()).unwrap())
}

async fn install(value: &Value, name: &str) {
    println!(r#"Installing  "{}"..."#, name);
    let request = reqwest::get(
        value[name]["latest"]
            .as_str()
            .expect("Error: Not a string."),
    )
    .await
    .expect("An error occured");

    if value[name]["sa"].as_bool().unwrap() {
        let out_dir = std::fs::read_dir(".")
            .expect("Couldn't read directory")
            .find_map(|(v)| {
                if v.unwrap().file_name().into_string().unwrap().as_str() == "bin" {
                    Some(true)
                } else {
                    None
                }
            });
        match out_dir {
            None => {
                let new_dir = std::fs::create_dir("bin").ok();
            }
            Some(_) => {
                let file_type = value[name].get("type");

                let ext = match file_type {
                    None => "",
                    Some(t) => t.as_str().unwrap(),
                }
                .to_string();

                std::fs::write(
                    format!(
                        "bin/{}{}",
                        name,
                        if ext == "" { ext } else { format!(".{}", ext) }
                    )
                    .as_str(),
                    request
                        .bytes()
                        .await
                        .expect("Error in unwrapping the bytes"),
                );
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let index: Value = read_index().unwrap();
    let argv = std::env::args().collect::<Vec<String>>();

    if argv.len() == 1 {
        // TODO: Print help.
        return;
    }

    match argv[1].as_str() {
        "install" => {
            if argv.len() > 2 {
                install(&index, argv[2].as_str()).await;
            }
        }
        _ => {}
    }

    // println!("{}", index[argv[1].as_str()]);
}
