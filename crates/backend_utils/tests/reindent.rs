use interoptopus_backends::template::{CurlyPlacement, IndentConfig, reindent};
use std::error::Error;

#[test]
fn newlines() -> Result<(), Box<dyn Error>> {
    let config_newline = IndentConfig { curly: CurlyPlacement::Newline, ..Default::default() };

    insta::assert_snapshot!(reindent(include_str!("reindent/basic.cs"), &config_newline)?);

    Ok(())
}

#[test]
fn curly() -> Result<(), Box<dyn Error>> {
    let config_end_of_line = IndentConfig { curly: CurlyPlacement::EndOfLine, ..Default::default() };

    insta::assert_snapshot!(reindent(include_str!("reindent/basic.cs"), &config_end_of_line)?);

    Ok(())
}
