use crate::Paths;

pub fn get_image(paths: &Paths, requestpath: &str) -> Result<Vec<u8>, ()> {
    let path = paths.wikipath().to_owned() + requestpath;
    println!("searching for image at: {}", path);
    match std::fs::read(path) {
        Ok(image_data) => Ok(image_data),
        Err(_)         => return Err(()),
    }
}
