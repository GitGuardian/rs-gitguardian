<a href="https://gitguardian.com/"><img src="https://cdn.jsdelivr.net/gh/gitguardian/py-gitguardian/doc/logo.svg"></a>

# [rs-gitguardian](https://github.com/GitGuardian/rs-gitguardian) - GitGuardian API Client

[![Crates.io](https://img.shields.io/crates/v/gitguardian-client?color=%231B2D55&style=for-the-badge)](https://crates.io/crates/gitguardian-client)
[![License](https://img.shields.io/github/license/GitGuardian/rs-gitguardian?color=%231B2D55&style=for-the-badge)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/gitguardian/rs-gitguardian?color=%231B2D55&style=for-the-badge)](https://github.com/GitGuardian/rs-gitguardian/stargazers)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/GitGuardian/rs-gitguardian/test-lint.yml?branch=master&style=for-the-badge)
[![Codecov](https://img.shields.io/codecov/c/github/GitGuardian/rs-gitguardian?style=for-the-badge)](https://codecov.io/gh/GitGuardian/rs-gitguardian/)

API client library for the [GitGuardian API](https://api.gitguardian.com/).

The GitGuardian API puts at your fingertips the power to detect more than 200 types of secrets in any text content, as well as other potential security vulnerabilities.

**rs-gitguardian** can be used to create integrations to scan various data sources, from your workstation's filesystem to your favorite chat application.

You can check API details [here](https://api.gitguardian.com/docs)
with all the response codes and expected structures on each method.

## Requirements

Rust 1.88+

## Getting started

You can obtain API keys for API usage on your [dashboard](https://dashboard.gitguardian.com/api/v1/auth/user/github_login/authorize?utm_source=github&utm_medium=rs_gitguardian&utm_campaign=rs1).

**cargo**

```bash
cargo add gitguardian-client
```

The HTTP client uses `ureq` by default. To use `reqwest` as an HTTP client, add the crate with

```bash
cargo add gitguardian-client --no-default-features --features reqwest
```

## Examples

### Scanning text content

```rust
// please don't hardcode your gg_api_key in source code :)
let api_key = env::var("GITGUARDIAN_API_KEY")?;
let document = "
    import urllib.request
    url = 'http://jen_barber:correcthorsebatterystaple@cake.gitguardian.com/isreal.json'
    response = urllib.request.urlopen(url)
    consume(response.read())
";

let client = Client::new(ApiConfig::new(&api_key)?);

match client.send(&Scan::new(Document::new(document))) {
    Ok(scan_result) => println!("{} policy breaks", scan_result.policy_break_count),
    Err(error) => eprintln!("{error}"),
}
```

### Scanning multiple files

```rust
let api_key = env::var("GITGUARDIAN_API_KEY")?;
let client = Client::new(ApiConfig::new(&api_key)?);

// Create a list of documents for scanning
let to_scan: Vec<Document> = paths
    .iter()
    .map(|path| {
        let content = fs::read(path).unwrap_or_default();
        Document::new(String::from_utf8_lossy(&content))
            .with_filename(path.file_name().unwrap_or_default().to_string_lossy())
    })
    .collect();

let scan = client.send(&MultiScan::new(to_scan))?;
```

### Dependencies

rs-gitguardian depends on these excellent libraries:

- `http` - Request and response types
- `serde` - Request (de)serialization
- `chrono` - Timestamps
- `uuid` - Identifiers
- `reqwest` - Asynchronous HTTP client
- `ureq` - Blocking HTTP client
