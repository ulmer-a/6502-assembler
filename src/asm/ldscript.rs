use logos::Logos;

#[derive(Logos, PartialEq, Copy, Clone)]
enum LdScriptToken {
    #[regex(r".[A-Za-z_]+")]
    SectionIdentifier,

    #[regex(r"@0x[0-9A-Fa-f]+")]
    Address,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f\n]+", logos::skip)]
    Error,
}

#[derive(Debug, PartialEq)]
pub struct LdSection {
    name: String,
    load_addr: Option<u16>,
}

#[cfg(test)]
fn check_for_addr(next_token: Option<LdScriptToken>, slice: &str) -> Option<u16> {
    if next_token? == LdScriptToken::Address {
        let hex_addr = &slice[3..];
        Some(u16::from_str_radix(hex_addr, 16).unwrap())
    } else {
        None
    }
}

#[cfg(test)]
pub fn parse(source: &str) -> Result<Vec<LdSection>, String> {
    let mut lexer = LdScriptToken::lexer(source);
    let mut sections = vec![];
    let mut current_token = lexer.next();
    loop {
        if let Some(token) = current_token {
            match token {
                LdScriptToken::SectionIdentifier => {
                    let name: String = (&lexer.slice()[1..]).into();
                    current_token = lexer.next();
                    let load_addr = check_for_addr(current_token, lexer.slice());
                    if load_addr.is_some() {
                        current_token = lexer.next();
                    }
                    sections.push(LdSection { name, load_addr });
                }
                _ => {
                    return Err(format!("unexpected token: '{}'", lexer.slice()));
                }
            }
        } else {
            break;
        }
    }
    Ok(sections)
}

#[test]
fn linker_script_parser_test() {
    let sections = parse(
        r#"
        .text @0xe000
        .data
        .vectors @0xfffa
    "#
        .into(),
    )
    .unwrap();
    assert_eq!(
        sections,
        vec![
            LdSection {
                name: "text".into(),
                load_addr: Some(0xe000),
            },
            LdSection {
                name: "data".into(),
                load_addr: None,
            },
            LdSection {
                name: "vectors".into(),
                load_addr: Some(0xfffa),
            }
        ]
    );
}

#[test]
fn linker_script_error_test() {
    let sections = parse(
        r#"
        @0x0001
        .text
        .data
        .vectors @0xfffa
    "#
        .into(),
    );
    assert_eq!(sections, Err("unexpected token: '@0x0001'".into()));
}
