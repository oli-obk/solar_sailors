use std::{fmt::Debug, ops::Deref, str::FromStr};

pub fn save(key: impl ToString, value: impl ToString) {
    quad_storage::STORAGE
        .lock()
        .unwrap()
        .set(&key.to_string(), &value.to_string())
}

pub fn load<T: FromStr>(key: impl ToString) -> Option<T>
where
    T::Err: Debug,
{
    try_load(key).map(Result::unwrap)
}

pub fn try_load<T: FromStr>(key: impl ToString) -> Option<Result<T, T::Err>> {
    quad_storage::STORAGE
        .lock()
        .unwrap()
        .get(&key.to_string())
        .map(|s| s.parse())
}

pub trait Save {
    fn save(&self, key: impl ToString);
    fn load(&mut self, key: impl ToString);
}

pub struct Saveable<T> {
    value: T,
    key: String,
}

impl<T: Save> Saveable<T> {
    pub fn new(value: T, key: impl ToString) -> Self {
        Self {
            value,
            key: key.to_string(),
        }
    }

    pub fn default(key: impl ToString) -> Self
    where
        T: Default,
    {
        let mut this = Self::new(Default::default(), key);
        this.load();
        this
    }

    pub fn save(&self) {
        self.value.save(&self.key)
    }
    pub fn load(&mut self) {
        self.value.load(&self.key)
    }
    pub fn update(&mut self, f: impl FnOnce(&mut T)) {
        f(&mut self.value);
        self.save()
    }
}

impl<T> Deref for Saveable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: FromStr> Save for T
where
    for<'a> &'a T: ToString,
    T::Err: Debug,
{
    fn save(&self, key: impl ToString) {
        save(key, self)
    }

    fn load(&mut self, key: impl ToString) {
        if let Some(val) = load(key) {
            *self = val;
        }
    }
}
