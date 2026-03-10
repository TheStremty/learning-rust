use architecture::db::Config as dbConfig;
use architecture::fast_parse;

fn main(){
    let config = dbConfig::new("localhost:5432");
    println!("Połączono z: {}", config.get_url());

    fast_parse("jakieś dane");
}