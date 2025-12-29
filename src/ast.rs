#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Command(Command),
    Group(Vec<Expr>),
    Literal(char),
    Superscript(Box<Expr>, Box<Expr>),
    Subscript(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Frac { numer: Box<Expr>, denom: Box<Expr> },
    Sqrt { content: Box<Expr> },
    Mathcal { content: Box<Expr> },
    Mathbb { content: Box<Expr> },
    Mathfrak { content: Box<Expr> },
    Mathbf { content: Box<Expr> },
    Mathit { content: Box<Expr> },
    Mathsf { content: Box<Expr> },
    Mathtt { content: Box<Expr> },
    Bar { content: Box<Expr> },
    Hat { content: Box<Expr> },
    Vec { content: Box<Expr> },
    Dot { content: Box<Expr> },
    Ddot { content: Box<Expr> },
    Tilde { content: Box<Expr> },
    Symbol { name: String },
}

#[derive(Clone, Copy)]
pub enum CommandDef {
    Symbol,
    Unary(fn(Box<Expr>) -> Command),
    Binary(fn(Box<Expr>, Box<Expr>) -> Command),
}

impl CommandDef {
    pub fn arity(&self) -> usize {
        match self {
            CommandDef::Symbol => 0,
            CommandDef::Unary(_) => 1,
            CommandDef::Binary(_) => 2,
        }
    }
}

pub struct CommandRegistry;

impl CommandRegistry {
    pub fn get(name: &str) -> CommandDef {
        match name {
            "frac" => CommandDef::Binary(|a, b| Command::Frac { numer: a, denom: b }),
            "sqrt" => CommandDef::Unary(|c| Command::Sqrt { content: c }),
            "mathcal" => CommandDef::Unary(|c| Command::Mathcal { content: c }),
            "mathbb" => CommandDef::Unary(|c| Command::Mathbb { content: c }),
            "mathfrak" => CommandDef::Unary(|c| Command::Mathfrak { content: c }),
            "mathbf" => CommandDef::Unary(|c| Command::Mathbf { content: c }),
            "mathit" => CommandDef::Unary(|c| Command::Mathit { content: c }),
            "mathsf" => CommandDef::Unary(|c| Command::Mathsf { content: c }),
            "mathtt" => CommandDef::Unary(|c| Command::Mathtt { content: c }),
            "bar" => CommandDef::Unary(|c| Command::Bar { content: c }),
            "hat" => CommandDef::Unary(|c| Command::Hat { content: c }),
            "vec" => CommandDef::Unary(|c| Command::Vec { content: c }),
            "dot" => CommandDef::Unary(|c| Command::Dot { content: c }),
            "ddot" => CommandDef::Unary(|c| Command::Ddot { content: c }),
            "tilde" => CommandDef::Unary(|c| Command::Tilde { content: c }),
            _ => CommandDef::Symbol,
        }
    }

    pub fn arity(name: &str) -> usize {
        Self::get(name).arity()
    }

    pub fn build(name: &str, mut args: Vec<Expr>) -> Command {
        match Self::get(name) {
            CommandDef::Symbol => Command::Symbol {
                name: name.to_string(),
            },
            CommandDef::Unary(builder) => {
                let arg = args
                    .pop()
                    .map(Box::new)
                    .unwrap_or_else(|| Box::new(Expr::Group(vec![])));
                builder(arg)
            }
            CommandDef::Binary(builder) => {
                let arg2 = args
                    .pop()
                    .map(Box::new)
                    .unwrap_or_else(|| Box::new(Expr::Group(vec![])));
                let arg1 = args
                    .pop()
                    .map(Box::new)
                    .unwrap_or_else(|| Box::new(Expr::Group(vec![])));
                builder(arg1, arg2)
            }
        }
    }
}
