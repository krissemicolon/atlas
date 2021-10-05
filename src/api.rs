use std::io::{stdout, Write};
use curl::easy::Easy;

pub fn get_geo_from_host(host: &str) -> Vec<u8> {
    let mut easy = Easy::new();
    let mut cords = Vec::new();
    let url = format!("http://ip-api.com/line/{}?fields=lat,lon", host);

    easy.url(&url).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            &cords.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    cords
}
