fn main() {
    let response = reqwest::blocking::get(
        "https://en.wiktionary.org/wiki/no%C5%BCyk#Polish",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    println!(document);
}
