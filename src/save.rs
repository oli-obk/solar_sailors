use std::{fmt::Debug, str::FromStr};

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
