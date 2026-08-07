pub(in crate::service) fn parse_version(value: &str) -> Option<Vec<u64>> {
    let trimmed = value.trim().trim_start_matches(['v', 'V']);
    let core = trimmed.split(['-', '+']).next()?;
    if core.is_empty() {
        return None;
    }
    let parts: Option<Vec<u64>> = core.split('.').map(|part| part.parse::<u64>().ok()).collect();
    parts.filter(|values| !values.is_empty())
}

pub(in crate::service) fn is_newer(candidate: &str, current: &str) -> bool {
    let Some(candidate) = parse_version(candidate) else {
        return false;
    };
    let Some(current) = parse_version(current) else {
        return false;
    };
    let length = candidate.len().max(current.len());
    for index in 0..length {
        let left = candidate.get(index).copied().unwrap_or(0);
        let right = current.get(index).copied().unwrap_or(0);
        if left != right {
            return left > right;
        }
    }
    false
}
