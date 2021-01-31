use opencv::imgcodecs;
use opencv::highgui;
use std::error::Error;


fn main() -> Result<(), Box<Error>> {
    const FIKENAME: &str = "/Users/satokeita/Pictures/1570017530.jpg";
    let img = imgcodecs::imread(FIKENAME, imgcodecs::IMREAD_COLOR)?;

    highgui::imshow("hello world", &img).unwrap();
    highgui::wait_key(10000).unwrap();
    Ok(())
}