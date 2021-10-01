extern crate clap;
use clap::{crate_version, App, Arg};
use serde_json::{from_str, Value};
mod unzip;
use unzip::extract_zip;

fn read_index() -> Result<Value, serde_json::Error> {
    let wazir_dir = std::env::var("WAZIR_DIR");

    let temp = if wazir_dir.is_ok() {
        let wazir_dir = wazir_dir.unwrap();
        std::fs::read_to_string(wazir_dir + "/index.json").unwrap()
    } else {
        panic!("$WAZIR_DIR is not defined");
    };
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

    let app = App::new("Wazir")
        .author("Rishit Khandelwal <github.com/rishit-khandelwal>")
        .version(crate_version!())
        .about("A hobby package manager")
        .arg(
            Arg::with_name("install")
                .short("i")
                .long("install")
                .value_name("PKG")
                .help("Install a package")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let package = matches.value_of("install");

    if let Some(package) = package {
        install(&index, package).await;
    }
}
