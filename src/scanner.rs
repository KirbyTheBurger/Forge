#[derive(Debug, Clone)]
pub enum Line {
    Rule(Action, String),
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
    for l in lines {
        let l = l.trim();
        output.push(
            match l.get(..2) {
                Some("@@") => Line::Rule(check_action(l), l.to_owned()),
                Some("^^") => Line::Delimiter(check_action(l), l.to_owned()),
                Some("~~") => Line::Split(check_action(l), l.to_owned()),
                Some(_) if l.is_empty() => continue,
                Some(_) => Line::Code(l.to_owned()),
                None if !l.is_empty() => Line::Code(l.to_owned()),
                None => continue,
            }
        );
    }

    output
}