use color_eyre::Result;

use crate::config::Config;

const TEXT: &'static str = "This is a bare minimum example.   There are many approaches to running an application loop, so
this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
teardown of a terminal application.";

pub fn get_text(_config: &Config) -> Result<String> {
    // TODO: get text from different sources
    Ok(TEXT.to_string())
}

