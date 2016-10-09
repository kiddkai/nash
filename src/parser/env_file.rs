use super::{EnvVar, ParseError, ErrorDetail, ParseResult};

type ItemResult = Result<EnvVar, ErrorDetail>;

pub fn parse_env_content(content: &str) -> ParseResult {
    let (pairs, errors): (Vec<ItemResult>, Vec<ItemResult>) = content.lines()
        .zip(0..)
        .filter(|&(s, _)| s.trim().len() > 0)
        .map(|(s, line)| {
            let pair = s.split("=")
                .map(|item| item.trim())
                .collect::<Vec<&str>>();
            match pair.len() {
                2 => Ok(EnvVar(pair[0].to_string(), pair[1].to_string())),
                _ => {
                    Err(ErrorDetail {
                        line_content: s.to_string(),
                        line_number: line,
                        description: "should have the format NAME=VALUE".to_string(),
                    })
                }
            }
        })
        .partition(|r| r.is_ok());

    if errors.len() > 0 {
        return Err(ParseError {
            content: content.to_string(),
            errors: errors.into_iter()
                .map(|v| v.err().unwrap())
                .collect::<Vec<ErrorDetail>>(),
        });
    }

    Ok(pairs.into_iter()
        .map(|v| v.unwrap())
        .collect::<Vec<EnvVar>>())
}

#[test]
fn test_simple_parse() {
    let res = parse_env_content("FOO=BAR").unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], EnvVar("FOO".to_string(), "BAR".to_string()));
}

#[test]
fn test_multiple_parse() {
    let content = "FOO=BAR\nBAR=BAZ";
    let res = parse_env_content(content).unwrap();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0], EnvVar("FOO".to_string(), "BAR".to_string()));
    assert_eq!(res[1], EnvVar("BAR".to_string(), "BAZ".to_string()));
}

#[test]
fn test_remove_spaces() {
    let content = "FOO=BAR     \n     BAR=BAZ";
    let res = parse_env_content(content).unwrap();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0], EnvVar("FOO".to_string(), "BAR".to_string()));
    assert_eq!(res[1], EnvVar("BAR".to_string(), "BAZ".to_string()));
}

#[test]
fn test_ignore_empty_lines() {
    let content = "FOO=BAR    \n \n \n \n     BAR=BAZ";
    let res = parse_env_content(content).unwrap();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0], EnvVar("FOO".to_string(), "BAR".to_string()));
    assert_eq!(res[1], EnvVar("BAR".to_string(), "BAZ".to_string()));
}

#[test]
fn test_generate_parse_error() {
    let content = "FOO";
    let res = parse_env_content(content);

    match res {
        Ok(_) => panic!("should have error"),
        Err(e) => {
            assert_eq!("FOO", e.content);
            assert_eq!(1, e.errors.len());
            assert_eq!(0, e.errors[0].line_number);
        }
    }
}

#[test]
fn test_generate_parse_error_for_multiline_env() {
    let content = "FOO=BAR\nBAR";
    let res = parse_env_content(content);

    match res {
        Ok(_) => panic!("should have error"),
        Err(e) => {
            assert_eq!(content, e.content);
            assert_eq!(1, e.errors.len());
            assert_eq!(1, e.errors[0].line_number);
            assert_eq!("BAR", e.errors[0].line_content);
        }
    }
}
