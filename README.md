# Overview
A simple URL constructor with a bit more customizability.

If you do use this library, note that it does _not_ do _any_ error handling nor
sanitation of inputs. Do verify the inputs before passing them to the builder
(and experiment with the output).

## Quick Start
In `Cargo.toml`:
```
[dependencies]
url-constructor = "0.1.0"
```
To create a URL:
```rust
use url_constructor::UrlConstructor;

let url = UrlConstructor::new()
    .scheme("http")
    .userinfo("alex:password1")
    .subdomain("api")
    .host("google.com")
    .port(8080)
    .subdir("v2")
    .subdir("users")
    .param("salary", ">10000")
    .param("lastName", "Wallace")
    .fragment("id")
    .build()

assert_eq!(
    url,
    "http://alex:password1@api.google.com:8080/v2/users?lastName=Wallace&salary=>10000#id"
)
```

## Specifications
- If the scheme is set to the empty string,
