use std::collections::HashMap;

#[derive(Clone)]
enum FlagKind {
    Bool(bool),
    String(Option<String>),
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

    pub fn string(&mut self, name: char) -> &mut Self {
        self.flags.push(FlagDef {
            name,
            kind: FlagKind::String(None),
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
                    f.kind.clone(),
                )
            })
            .collect();

        let mut positional: Vec<String> = Vec::new();
        let mut parsing_flags = true;

        loop {
            let arg = match iter.next() {
                Some(a) => a,
                None => break,
            };

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

            let mut chars = flags_str.chars().peekable();
            let snapshot: HashMap<char, FlagKind> = values.clone();
            let mut valid = true;
            let mut need_arg = false;

            while let Some(ch) = chars.next() {
                let is_last = chars.peek().is_none();
                match values.get(&ch) {
                    Some(FlagKind::String(_)) => {
                        if is_last {
                            let rest: String = chars.collect();
                            if !rest.is_empty() {
                                values.insert(ch, FlagKind::String(Some(rest)));
                            } else {
                                match iter.next() {
                                    Some(next_arg) => {
                                        values.insert(ch, FlagKind::String(Some(next_arg)));
                                    }
                                    None => {
                                        need_arg = true;
                                        valid = false;
                                    }
                                }
                            }
                        } else {
                            let rest: String = chars.collect();
                            if rest.is_empty() {
                                match iter.next() {
                                    Some(next_arg) => {
                                        values.insert(ch, FlagKind::String(Some(next_arg)));
                                    }
                                    None => {
                                        need_arg = true;
                                        valid = false;
                                    }
                                }
                            } else {
                                values.insert(ch, FlagKind::String(Some(rest)));
                            }
                        }
                        break;
                    }
                    Some(FlagKind::Bool(_)) => {
                        values.insert(ch, FlagKind::Bool(true));
                    }
                    None => {
                        valid = false;
                        break;
                    }
                }
            }

            if !valid {
                if need_arg {
                    // string flag missing argument: treat as error and stop parsing,
                    // but don't add this flag to positional (it was consumed)
                } else {
                    // unknown flag: rollback and treat as positional
                    values = snapshot;
                    positional.push(arg);
                }
                parsing_flags = false;
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

    pub fn string(&self, name: char) -> Option<&str> {
        self.values.get(&name).and_then(|v| match v {
            FlagKind::String(v) => v.as_deref(),
            _ => None,
        })
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }
}

