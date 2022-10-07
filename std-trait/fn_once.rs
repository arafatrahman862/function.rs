/// The version of the call operator that takes a by-value receiver.
///
/// Instances of `FnOnce` can be called, but might not be callable multiple
/// times. Because of this, if the only thing known about a type is that it
/// implements `FnOnce`, it can only be called once.
///
/// `FnOnce` is implemented automatically by closures that might consume captured
/// variables, as well as all types that implement [`FnMut`], e.g., (safe)
/// [function pointers] (since `FnOnce` is a supertrait of [`FnMut`]).
///
/// Since both [`Fn`] and [`FnMut`] are subtraits of `FnOnce`, any instance of
/// [`Fn`] or [`FnMut`] can be used where a `FnOnce` is expected.
///
/// Use `FnOnce` as a bound when you want to accept a parameter of function-like
/// type and only need to call it once. If you need to call the parameter
/// repeatedly, use [`FnMut`] as a bound; if you also need it to not mutate
/// state, use [`Fn`].
///
/// See the [chapter on closures in *The Rust Programming Language*][book] for
/// some more information on this topic.
///
/// Also of note is the special syntax for `Fn` traits (e.g.
/// `Fn(usize, bool) -> usize`). Those interested in the technical details of
/// this can refer to [the relevant section in the *Rustonomicon*][nomicon].
///
/// [book]: ../../book/ch13-01-closures.html
/// [function pointers]: fn
/// [nomicon]: ../../nomicon/hrtb.html
///
/// # Examples
///
/// ## Using a `FnOnce` parameter
///
/// ```
/// fn consume_with_relish<F>(func: F)
/// where
///     F: std_trait::FnOnce<(), Output = String>
/// {
///     // `func` consumes its captured variables, so it cannot be run more than once.
///     println!("Consumed: {}", func.call_once(()));
///     println!("Delicious!");
///
///     // Attempting to invoke `func()` again will throw a `use of moved value` error for `func`.
/// }
///
/// let x = String::from("x");
/// let consume_and_return_x = move || x;
/// consume_with_relish(consume_and_return_x);
///
/// // `consume_and_return_x` can no longer be invoked at this point
/// ```
pub trait FnOnce<Args> {
    /// The returned type after the call operator is used.
    type Output;

    /// Performs the call operation.
    fn call_once(self, args: Args) -> Self::Output;
}

// use core::ops::FnOnce;
macro_rules! impl_for_typles {
    [$(($($i: tt; $ty: ident),*)),*]  => ($(
        impl<Func, Ret, $($ty,)*> FnOnce<($($ty,)*)> for Func
        where
            Func: core::ops::FnOnce($($ty),*) -> Ret,
        {
            type Output = Ret;
            #[inline] fn call_once(self, _args: ($($ty,)*)) -> Self::Output {
                self($(_args.$i),*)
            }
        }
    )*);
}

impl_for_typles!(
    (),
    (0; T0),
    (0; T0, 1; T1),
    (0; T0, 1; T1, 2; T2),
    (0; T0, 1; T1, 2; T2, 3; T3),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11, 12; T12),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11, 12; T12, 13; T13),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11, 12; T12, 13; T13, 14; T14),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11, 12; T12, 13; T13, 14; T14, 15; T15)
);
