#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

/* A user defined type */
type Msg = String;

/* Parse a line of the log file to structured data. */
fn parse_log(line: &str) -> (Event, Msg) {
    let parts: Vec<_> = line.splitn(2, ' ').collect();

    /* If there aren't two parts to the line, it is invallid. */
    if parts.len() != 2 {
        return (Event::Unknown, String::from(line));
    }

    /* Assign the parts to variables. */
    let event = parts[0];
    let rest = String::from(parts[1]);

    /* Match the event string to a structured type. */
    return match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    };
}

fn main() {
    let log = "\
BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_res = parse_log(line);
        println!("{:?}", parse_res);
    }
}
