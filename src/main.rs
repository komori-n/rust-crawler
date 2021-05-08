use anyhow::Result;

fn main() -> Result<()> {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;

    println!("body = {:?}", body);

    Ok(())
}
