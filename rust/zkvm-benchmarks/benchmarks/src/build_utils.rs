fn generate_json(target_size: usize, depth: usize) -> String {
    // estimate overhead of nesting
    let mut overhead = 0;
    for lvl in 0..=depth {
        // each nesting adds {"levelN":<…>}
        overhead += format!("{{\"level{lvl}\":").len() + 1;
    }

    // build flat body under (target_size - overhead)
    let limit = target_size.saturating_sub(overhead);
    let mut body = String::with_capacity(limit);
    body.push('{');
    let mut i = 1;
    while body.len() < limit {
        let entry = format!("\"key{i}\":\"value\",");
        if body.len() + entry.len() + 1 > limit {
            break;
        }
        body.push_str(&entry);
        i += 1;
    }
    if body.ends_with(',') {
        body.pop();
    }
    body.push('}');

    // wrap in nested levels
    let mut result = body;
    if depth > 0 {
        for lvl in (0..=depth).rev() {
            result = format!("{{\"level{lvl}\":{result}}}");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_json() {
        let json = generate_json(100, 0);
    }
}
