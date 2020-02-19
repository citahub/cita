#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub free_trial_time: Cow<'static, str>,
    pub issuer_address: Cow<'static, str>,
}

pub const LICENSE_CONFIG: Config = Config {
    free_trial_time: Cow::Borrowed("864000"),
    issuer_address: Cow::Borrowed("0x69144e9b845fadd41ee0c14e3127c1df87f2500e"),
};
