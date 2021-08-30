use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{AddAssign, Deref, DerefMut, MulAssign, RemAssign, SubAssign},
    str::FromStr,
};

use hex2d::Coordinate;

mod storage;

pub fn save(key: impl ToString, value: impl ToString) {
    storage::set(&key.to_string(), &value.to_string())
}

pub fn load<T: FromStr>(key: impl ToString) -> Option<T>
where
    T::Err: Debug,
{
    try_load(key).map(Result::unwrap)
}

pub fn try_load<T: FromStr>(key: impl ToString) -> Option<Result<T, T::Err>> {
    storage::get(&key.to_string())
        .map(|s| s.parse())
}

pub trait Save {
    fn save(&self, key: impl Display);
    fn load(&mut self, key: impl Display);
}

pub struct Saveable<T> {
    value: T,
    key: String,
}

pub type ComplexSaveable<T> = Saveable<ComplexSave<T>>;

impl<T: Save> Saveable<T> {
    pub fn new(value: impl Into<T>, key: impl ToString) -> Self {
        let mut this = Self {
            value: value.into(),
            key: key.to_string(),
        };
        this.load();
        this
    }

    pub fn default(key: impl ToString) -> Self
    where
        T: Default,
    {
        Self::new(T::default(), key)
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
    pub fn set(&mut self, t: impl Into<T>) {
        self.update(|v| *v = t.into());
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

pub struct ComplexSave<T>(T);

impl<T> From<T> for ComplexSave<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> Deref for ComplexSave<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for ComplexSave<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Save for ComplexSave<Coordinate> {
    fn save(&self, key: impl Display) {
        self.x.save(format_args!("{}/x", key));
        self.y.save(format_args!("{}/y", key));
    }

    fn load(&mut self, key: impl Display) {
        self.x.load(format_args!("{}/x", key));
        self.y.load(format_args!("{}/y", key));
    }
}
