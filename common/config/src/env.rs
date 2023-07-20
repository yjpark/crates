use std::ffi::OsStr;


pub struct EnvInjector {
    pub prefix: String,
}

impl EnvInjector {
    pub fn inject<V>(&self, key: &str, value: V)
        where V: AsRef<OsStr>,
    {
        let key_with_prefix = format!("{}{}", self.prefix, key);
        std::env::set_var(key_with_prefix, value);
    }
}