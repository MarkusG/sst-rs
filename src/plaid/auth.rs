use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub token: String
}

impl Credentials {
    pub fn from_file(path: &Path) -> Result<Credentials, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let lines: Vec<_> = contents.split('\n').collect();

        // first line is client id, second line is client secret
        Ok(Credentials {
            client_id: lines[0].to_string(),
            client_secret: lines[1].to_string(),
            token: lines[2].to_string()
        })
    }
}
