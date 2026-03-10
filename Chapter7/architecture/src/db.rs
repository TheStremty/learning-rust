pub struct Config {
    database_url: String,
}
impl Config {
    pub fn new(url: &str) -> Self{
        Self{
            database_url: String::from(url),
        }
    }
    pub fn get_url(&self) -> &str {
        &self.database_url
    }
}