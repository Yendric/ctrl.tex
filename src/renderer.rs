use crate::ast::{Command, Expr};

#[derive(Default)]
pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer
    }

    pub fn render(&self, exprs: &[Expr]) -> String {
        exprs.iter().map(|e| self.render_expr(e)).collect()
    }

    fn render_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(c) => c.to_string(),
            Expr::Command(cmd) => self.render_command(cmd),
            Expr::Group(group_exprs) => self.render(group_exprs),
            Expr::Superscript(base, exp) => {
                let base_str = self.render_expr(base);
                let exp_str = self.render_expr(exp);
                if let Some(sup_str) = to_superscript(&exp_str) {
                    format!("{}{}", base_str, sup_str)
                } else {
                    format!("{}^{{{}}}", base_str, exp_str)
                }
            }
            Expr::Subscript(base, sub) => {
                let base_str = self.render_expr(base);
                let sub_str = self.render_expr(sub);
                if let Some(sub_chars) = to_subscript(&sub_str) {
                    format!("{}{}", base_str, sub_chars)
                } else {
                    format!("{}_{{{}}}", base_str, sub_str)
                }
            }
        }
    }

    fn render_command(&self, cmd: &Command) -> String {
        match cmd {
            Command::Frac { numer, denom } => {
                format!(
                    "({})/({})",
                    self.render_expr(numer),
                    self.render_expr(denom)
                )
            }
            Command::Sqrt { content } => format!("√({})", self.render_expr(content)),
            Command::Mathcal { content } => apply_style("mathcal", &self.render_expr(content)),
            Command::Mathbb { content } => apply_style("mathbb", &self.render_expr(content)),
            Command::Mathfrak { content } => apply_style("mathfrak", &self.render_expr(content)),
            Command::Mathbf { content } => apply_style("mathbf", &self.render_expr(content)),
            Command::Mathit { content } => apply_style("mathit", &self.render_expr(content)),
            Command::Mathsf { content } => apply_style("mathsf", &self.render_expr(content)),
            Command::Mathtt { content } => apply_style("mathtt", &self.render_expr(content)),
            Command::Bar { content } => format!("{}\u{0304}", self.render_expr(content)),
            Command::Hat { content } => format!("{}\u{0302}", self.render_expr(content)),
            Command::Vec { content } => format!("{}\u{20D7}", self.render_expr(content)),
            Command::Dot { content } => format!("{}\u{0307}", self.render_expr(content)),
            Command::Ddot { content } => format!("{}\u{0308}", self.render_expr(content)),
            Command::Tilde { content } => format!("{}\u{0303}", self.render_expr(content)),
            Command::Symbol { name } => self.render_symbol(name),
        }
    }

    fn render_symbol(&self, name: &str) -> String {
        match name {
            "alpha" => "α".to_string(),
            "beta" => "β".to_string(),
            "gamma" => "γ".to_string(),
            "delta" => "δ".to_string(),
            "epsilon" => "ε".to_string(),
            "zeta" => "ζ".to_string(),
            "eta" => "η".to_string(),
            "theta" => "θ".to_string(),
            "iota" => "ι".to_string(),
            "kappa" => "κ".to_string(),
            "lambda" => "λ".to_string(),
            "mu" => "μ".to_string(),
            "nu" => "ν".to_string(),
            "xi" => "ξ".to_string(),
            "omicron" => "ο".to_string(),
            "pi" => "π".to_string(),
            "rho" => "ρ".to_string(),
            "sigma" => "σ".to_string(),
            "tau" => "τ".to_string(),
            "upsilon" => "υ".to_string(),
            "phi" => "φ".to_string(),
            "chi" => "χ".to_string(),
            "psi" => "ψ".to_string(),
            "omega" => "ω".to_string(),
            "Gamma" => "Γ".to_string(),
            "Delta" => "Δ".to_string(),
            "Theta" => "Θ".to_string(),
            "Lambda" => "Λ".to_string(),
            "Xi" => "Ξ".to_string(),
            "Pi" => "Π".to_string(),
            "Sigma" => "Σ".to_string(),
            "Upsilon" => "Υ".to_string(),
            "Phi" => "Φ".to_string(),
            "Psi" => "Ψ".to_string(),
            "Omega" => "Ω".to_string(),
            "le" | "leq" => "≤".to_string(),
            "ge" | "geq" => "≥".to_string(),
            "ne" | "neq" => "≠".to_string(),
            "approx" => "≈".to_string(),
            "equiv" => "≡".to_string(),
            "sim" => "∼".to_string(),
            "cong" => "≅".to_string(),
            "propto" => "∝".to_string(),
            "pm" => "±".to_string(),
            "times" => "×".to_string(),
            "div" => "÷".to_string(),
            "cdot" => "⋅".to_string(),
            "in" => "∈".to_string(),
            "notin" => "∉".to_string(),
            "subset" => "⊂".to_string(),
            "subseteq" => "⊆".to_string(),
            "cup" => "∪".to_string(),
            "cap" => "∩".to_string(),
            "setminus" => "∖".to_string(),
            "emptyset" => "∅".to_string(),
            "land" | "wedge" => "∧".to_string(),
            "lor" | "vee" => "∨".to_string(),
            "neg" | "lnot" => "¬".to_string(),
            "implies" => "⟹".to_string(),
            "iff" => "⟺".to_string(),
            "forall" => "∀".to_string(),
            "exists" => "∃".to_string(),
            "rightarrow" | "to" => "→".to_string(),
            "leftarrow" => "←".to_string(),
            "Rightarrow" => "⇒".to_string(),
            "Leftarrow" => "⇐".to_string(),
            "leftrightarrow" => "↔".to_string(),
            "Leftrightarrow" => "⇔".to_string(),
            "mapsto" => "↦".to_string(),
            "partial" => "∂".to_string(),
            "nabla" => "∇".to_string(),
            "sum" => "∑".to_string(),
            "prod" => "∏".to_string(),
            "int" => "∫".to_string(),
            "infty" => "∞".to_string(),
            "ldots" | "dots" => "…".to_string(),
            "cdots" => "⋯".to_string(),
            "vdots" => "⋮".to_string(),
            "ddots" => "⋱".to_string(),
            "prime" => "′".to_string(),
            "degree" => "°".to_string(),
            "angle" => "∠".to_string(),
            "triangle" => "△".to_string(),
            "circ" => "∘".to_string(),
            "bullet" => "∙".to_string(),
            "star" => "⋆".to_string(),
            "ast" => "∗".to_string(),
            "mid" => "|".to_string(),
            "parallel" => "∥".to_string(),
            "perp" => "⊥".to_string(),
            "dagger" => "†".to_string(),
            "ddagger" => "‡".to_string(),
            "ell" => "ℓ".to_string(),
            "Re" => "ℜ".to_string(),
            "Im" => "ℑ".to_string(),
            "aleph" => "ℵ".to_string(),
            "hbar" => "ℏ".to_string(),
            "{" => "{".to_string(),
            "}" => "}".to_string(),
            "sin" | "cos" | "tan" | "csc" | "sec" | "cot" | "sinh" | "cosh" | "tanh" | "arcsin"
            | "arccos" | "arctan" | "log" | "ln" | "lim" | "min" | "max" | "sup" | "inf"
            | "det" | "exp" | "dim" | "ker" | "deg" | "arg" => name.to_string(),
            "," | ";" | ":" => " ".to_string(),
            "!" => "".to_string(),
            "quad" => "  ".to_string(),
            "qquad" => "    ".to_string(),
            _ => format!("\\{}", name),
        }
    }
}

