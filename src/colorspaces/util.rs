//! Helper functions. Mostly unrealted to colors.

pub fn avg(i: &[f32]) -> f32 { i.iter().sum::<f32>() / i.len() as f32 }
