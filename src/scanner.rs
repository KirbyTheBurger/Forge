#[derive(Debug, Clone)]
pub enum Line {
    Macro(Action, String),
    Delimiter(Action, String),
    Split(Action, String),
    Code(String),
}

#[derive(Debug, Clone)]
pub enum Action {
    Define,
    Remove,
}

pub fn scan(input: String) -> Vec<Line> {
    let mut output = vec![];

    let check_action = |l: &str| -> Action {
        if matches!(l.chars().nth(2), Some('/')) {
            Action::Remove
        } else {
            Action::Define
        }
    };

    let lines: Vec<&str> = input.lines().collect();

    let strip_prefix = |l: &str, prefix: &str| -> String {
        let without_slash = format!("{}/", prefix);
        l.trim_start_matches(without_slash.as_str())
        .trim_start_matches(prefix)
        .trim_start()
        .to_owned()
    };

    for l in lines {
        let l = l.trim();
        output.push(
            match l.get(..2) {
                Some("@@") => Line::Macro(check_action(l), strip_prefix(l, "@@")),
                Some("^^") => Line::Delimiter(check_action(l), strip_prefix(l, "^^")),
                Some("~~") => Line::Split(check_action(l), strip_prefix(l, "~~")),
                Some(_) if l.is_empty() => continue,
                Some(_) => Line::Code(l.to_owned()),
                None if !l.is_empty() => Line::Code(l.to_owned()),
                None => continue,
            }
        );
    }

    output
}