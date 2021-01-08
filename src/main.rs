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
    .unwrap();

    let file_type = request
        .headers()
        .get("Content-Type")
        .expect("Expected a Content-Type header.")
        .to_str()
        .expect("Content-Type is not a string"); // I suspect this will never happen

    println!("{}", file_type);
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
