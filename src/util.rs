fn type_of<T>(_: &T) -> &'static str { return std::any::type_name::<T>() }
fn size_of<T>(_: &T) -> usize { return std::mem::size_of::<T>() }
