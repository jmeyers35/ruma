/// Check whether a given string is a valid server name according to [the specification][].
///
/// [the specification]: https://matrix.org/docs/spec/appendices#server-name
pub fn is_valid_server_name(name: &str) -> bool {
    use std::net::Ipv6Addr;

    if name.is_empty() {
        return false;
    }

    let end_of_host = if name.starts_with('[') {
        let end_of_ipv6 = match name.find(']') {
            Some(idx) => idx,
            None => return false,
        };

        if name[1..end_of_ipv6].parse::<Ipv6Addr>().is_err() {
            return false;
        }

        end_of_ipv6 + 1
    } else {
        let end_of_host = name.find(':').unwrap_or_else(|| name.len());

        if name[..end_of_host]
            .bytes()
            .any(|byte| !(byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'.'))
        {
            return false;
        }

        end_of_host
    };

    if name.len() == end_of_host {
        true
    } else if name.as_bytes()[end_of_host] != b':' {
        // hostname is followed by something other than ":port"
        false
    } else {
        // are the remaining characters after ':' a valid port?
        name[end_of_host + 1..].parse::<u16>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::is_valid_server_name;

    #[test]
    fn ipv4_host() {
        assert!(is_valid_server_name("127.0.0.1"));
    }

    #[test]
    fn ipv4_host_and_port() {
        assert!(is_valid_server_name("1.1.1.1:12000"));
    }

    #[test]
    fn ipv6() {
        assert!(is_valid_server_name("[::1]"));
    }

    #[test]
    fn ipv6_with_port() {
        assert!(is_valid_server_name("[1234:5678::abcd]:5678"));
    }

    #[test]
    fn dns_name() {
        assert!(is_valid_server_name("example.com"));
    }

    #[test]
    fn dns_name_with_port() {
        assert!(is_valid_server_name("ruma.io:8080"));
    }

    #[test]
    fn empty_string() {
        assert!(!is_valid_server_name(""));
    }

    #[test]
    fn invalid_ipv6() {
        assert!(!is_valid_server_name("[test::1]"));
    }

    #[test]
    fn ipv4_with_invalid_port() {
        assert!(!is_valid_server_name("127.0.0.1:"));
    }

    #[test]
    fn ipv6_with_invalid_port() {
        assert!(!is_valid_server_name("[fe80::1]:100000"));
        assert!(!is_valid_server_name("[fe80::1]!"));
    }

    #[test]
    fn dns_name_with_invalid_port() {
        assert!(!is_valid_server_name("matrix.org:hello"));
    }
}
