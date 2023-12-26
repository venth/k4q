pub(crate) mod option;

/// Use for compile time checking, whether a function and its argument are
/// prepared for async.
/// Example:
/// ```
/// async fn some_func(foo: Foo) -> Value { todo!() }
///
/// fn _some_func_is_prepared_for_async() {
///     _is_prepared_for_async(some_func(Foo:default()));
/// }
/// ```
pub(crate) fn _is_prepared_for_async<T: Send>(_: T) {}
