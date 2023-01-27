use rand::{self, Rng};
use rocket::request::FromParam;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};


#[derive(UriDisplayPath)]
pub struct PashaId<'a>(Cow<'a, str>);

impl PashaId<'_> {
    pub fn new(size: usize) -> PashaId<'static> {
        const BASE62: &[u8] = b"0123456789abcdefghijklmonpqrstuvwxyz";
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PashaId(Cow::Owned(id))
    }

    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");
        Path::new(root).join(self.0.as_ref())
    }
}

impl<'a> FromParam<'a> for PashaId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
            .then(|| PashaId(param.into()))
            .ok_or(param)
    }
}
