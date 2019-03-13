extern crate reqwest;

fn main() {
    let result = fetch_text("http://www.rust-lang.org");
    let count = result.unwrap().chars().count();

    println!("{}", count);
}

fn fetch_text(url: &str) -> Result<(String), reqwest::Error> {
    let body = reqwest::get(url)?.text()?;
    // println!("{}", body);

    Ok(body)
}
