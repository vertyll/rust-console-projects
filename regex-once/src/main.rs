use once_cell::sync::OnceCell;
use regex::Regex;
use std::io;

static DATE_RE: OnceCell<Regex> = OnceCell::new();

fn main() {
    let re = DATE_RE.get_or_init(|| {
        Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap()
    });

    println!("Wprowadź datę w formacie YYYY-MM-DD:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Some(caps) = re.captures(input.trim()) {
        let year = caps.name("year").unwrap().as_str();
        let month = caps.name("month").unwrap().as_str();
        let day = caps.name("day").unwrap().as_str();

        println!("Rok: {}, Miesiąc: {}, Dzień: {}", year, month, day);
    } else {
        println!("Wprowadzony tekst nie jest poprawną datą.");
    }
}
