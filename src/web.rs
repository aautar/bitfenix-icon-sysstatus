pub fn http_get() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://scratchgraph.com/healthz")?.text()?;
    println!("{}", resp);
    Ok(())
}
