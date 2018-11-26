use crate::Result;
use argon2::{hash_encoded, verify_encoded, Config, ThreadMode, Variant, Version};
use rand::Rng;
use std::borrow::Cow;
use std::convert::{AsMut, AsRef};

#[derive(Debug, Clone, Hash)]
pub struct Secret<'s>(Cow<'s, str>);

impl<'s> From<&'s str> for Secret<'s> {
    fn from(s: &'s str) -> Secret<'s> {
        Secret(Cow::Borrowed(s))
    }
}

static CONFIG: Config = Config {
    variant: Variant::Argon2id,
    version: Version::Version13,
    mem_cost: 65536,
    time_cost: 10,
    lanes: 4,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 64,
};

impl<'s> Secret<'s> {
    pub fn new<A: AsMut<[u8]>>(mut data: A) -> Result<Secret<'s>> {
        let mut salt: [u8; 64] = [0_u8; 64];
        rand::thread_rng().fill(&mut salt);
        let secret = Secret(Cow::Owned(hash_encoded(data.as_mut(), &salt, &CONFIG)?));
        for m in data.as_mut().iter_mut() {
            *m = 0;
        }
        Ok(secret)
    }

    pub fn verify<A: AsRef<[u8]>>(&self, given: A) -> Result<bool> {
        verify_encoded(&self.0, given.as_ref()).map_err(|e| e.into())
    }
}

impl<'s> AsRef<str> for Secret<'s> {
    fn as_ref(&self) -> &str { self.0.as_ref() }
}