fn to_superscript(s: &str) -> Option<String> {
    let mut result = String::new();
    for c in s.chars() {
        let sup = match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            '+' => '⁺',
            '-' => '⁻',
            '=' => '⁼',
            '(' => '⁽',
            ')' => '⁾',
            ',' => 'ʼ',
            '.' => '˙',
            '*' => '*',
            'a' => 'ᵃ',
            'b' => 'ᵇ',
            'c' => 'ᶜ',
            'd' => 'ᵈ',
            'e' => 'ᵉ',
            'f' => 'ᶠ',
            'g' => 'ᵍ',
            'h' => 'ʰ',
            'i' => 'ⁱ',
            'j' => 'ʲ',
            'k' => 'ᵏ',
            'l' => 'ˡ',
            'm' => 'ᵐ',
            'n' => 'ⁿ',
            'o' => 'ᵒ',
            'p' => 'ᵖ',
            'r' => 'ʳ',
            's' => 'ˢ',
            't' => 'ᵗ',
            'u' => 'ᵘ',
            'v' => 'ᵛ',
            'w' => 'ʷ',
            'x' => 'ˣ',
            'y' => 'ʸ',
            'z' => 'ᶻ',
            'A' => 'ᴬ',
            'B' => 'ᴮ',
            'D' => 'ᴰ',
            'E' => 'ᴱ',
            'G' => 'ᴳ',
            'H' => 'ᴴ',
            'I' => 'ᴵ',
            'J' => 'ᴶ',
            'K' => 'ᴷ',
            'L' => 'ᴸ',
            'M' => 'ᴹ',
            'N' => 'ᴺ',
            'O' => 'ᴼ',
            'P' => 'ᴾ',
            'R' => 'ᴿ',
            'T' => 'ᵀ',
            'U' => 'ᵁ',
            'V' => 'ⱽ',
            'W' => 'ᵂ',
            'α' => 'ᵅ',
            'β' => 'ᵝ',
            'γ' => 'ᵞ',
            'δ' => 'ᵟ',
            'ε' => 'ᵋ',
            'θ' => 'ᶿ',
            'ι' => 'ᶥ',
            'φ' => 'ᵠ',
            'χ' => 'ᵡ',
            'ʊ' => 'ᵁ',
            'ə' => 'ᵊ',
            'ɛ' => 'ᵋ',
            'ɣ' => 'ˠ',
            'ʁ' => 'ʶ',
            'ʃ' => 'ᶴ',
            'ʒ' => 'ᶾ',
            'ŋ' => 'ᵑ',
            _ => return None,
        };
        result.push(sup);
    }
    Some(result)
}

