use std::collections::HashMap;

enum FlagKind {
    Bool(bool),
}

struct FlagDef {
    name: char,
    kind: FlagKind,
}

pub struct FlagParser {
    flags: Vec<FlagDef>,
}

pub struct Parsed {
    values: HashMap<char, FlagKind>,
    args: Vec<String>,
}

impl FlagParser {
    pub fn new() -> Self {
        Self { flags: Vec::new() }
    }

    pub fn bool(&mut self, name: char, default: bool) -> &mut Self {
        self.flags.push(FlagDef {
            name,
            kind: FlagKind::Bool(default),
        });
        self
    }

    pub fn parse<I>(&self, args: I) -> Parsed
    where
        I: IntoIterator<Item = String>,
    {
        let mut iter = args.into_iter();
        let _program = iter.next();

        let mut values: HashMap<char, FlagKind> = self
            .flags
            .iter()
            .map(|f| {
                (
                    f.name,
                    match f.kind {
                        FlagKind::Bool(v) => FlagKind::Bool(v),
                    },
                )
            })
            .collect();

        let mut positional: Vec<String> = Vec::new();
        let mut parsing_flags = true;

        for arg in iter {
            if !parsing_flags
                || arg == "--"
                || arg == "-"
                || arg.is_empty()
                || !arg.starts_with('-')
            {
                parsing_flags = false;
                if arg != "--" {
                    positional.push(arg);
                }
                continue;
            }

            let flags_str = &arg[1..];
            if flags_str.is_empty() {
                parsing_flags = false;
                positional.push(arg);
                continue;
            }

            let snapshot: HashMap<char, bool> = values
                .iter()
                .map(|(k, v)| (*k, matches!(v, FlagKind::Bool(true))))
                .collect();

            let mut valid = true;
            for ch in flags_str.chars() {
                if let Some(value) = values.get_mut(&ch) {
                    match value {
                        FlagKind::Bool(v) => *v = true,
                    }
                } else {
                    valid = false;
                    break;
                }
            }

            if !valid {
                for (ch, was_true) in &snapshot {
                    if let Some(value) = values.get_mut(ch) {
                        match value {
                            FlagKind::Bool(v) => *v = *was_true,
                        }
                    }
                }
                parsing_flags = false;
                positional.push(arg);
            }
        }

        Parsed { values, args: positional }
    }
}

impl Parsed {
    pub fn bool(&self, name: char) -> bool {
        self.values
            .get(&name)
            .map(|v| matches!(v, FlagKind::Bool(true)))
            .unwrap_or(false)
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }
}

