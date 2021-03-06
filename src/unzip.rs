use std::fs;

pub fn extract_zip(name: &str) {
    let fname = std::path::Path::new(name);

    let file = fs::File::open(fname).unwrap();

    let mut arch = zip::ZipArchive::new(file).unwrap();

    arch.extract("bin/").unwrap();

    fs::remove_file(fname).unwrap();
}
