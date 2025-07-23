pub fn parse_blocklist(domains: &[String]) -> Vec<String> {
    domains.iter()
        .filter(|d| !d.is_empty() && !d.starts_with('#'))
        .cloned()
        .collect()
}
