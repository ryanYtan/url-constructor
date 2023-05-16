use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct UrlConstructor {
    scheme: String,
    subdomains: Vec<String>,
    userinfo: Option<String>,
    host: String,
    port: Option<u16>,
    subdirs: Vec<String>,
    params: BTreeMap<String, String>, //ordered keys for determinism
    fragment: Option<String>
}

impl UrlConstructor {
    pub fn new() -> Self {
        Self {
            scheme: "https".to_owned(),
            subdomains: Vec::new(),
            userinfo: None,
            host: String::new(),
            port: None,
            subdirs: Vec::new(),
            params: BTreeMap::new(),
            fragment: None,
        }
    }

    pub fn scheme<S>(&mut self, scheme: S) -> &mut Self
        where S: Into<String>
    {
        self.scheme = scheme.into();
        self
    }

    pub fn userinfo<S>(&mut self, userinfo: S) -> &mut Self
        where S: Into<String>
    {
        self.userinfo = Some(userinfo.into());
        self
    }

    pub fn host<S>(&mut self, host: S) -> &mut Self
        where S: Into<String>
    {
        let host_s: String = host.into();
        self.host = host_s;
        self
    }

    /// Subdomains will appear left-to-right in the calling order in the final
    /// URL e.g. calling `.host("google.com").subdomain("api").subdomain("v2")`
    /// will be built as `api.v2.google.com`.
    pub fn subdomain<S>(&mut self, subdomain: S) -> &mut Self
        where S: Into<String>
    {
        self.subdomains.push(subdomain.into());
        self
    }

    pub fn port(&mut self, port: u16) -> &mut Self
    {
        self.port = Some(port);
        self
    }

    pub fn subdir<S>(&mut self, subdir: S) -> &mut Self
        where S: Into<String>
    {
        self.subdirs.push(subdir.into());
        self
    }

    pub fn param<S1, S2>(&mut self, key: S1, value: S2) -> &mut Self
        where
            S1: Into<String>,
            S2: Into<String>
    {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn fragment<S>(&mut self, fragment: S) -> &mut Self
        where S: Into<String>
    {
        self.fragment = Some(fragment.into());
        self
    }

    pub fn build(&self) -> String {
        let scheme_s = if self.scheme.is_empty() {
            "".to_owned()
        } else {
            self.scheme.clone() + "://"
        };

        let userinfo_s = match &self.userinfo {
            Some(v) => v.clone() + "@",
            None => String::new(),
        };

        let subdomains_s = self
            .subdomains
            .iter()
            .cloned()
            .reduce(|a, b| a + "." + &b)
            .map(|s| if self.host.is_empty() { s } else { s + "." })
            .or(Some(String::new()))
            .unwrap();

        let port_s = match self.port {
            Some(num) => ":".to_owned() + &num.to_string(),
            None => String::new(),
        };

        let subdirs_s = self
            .subdirs
            .iter()
            .cloned()
            .reduce(|a, b| a + "/" + &b)
            .map(|s| "/".to_owned() + &s)
            .or(Some(String::new()))
            .unwrap();

        let params_s = self
            .params
            .clone()
            .into_iter()
            .map(|(k, v)| k + "=" + &v)
            .reduce(|p1, p2| p1 + "&" + &p2)
            .map(|s| "?".to_owned() + &s)
            .or(Some(String::new()))
            .unwrap();

        let fragment_s = match &self.fragment {
            Some(v) => "#".to_owned() + v,
            None => String::new(),
        };

        scheme_s
            + &userinfo_s
            + &subdomains_s
            + &self.host
            + &port_s
            + &subdirs_s
            + &params_s
            + &fragment_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static HOST: &str = "google.com";

    #[test]
    fn builder_readme_example() {
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
            .build();

        assert_eq!(
            url,
            "http://alex:password1@api.google.com:8080/v2/users?lastName=Wallace&salary=>10000#id"
        )
    }

    #[test]
    fn builder_normal_usage() {
        let actual = UrlConstructor::new()
            .scheme("http")
            .userinfo("user:password")
            .subdomain("api")
            .subdomain("v2")
            .host(HOST)
            .port(400)
            .subdir("s1")
            .subdir("s2")
            .param("k1", "v1")
            .param("k2", "v2")
            .param("k3", "v4")
            .fragment("foo")
            .build();
        let expected = format!("http://user:password@api.v2.{}:400/s1/s2?k1=v1&k2=v2&k3=v4#foo", HOST.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_empty() {
        let actual = UrlConstructor::new().scheme("").build();
        let expected = "";
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_scheme() {
        let actual = UrlConstructor::new()
            .scheme("ssh")
            .build();
        let expected = format!("ssh://");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_no_scheme() {
        let actual = UrlConstructor::new()
            .scheme("")
            .build();
        assert_eq!("", actual);
    }

    #[test]
    fn test_builder_userinfo() {
        let actual = UrlConstructor::new()
            .userinfo("user:pass")
            .build();
        assert_eq!("https://user:pass@", actual);
    }

    #[test]
    fn test_builder_userinfo_host() {
        let actual = UrlConstructor::new()
            .userinfo("user:pass")
            .host(HOST)
            .build();
        let expected = format!("https://user:pass@{}", HOST);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_no_scheme_with_host() {
        let actual = UrlConstructor::new()
            .scheme("")
            .host(HOST)
            .build();
        assert_eq!(HOST, actual);
    }

    #[test]
    fn test_builder_host() {
        let actual = UrlConstructor::new()
            .host(HOST)
            .build();
        let expected = format!("https://{}", HOST);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_subdomains() {
        let actual = UrlConstructor::new()
            .subdomain("api")
            .subdomain("google")
            .subdomain("com")
            .build();
        let expected = format!("https://api.google.com");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_host_subdomains() {
        let actual = UrlConstructor::new()
            .host("google.com")
            .subdomain("api")
            .subdomain("v2")
            .build();
        let expected = format!("https://api.v2.google.com");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_port() {
        let actual = UrlConstructor::new()
            .port(443)
            .build();
        let expected = format!("https://:443");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_host_port() {
        let actual = UrlConstructor::new()
            .host(HOST)
            .port(443)
            .build();
        let expected = format!("https://{}:443", HOST);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_subdirs() {
        let actual = UrlConstructor::new()
            .subdir("s1")
            .subdir("s2")
            .build();
        let expected = format!("https:///s1/s2");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_host_subdirs() {
        let actual = UrlConstructor::new()
            .host(HOST)
            .subdir("s1")
            .subdir("s2")
            .build();
        let expected = format!("https://{}/s1/s2", HOST);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_host_params() {
        let actual = UrlConstructor::new()
            .host(HOST)
            .param("k1", "v1")
            .param("k2", "v2")
            .param("k3", "v4")
            .build();
        let expected = format!("https://{}?k1=v1&k2=v2&k3=v4", HOST.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_params() {
        let actual = UrlConstructor::new()
            .param("k1", "v1")
            .param("k2", "v2")
            .param("k3", "v4")
            .build();
        let expected = format!("https://?k1=v1&k2=v2&k3=v4");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_builder_fragment() {
        let actual = UrlConstructor::new()
            .fragment("foo")
            .build();
        let expected = format!("https://#foo");
        assert_eq!(expected, actual);
    }
}