fn to_subscript(s: &str) -> Option<String> {
    let mut result = String::new();
    for c in s.chars() {
        let sub = match c {
            '0' => '₀',
            '1' => '₁',
            '2' => '₂',
            '3' => '₃',
            '4' => '₄',
            '5' => '₅',
            '6' => '₆',
            '7' => '₇',
            '8' => '₈',
            '9' => '₉',
            '+' => '₊',
            '-' => '₋',
            '=' => '₌',
            '(' => '₍',
            ')' => '₎',
            ',' => '‚',
            '.' => '.',
            'a' => 'ₐ',
            'e' => 'ₑ',
            'h' => 'ₕ',
            'i' => 'ᵢ',
            'j' => 'ⱼ',
            'k' => 'ₖ',
            'l' => 'ₗ',
            'm' => 'ₘ',
            'n' => 'ₙ',
            'o' => 'ₒ',
            'p' => 'ₚ',
            'r' => 'ᵣ',
            's' => 'ₛ',
            't' => 'ₜ',
            'u' => 'ᵤ',
            'v' => 'ᵥ',
            'x' => 'ₓ',
            'β' => 'ᵦ',
            'γ' => 'ᵧ',
            'ρ' => 'ᵨ',
            'φ' => 'ᵩ',
            'χ' => 'ᵪ',
            'ə' => 'ₔ',
            _ => return None,
        };
        result.push(sub);
    }
    Some(result)
}

