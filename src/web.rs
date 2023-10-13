pub fn http_get() -> String {
    let sc = String::from(reqwest::blocking::get("https://scratchgraph.com/healthz").unwrap().status().as_str());
    sc
}
