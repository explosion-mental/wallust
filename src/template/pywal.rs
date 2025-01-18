use thiserror::Error;

//use crate::colors::Colors;
use crate::colors::Myrgb;
use palette::Srgb;

use super::TemplateFields;
use super::alpha_hexa;

#[derive(Error, Debug)]
pub enum PywalTemplateError {
    #[error("Missing variable: {0}")]
    MissingVariable(String),
    #[error("Invalid modifier: {0}")]
    InvalidModifier(String),
}

fn get_func(fname: &str, value: &str) -> Result<String, PywalTemplateError> {

    let c: Srgb<u8> = value.parse().expect("SHOULD BE A VALID COLOR");
    let c: Myrgb = c.into();
    let alpha = 100.0;

    let ret = match fname {
        "rgb" => c.rgb(),
        "rgba" => c.rgba(alpha),
        "xrgba" => c.xrgba(&alpha_hexa(alpha as usize).unwrap()),
        "strip" => c.strip(),
        "red" => c.red(),
        "green" => c.green(),
        "blue" => c.blue(),
        "alpha" => format!("[{}]{c}", alpha as usize),
        "alpha_dec" => format!("{:.2}", alpha / 100.0),
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
                    output_value = get_func(part, value)?;
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

