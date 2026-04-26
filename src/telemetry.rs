/// Strip the query string from any URLs embedded in `input`.
///
/// Keeps the scheme/host/path so we can still tell which provider failed,
/// but drops `?key=value&...` where Overpass and elevation provider URLs
/// carry bbox coordinates. Non-URL `?` (e.g. in English text) is preserved
/// because we only strip after a `://` scheme.
pub fn redact_url_queries(mut input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    while let Some(scheme_idx) = input.find("://") {
        let (before, after) = input.split_at(scheme_idx);
        out.push_str(before);
        out.push_str("://");
        let after_scheme = &after[3..];
        // Terminators: whitespace and `)` only. `,` is valid inside URL
        // queries (raw bbox coords in tile-service URLs) so stripping on
        // it would leak the tail of the query past the comma.
        let url_end = after_scheme
            .find(|c: char| c.is_whitespace() || c == ')')
            .unwrap_or(after_scheme.len());
        let url = &after_scheme[..url_end];
        let tail = &after_scheme[url_end..];
        match url.find('?') {
            Some(q) => out.push_str(&url[..q]),
            None => out.push_str(url),
        }
        input = tail;
    }
    out.push_str(input);
    out
}

// ---------------------------------------------------------------------------
// Stub API surface — telemetry is disabled; these no-ops keep callers
// compiling without changes.
// ---------------------------------------------------------------------------

/// Log levels (retained for caller compatibility).
#[allow(dead_code)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// No-op: telemetry is disabled.
#[allow(unused_variables)]
pub fn send_log(_level: LogLevel, _message: &str) {}

/// No-op: telemetry is disabled.
pub fn send_generation_click() {}

/// No-op: telemetry consent is unused.
#[allow(unused_variables)]
pub fn set_telemetry_consent(_consent: bool) {}

/// Installs a panic hook that logs panics to stderr (no remote reporting).
pub fn install_panic_hook() {
    use log::error;
    std::panic::set_hook(Box::new(|panic_info| {
        error!("Application panicked: {:?}", panic_info);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_strips_overpass_query() {
        let input = "error sending request for url (https://overpass-api.de/api/interpreter?data=%5Bbbox%3A50.1%2C13.7%5D): connection timed out";
        let out = redact_url_queries(input);
        assert!(!out.contains("bbox"), "bbox leaked: {out}");
        assert!(!out.contains("?"), "query not stripped: {out}");
        assert!(out.contains("overpass-api.de"), "host lost: {out}");
        assert!(out.ends_with("): connection timed out"));
    }

    #[test]
    fn redact_preserves_non_url_question_marks() {
        let input = "What? That is odd.";
        assert_eq!(redact_url_queries(input), input);
    }

    #[test]
    fn redact_handles_multiple_urls_and_no_query() {
        let input = "first https://a.com/p?x=1 middle https://b.com/q end";
        assert_eq!(
            redact_url_queries(input),
            "first https://a.com/p middle https://b.com/q end"
        );
    }

    #[test]
    fn redact_handles_truncated_tail() {
        let input = "error for url https://host.tld/path?bbox=50.1,13.7";
        assert_eq!(
            redact_url_queries(input),
            "error for url https://host.tld/path"
        );
    }
}
