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
These are the current "quirks" of the constructor
- The default scheme is `https`, there are no defaults for the other components
- If the scheme is set to the empty string, then the `://` part of the URL will not be returned
- Subdomains are returned left-to-right according to the order in which it is called (see example above)
- Control characters such as `&`, `?`, `/`, `#` and `@` are automatically added to their respectively URL components (**note**: no checks are made to prevent duplicates)
- Defining each part of the host using the `subdomain` method should yield the same result as using `host` directly
- Multiplicity of each component:
  - _scheme_: 0...1
  - _userinfo_: 0...1
  - _subdomain_: 0...*
  - _host_: 0...1
  - _port_: 0...1
  - _subdir_: 0...*
  - _param_: 0...*
  - _fragment_: 0...*
