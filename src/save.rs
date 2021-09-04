use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    future::Future,
    ops::{AddAssign, Deref, DerefMut, MulAssign, RemAssign, SubAssign},
    str::FromStr,
};

use hex2d::Coordinate;

mod storage;
pub use storage::transaction_loop;

fn save(key: impl ToString, value: impl ToString) {
    storage::set(&key.to_string(), &value.to_string())
}

fn load(key: &str) -> impl Future<Output = Option<String>> {
    storage::get(key)
}

pub trait Save {
    fn save(&self, key: impl Display);
    type Load<'a>: Future<Output = ()>;
    fn load<'a>(&'a mut self, key: &str) -> Self::Load<'a>;
}

pub struct Saveable<T> {
    value: T,
    key: String,
}

pub type ComplexSaveable<T> = Saveable<ComplexSave<T>>;

impl<T: Save> Saveable<T> {
    pub async fn new(value: impl Into<T>, key: impl ToString) -> Self {
        let mut this = Self {
            value: value.into(),
            key: key.to_string(),
        };
        this.load().await;
        this
    }

    pub async fn default(key: impl ToString) -> Self
    where
        T: Default,
    {
        Self::new(T::default(), key).await
    }

    fn save(&self) {
        self.value.save(&self.key)
    }

    async fn load(&mut self) {
        self.value.load(&self.key).await
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

    type Load<'a> = impl Future<Output = ()>;
    fn load<'a>(&'a mut self, key: &str) -> Self::Load<'a> {
        let val = load(key);
        async move {
            if let Some(val) = val.await {
                *self = val.parse().unwrap();
            }
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

    type Load<'a> = impl Future<Output = ()>;
    fn load(&mut self, key: &str) -> Self::Load<'_> {
        let x = format!("{}/x", key);
        let y = format!("{}/y", key);
        async move {
            self.x.load(&x).await;
            self.y.load(&y).await;
        }
    }
}
