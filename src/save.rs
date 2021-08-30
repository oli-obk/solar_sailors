use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{AddAssign, Deref, MulAssign, RemAssign, SubAssign},
    str::FromStr,
};

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
        let mut this = Self {
            value,
            key: key.to_string(),
        };
        this.load();
        this
    }

    pub fn default(key: impl ToString) -> Self
    where
        T: Default,
    {
        Self::new(Default::default(), key)
    }

    fn save(&self) {
        self.value.save(&self.key)
    }

    fn load(&mut self) {
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

impl<T: Save + AddAssign> AddAssign<T> for Saveable<T> {
    fn add_assign(&mut self, rhs: T) {
        self.update(|val| *val += rhs)
    }
}

impl<T: Save + MulAssign> MulAssign<T> for Saveable<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.update(|val| *val *= rhs)
    }
}

impl<T: Save + SubAssign> SubAssign<T> for Saveable<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.update(|val| *val -= rhs)
    }
}

impl<T: Save + RemAssign> RemAssign<T> for Saveable<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.update(|val| *val %= rhs)
    }
}

impl<T: PartialEq> PartialEq<T> for Saveable<T> {
    fn eq(&self, other: &T) -> bool {
        self.value.eq(other)
    }
}

impl<T: PartialOrd> PartialOrd<T> for Saveable<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.value.partial_cmp(other)
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
