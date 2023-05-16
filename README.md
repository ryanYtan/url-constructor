# Overview
A simple URL builder/creator that's a bit more general.

This library does _not_ do _any_ error handling nor sanitation of inputs. Do
verify the inputs before passing them to the builder (and experiment with the
output).

## Quick Start
In `Cargo.toml`:
```
[dependencies]
= "0.1.0" //TODO add package name
```
To create a URL:
```rust
use ::UrlBuilder; //TODO add package name

let url = UrlBuilder::new()
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
