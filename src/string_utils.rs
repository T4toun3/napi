use std::str::FromStr;

pub trait StringUtils {
    fn before(&self, string: &str) -> Option<&str>;

    fn after(&self, string: &str) -> Option<&str>;

    fn between(&self, a: &str, b: &str) -> Option<&str>;

    fn after_take(&self, string: &str, i: usize) -> Option<&str>;

    fn before_take(&self, string: &str, i: usize) -> Option<&str>;

    fn to_type<T: FromStr>(&self) -> Option<T>;

    fn trim_(&self) -> Option<&str>;

    fn replace_(&self, a: &str, b: &str) -> Option<String>;
}

impl StringUtils for String {
    fn before(&self, string: &str) -> Option<&str> {
        Some(
            &self[0..if let Some(e) = self.find(string) {
                e
            } else {
                self.len()
            }],
        )
    }

    fn after(&self, string: &str) -> Option<&str> {
        Some(&self[self.find(string)? + string.len()..self.len()])
    }

    fn after_take(&self, string: &str, i: usize) -> Option<&str> {
        let p = self.find(string)? + string.len();
        Some(&self[p..p + i])
    }

    fn before_take(&self, string: &str, i: usize) -> Option<&str> {
        let p = self.find(string)?;
        Some(&self[p - i..p])
    }

    fn between(&self, a: &str, b: &str) -> Option<&str> {
        Some(&self[self.find(a)? + a.len()..self.find(b)?])
    }

    fn to_type<T: FromStr>(&self) -> Option<T> {
        self.parse().ok()
    }

    fn trim_(&self) -> Option<&str> {
        Some(self.trim())
    }

    fn replace_(&self, a: &str, b: &str) -> Option<String> {
        Some(self.replace(a, b))
    }
}

impl StringUtils for &str {
    fn before(&self, string: &str) -> Option<&str> {
        Some(
            &self[0..if let Some(e) = self.find(string) {
                e
            } else {
                self.len()
            }],
        )
    }

    fn after(&self, string: &str) -> Option<&str> {
        Some(&self[self.find(string)? + string.len()..self.len()])
    }

    fn after_take(&self, string: &str, i: usize) -> Option<&str> {
        let p = self.find(string)? + string.len();
        Some(&self[p..p + i])
    }

    fn before_take(&self, string: &str, i: usize) -> Option<&str> {
        let p = self.find(string)?;
        Some(&self[p - i..p])
    }

    fn between(&self, a: &str, b: &str) -> Option<&str> {
        Some(&self[self.find(a)? + a.len()..self.find(b)?])
    }

    fn to_type<T: FromStr>(&self) -> Option<T> {
        self.parse().ok()
    }

    fn trim_(&self) -> Option<&str> {
        Some(self.trim())
    }

    fn replace_(&self, a: &str, b: &str) -> Option<String> {
        Some(self.replace(a, b))
    }
}

impl StringUtils for Option<String> {
    fn before(&self, string: &str) -> Option<&str> {
        if let Some(e) = self {
            Some(
                &e[0..if let Some(e) = e.find(string) {
                    e
                } else {
                    e.len()
                }],
            )
        } else {
            None
        }
    }

    fn after(&self, string: &str) -> Option<&str> {
        if let Some(e) = self {
            Some(&e[e.find(string)? + string.len()..e.len()])
        } else {
            None
        }
    }

    fn after_take(&self, string: &str, i: usize) -> Option<&str> {
        if let Some(e) = self {
            let p = e.find(string)? + string.len();
            Some(&e[p..p + i])
        } else {
            None
        }
    }

    fn before_take(&self, string: &str, i: usize) -> Option<&str> {
        if let Some(e) = self {
            let p = e.find(string)?;
            Some(&e[p - i..p])
        } else {
            None
        }
    }

    fn between(&self, a: &str, b: &str) -> Option<&str> {
        if let Some(e) = self {
            Some(&e[e.find(a)? + a.len()..e.find(b)?])
        } else {
            None
        }
    }

    fn to_type<T: FromStr>(&self) -> Option<T> {
        if let Some(e) = self {
            e.parse().ok()
        } else {
            None
        }
    }

    fn trim_(&self) -> Option<&str> {
        if let Some(e) = self {
            Some(e.trim())
        } else {
            None
        }
    }

    fn replace_(&self, a: &str, b: &str) -> Option<String> {
        if let Some(e) = self {
            Some(e.replace(a, b))
        } else {
            None
        }
    }
}

impl StringUtils for Option<&str> {
    fn before(&self, string: &str) -> Option<&str> {
        if let Some(e) = self {
            Some(
                &e[0..if let Some(e) = e.find(string) {
                    e
                } else {
                    e.len()
                }],
            )
        } else {
            None
        }
    }

    fn after(&self, string: &str) -> Option<&str> {
        if let Some(e) = self {
            Some(&e[e.find(string)? + string.len()..e.len()])
        } else {
            None
        }
    }

    fn after_take(&self, string: &str, i: usize) -> Option<&str> {
        if let Some(e) = self {
            let p = e.find(string)? + string.len();
            Some(&e[p..p + i])
        } else {
            None
        }
    }

    fn before_take(&self, string: &str, i: usize) -> Option<&str> {
        if let Some(e) = self {
            let p = e.find(string)?;
            Some(&e[p - i..p])
        } else {
            None
        }
    }

    fn between(&self, a: &str, b: &str) -> Option<&str> {
        if let Some(e) = self {
            Some(&e[e.find(a)? + a.len()..e.find(b)?])
        } else {
            None
        }
    }

    fn to_type<T: FromStr>(&self) -> Option<T> {
        if let Some(e) = self {
            e.parse().ok()
        } else {
            None
        }
    }

    fn trim_(&self) -> Option<&str> {
        if let Some(e) = self {
            Some(e.trim())
        } else {
            None
        }
    }

    fn replace_(&self, a: &str, b: &str) -> Option<String> {
        if let Some(e) = self {
            Some(e.replace(a, b))
        } else {
            None
        }
    }
}
