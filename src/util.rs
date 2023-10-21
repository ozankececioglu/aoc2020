#![allow(warnings)]


pub fn type_of<T>(_: &T) -> &'static str { return std::any::type_name::<T>() }
pub fn size_of<T>(_: &T) -> usize { return std::mem::size_of::<T>() }


#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