fn apply_style(style: &str, content: &str) -> String {
    let mut result = String::new();
    for c in content.chars() {
        result.push(map_char_style(style, c));
    }
    result
}

fn map_char_style(style: &str, c: char) -> char {
    let code = c as u32;

    match style {
        "mathbb" => match c {
            'C' => '\u{2102}',
            'H' => '\u{210D}',
            'N' => '\u{2115}',
            'P' => '\u{2119}',
            'Q' => '\u{211A}',
            'R' => '\u{211D}',
            'Z' => '\u{2124}',
            'A'..='Z' => char::from_u32(0x1D538 + (code - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D56C + (code - 'a' as u32)).unwrap_or(c),
            '0'..='9' => char::from_u32(0x1D7D8 + (code - '0' as u32)).unwrap_or(c),
            _ => c,
        },
        "mathcal" => match c {
            'B' => '\u{212C}',
            'E' => '\u{2130}',
            'F' => '\u{2131}',
            'H' => '\u{210B}',
            'I' => '\u{2110}',
            'L' => '\u{2112}',
            'M' => '\u{2133}',
            'R' => '\u{211B}',
            'e' => '\u{212F}',
            'g' => '\u{210A}',
            'o' => '\u{2134}',
            'A'..='Z' => char::from_u32(0x1D49C + (code - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D4B6 + (code - 'a' as u32)).unwrap_or(c),
            _ => c,
        },
        "mathfrak" => match c {
            'A'..='Z' => char::from_u32(0x1D504 + (code - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D51E + (code - 'a' as u32)).unwrap_or(c),
            _ => c,
        },
        "mathbf" => match c {
            'A'..='Z' => char::from_u32(0x1D400 + (code - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D41A + (code - 'a' as u32)).unwrap_or(c),
            '0'..='9' => char::from_u32(0x1D7CE + (code - '0' as u32)).unwrap_or(c),
            _ => c,
        },
        "mathit" => match c {
            'A'..='Z' => char::from_u32(0x1D434 + (code - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D44E + (code - 'a' as u32)).unwrap_or(c),
            _ => c,
        },
        "mathsf" => match c {
            'A'..='Z' => char::from_u32(0x1D5A0 + (code - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D5BA + (code - 'a' as u32)).unwrap_or(c),
            '0'..='9' => char::from_u32(0x1D7E2 + (code - '0' as u32)).unwrap_or(c),
            _ => c,
        },
        "mathtt" => match c {
            'A'..='Z' => char::from_u32(0x1D670 + (code - 'A' as u32)).unwrap_or(c),
            'a'..='z' => char::from_u32(0x1D68A + (code - 'a' as u32)).unwrap_or(c),
            '0'..='9' => char::from_u32(0x1D7F6 + (code - '0' as u32)).unwrap_or(c),
            _ => c,
        },
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn render(input: &str) -> String {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();
        let renderer = Renderer::new();
        renderer.render(&ast)
    }

    #[test]
    fn test_greek() {
        assert_eq!(render(r"\alpha \beta \Gamma"), "αβΓ");
    }

    #[test]
    fn test_superscript() {
        assert_eq!(render("x^2"), "x²");
        assert_eq!(render("x^{10}"), "x¹⁰");
        assert_eq!(render("x^{y}"), "xʸ");
        assert_eq!(render("x^{q}"), "x^{q}");
    }

    #[test]
    fn test_subscript() {
        assert_eq!(render("x_2"), "x₂");
        assert_eq!(render("x_{ij}"), "xᵢⱼ");
    }

    #[test]
    fn test_fraction() {
        assert_eq!(render(r"\frac{1}{2}"), "(1)/(2)");
        assert_eq!(render(r"\frac{a+b}{c}"), "(a+b)/(c)");
    }

    #[test]
    fn test_styles() {
        let res = render(r"\mathbb{R}");
        assert_eq!(res, "ℝ");
    }

    #[test]
    fn test_complex_expression() {
        let input = r"\alpha^2 + \beta_i = \frac{\gamma}{2}";
        let output = render(input);
        assert_eq!(output, "α²+βᵢ=(γ)/(2)");
    }

    #[test]
    fn test_ldots() {
        assert_eq!(render(r"\ldots"), "…");
        assert_eq!(render(r"\dots"), "…");
        assert_eq!(render(r"\cdots"), "⋯");
    }

    #[test]
    fn test_logic() {
        assert_eq!(render(r"\land"), "∧");
        assert_eq!(render(r"\lor"), "∨");
        assert_eq!(render(r"\neg"), "¬");
        assert_eq!(render(r"\implies"), "⟹");
        assert_eq!(render(r"\iff"), "⟺");
    }

    #[test]
    fn test_misc_symbols() {
        assert_eq!(render(r"\circ"), "∘");
        assert_eq!(render(r"\bullet"), "∙");
        assert_eq!(render(r"\star"), "⋆");
        assert_eq!(render(r"\ast"), "∗");
        assert_eq!(render(r"\mid"), "|");
        assert_eq!(render(r"\parallel"), "∥");
        assert_eq!(render(r"\perp"), "⊥");
        assert_eq!(render(r"\dagger"), "†");
        assert_eq!(render(r"\ddagger"), "‡");
        assert_eq!(render(r"\ell"), "ℓ");
        assert_eq!(render(r"\Re"), "ℜ");
        assert_eq!(render(r"\Im"), "ℑ");
        assert_eq!(render(r"\aleph"), "ℵ");
        assert_eq!(render(r"\hbar"), "ℏ");
        assert_eq!(render(r"\mapsto"), "↦");
    }

    #[test]
    fn test_spaces() {
        assert_eq!(render(r"a \, b"), "a b");
        assert_eq!(render(r"a \; b"), "a b");
        assert_eq!(render(r"a \quad b"), "a  b");
        assert_eq!(render(r"a \! b"), "ab");
    }

    #[test]
    fn test_functions() {
        assert_eq!(render(r"\sin x"), "sinx");
        assert_eq!(render(r"\cos(x)"), "cos(x)");
        assert_eq!(render(r"\log x"), "logx");
        assert_eq!(render(r"\ln e"), "lne");
        assert_eq!(render(r"\max(a,b)"), "max(a,b)");
    }

    #[test]
    fn test_escaped_braces() {
        assert_eq!(render(r"\{ a, b \}").trim(), "{a,b}");
        assert_eq!(render(r"\{ \}").trim(), "{}");
    }

    #[test]
    fn test_superscript_comma() {
        assert_eq!(render("x^{,2}"), "xʼ²");
        assert_eq!(render("x_{,2}"), "x‚₂");
    }

    #[test]
    fn test_extended_superscript() {
        assert_eq!(render("x^A"), "xᴬ");
        assert_eq!(render("x^B"), "xᴮ");
        assert_eq!(render("x^\\beta"), "xᵝ");
        assert_eq!(render("x^\\gamma"), "xᵞ");
    }

    #[test]
    fn test_extended_subscript() {
        assert_eq!(render("x_\\beta"), "xᵦ");
        assert_eq!(render("x_\\rho"), "xᵨ");
    }

    #[test]
    fn test_accents() {
        assert_eq!(render(r"\bar{x}"), "x̄");
        assert_eq!(render(r"\hat{x}"), "x̂");
        assert_eq!(render(r"\vec{x}"), "x⃗");
        assert_eq!(render(r"\dot{x}"), "ẋ");
        assert_eq!(render(r"\ddot{x}"), "ẍ");
        assert_eq!(render(r"\tilde{x}"), "x̃");
    }
    #[test]
    fn test_superscript_star() {
        assert_eq!(render("x^{*}"), "x*");
        assert_eq!(render("1^{*}"), "1*");
        assert_eq!(render("A^*"), "A*");
    }
}
