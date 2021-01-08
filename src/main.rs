use serde_json::{from_str, Value};
mod unzip;
use unzip::extract_zip;

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
        std::fs::create_dir("bin").ok();

        let file_type = value[name].get("type");

        let ext = match file_type {
            None => "",
            Some(t) => t.as_str().unwrap(),
        };

        std::fs::write(
            format!(
                "bin/{}{}",
                name,
                if ext == "" {
                    ext.to_string()
                } else {
                    format!(".{}", ext)
                }
            )
            .as_str(),
            request
                .bytes()
                .await
                .expect("Error in unwrapping the bytes"),
        )
        .expect("An error occuered while writing the file.");

        if ext == "zip" {
            extract_zip(format!("bin/{}.zip", name).as_str())
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
