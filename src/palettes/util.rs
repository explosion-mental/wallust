//! utility to use

pub fn sample_distributed<T: Clone>(v: &[T], amount: usize) -> Vec<T> {
    let n = v.len();
    if amount == 0 || n == 0 { return Vec::new(); }
    if amount >= n { return v.to_vec() }
    // index: % of total length * total length
    // but int division, so changed order of operations
    (0..amount).map(|i| v[i*(n-1)/(amount-1)].clone()).collect()
}

pub fn sample_center<T: Clone>(v: &[T], sample: usize) -> Vec<T> {
    if v.is_empty() || sample == 0 { return v[0..0].to_vec(); }

    let n = v.len();
    let mid = n / 2;
    let half = sample / 2;

    // For even sample sizes, bias slightly right (so 4 elements → 2 after mid, 1 before)
    let start = mid.saturating_sub(half);
    let end = (start + sample).min(n);

    // If we got cut off at the end, shift left to still get `sample` elements when possible
    let start = end.saturating_sub(sample);

    v[start..end].to_vec()
}

/// Return the indices picked by sample_center
pub fn sample_center_idxs<T>(v: &[T], sample: usize) -> Vec<usize> {
    if v.is_empty() || sample == 0 { return [].into(); }

    let n = v.len();
    let mid = n / 2;
    let half = sample / 2;

    // For even sample sizes, bias slightly right (so 4 elements → 2 after mid, 1 before)
    let start = mid.saturating_sub(half);
    let end = (start + sample).min(n);

    // If we got cut off at the end, shift left to still get `sample` elements when possible
    let start = end.saturating_sub(sample);

    (start..end).collect::<Vec<usize>>()
}
