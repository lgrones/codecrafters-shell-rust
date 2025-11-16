pub struct Params {
    pub name: String,
    pub args: Vec<String>,
}

pub trait SplitArgs {
    const QUOTES: [char; 2];
    const ESCAPE: char;
    fn get_args(&self) -> Params;
}

impl SplitArgs for &str {
    const QUOTES: [char; 2] = ['\'', '"'];
    const ESCAPE: char = '\\';

    // Yes, this is a tokenizer
    // No, don't ask me how in the fuck this works
    fn get_args(&self) -> Params {
        let mut args = vec![];
        let mut arg = vec![];
        let mut quote = None;
        let mut escaped = false;

        for char in self.trim().chars() {
            if !escaped
                && <&str as SplitArgs>::QUOTES.contains(&char)
                && quote.is_none_or(|x| x == char)
            {
                quote = match quote {
                    Some(_) => None,
                    None => Some(char),
                };

                continue;
            }

            if !escaped && char == <&str as SplitArgs>::ESCAPE {
                escaped = true;
                continue;
            }

            if quote.is_some_and(|x| x != char) && char != <&str as SplitArgs>::ESCAPE && escaped {
                arg.push(<&str as SplitArgs>::ESCAPE);
                arg.push(char);
                escaped = false;
                continue;
            }

            if quote.is_some() || escaped || char != ' ' {
                arg.push(char);
                escaped = false;
                continue;
            }

            args.push(arg.iter().collect::<String>());
            arg.clear();
        }

        args.push(arg.iter().collect::<String>());

        let mut iter = args.into_iter();

        Params {
            name: iter.next().unwrap_or(String::new()),
            args: iter.filter(|x| !x.trim().is_empty()).collect(),
        }
    }
}
