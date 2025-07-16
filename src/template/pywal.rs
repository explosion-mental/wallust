//! Pywal Template Engine
//! This module goal is to create a 1to1, or realistically a "just works", replica of the pywal
//! templating system. The pywal system is described in the pywal wiki [1] and it should be able to
//! at least parse all the pywal templates [2].
//! The code bellow is an engine made by hand.
//!
//! Refs:
//! 1: https://github.com/dylanaraps/pywal/wiki/User-Template-Files
//! 2: https://github.com/dylanaraps/pywal/tree/master/pywal/templates
use crate::colors::Myrgb;
use super::alpha_hexa;
use super::TemplateFields;

use thiserror::Error;
use palette::Srgb;

#[derive(Error, Debug)]
pub enum PywalTemplateError {
    #[error("Missing variable: {0}")]
    MissingVariable(String),
    #[error("Invalid modifier: {0}")]
    InvalidModifier(String),
}

fn get_func(fname: &str, value: &str, alpha: u8) -> Result<String, PywalTemplateError> {
    let c: Srgb<u8> = value.parse().expect("SHOULD BE A VALID COLOR");
    let c: Myrgb = c.into();
    let alpha_hex = alpha_hexa(alpha as usize).expect("CANNOT OVERFLOW, validation with clap 0..=100");
    let alpha_dec = f32::from(alpha) / 100.0;
    let alpha_dec_display = if alpha % 10 == 0 { format!("{alpha_dec:.1}") } else { format!("{alpha_dec:.2}") };

    let ret = match fname {
        "rgb" => c.rgb(), //.rgb output `235,235,235`
        "rgba" => c.rgba(alpha_dec), //.rgba output `235,235,235,1.0`
        "xrgba" => c.xrgba(&alpha_hex), //.xrgba output `ee/ee/ee/ff`
        "strip" => c.strip(), //.strip output `EEEEEE`
        "red" => c.red(),
        "green" => c.green(),
        "blue" => c.blue(),
        "alpha" => format!("[{}]{c}", alpha),
        "alpha_dec" => alpha_dec_display,
        _ => return Err(PywalTemplateError::InvalidModifier(fname.to_string())),
    };

    Ok(ret)
}

pub fn render(content: &str, t: &TemplateFields) -> Result<String, PywalTemplateError> {
    let mut output = String::new();
    let mut i = 0;
    while i < content.len() {
        // println!("{i}\n{output}\n\n");
        if content.chars().nth(i) == Some('{')
        && content.chars().nth(i+1) == Some('{')
        {
            let mut counts = 2; //starts from 2
            //current char [nth(i)] is "{" and next one [nth(i+1)] as well
            while content.chars().nth(i+counts) == Some('{') {
                counts += 1;
            }
            output.push_str(&"{".repeat(counts-1));
            i += counts;
        } else
        if content.chars().nth(i) == Some('}')
        && content.chars().nth(i+1) == Some('}')
        {
            let mut counts = 2; //starts from 2
            //current char [nth(i)] is "{" and next one [nth(i+1)] as well
            while content.chars().nth(i+counts) == Some('}') {
                counts += 1;
            }
            output.push_str(&"}".repeat(counts-1));
            i += counts;
        } else
        if content.chars().nth(i) == Some('{')
        && content.chars().nth(i+1) != Some('{')
        {
            // if self.content.chars().nth(i) != Some('{') { //skip
            //     output.push(self.content.chars().nth(i).unwrap());
            //     i += 1;
            // } else if self.content.chars().nth(i + 1) == Some('{') {
            //     output.push('{');
            //     i += 2;
            // } else {
            let end = content[i + 1..]
                .find('}')
                .map(|x| x + i + 1)
                .ok_or(PywalTemplateError::MissingVariable(content[i + 1..].to_string()))?;

                let var = &content[i + 1..end];

                let mut parts = var.split('.');
                let name = parts.next().ok_or(PywalTemplateError::MissingVariable(var.to_string()))?;
                let value = t.to_hash();
                let value = value
                    .get(name)
                    .ok_or(PywalTemplateError::MissingVariable(name.to_string()))?;
                let mut output_value = value.to_string();
                //XXX this allows to stack funcs
                for part in parts {
                    // println!("{}", output_value);
                    output_value = get_func(part, value, t.alpha)?;
                }

                output.push_str(&output_value);
                i = end + 1;
        }
        else
        {
            output.push(content.chars().nth(i).unwrap());
            i += 1;
        }
    }//while


    Ok(output)
}

