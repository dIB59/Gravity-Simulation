//! Parallel-iterator shim that picks rayon or std iteration based on the
//! `parallel` feature. Lets the n-body simulation code use one set of method
//! names whether we're compiling for native (with rayon threads) or for
//! `wasm32-unknown-unknown` without atomics (single-threaded fallback).
//!
//! In a follow-up, this will be replaced with `wasm-bindgen-rayon` so the
//! parallel path stays active even on WASM (via Web Workers).

#[cfg(feature = "parallel")]
pub use rayon::prelude::*;

// ─── Parallel build: thin wrappers that delegate to rayon. ───────────────────
#[cfg(feature = "parallel")]
pub trait IntoMaybePar {
    type Iter: ParallelIterator;
    fn into_maybe_par_iter(self) -> Self::Iter;
}
#[cfg(feature = "parallel")]
impl<T: IntoParallelIterator> IntoMaybePar for T {
    type Iter = T::Iter;
    fn into_maybe_par_iter(self) -> Self::Iter {
        self.into_par_iter()
    }
}

#[cfg(feature = "parallel")]
pub trait MaybeParRef<'a> {
    type Iter: ParallelIterator;
    fn maybe_par_iter(self) -> Self::Iter;
}
#[cfg(feature = "parallel")]
impl<'a, T> MaybeParRef<'a> for &'a [T]
where
    T: Sync + 'a,
{
    type Iter = rayon::slice::Iter<'a, T>;
    fn maybe_par_iter(self) -> Self::Iter {
        self.par_iter()
    }
}
#[cfg(feature = "parallel")]
impl<'a, T> MaybeParRef<'a> for &'a Vec<T>
where
    T: Sync + 'a,
{
    type Iter = rayon::slice::Iter<'a, T>;
    fn maybe_par_iter(self) -> Self::Iter {
        self.as_slice().par_iter()
    }
}

// ─── Serial build: identical surface, but std iterators. ─────────────────────
#[cfg(not(feature = "parallel"))]
pub trait IntoMaybePar {
    type Iter: Iterator;
    fn into_maybe_par_iter(self) -> Self::Iter;
}
#[cfg(not(feature = "parallel"))]
impl<T: IntoIterator> IntoMaybePar for T {
    type Iter = T::IntoIter;
    fn into_maybe_par_iter(self) -> Self::Iter {
        self.into_iter()
    }
}

#[cfg(not(feature = "parallel"))]
pub trait MaybeParRef<'a> {
    type Iter: Iterator;
    fn maybe_par_iter(self) -> Self::Iter;
}
#[cfg(not(feature = "parallel"))]
impl<'a, T: 'a> MaybeParRef<'a> for &'a [T] {
    type Iter = std::slice::Iter<'a, T>;
    fn maybe_par_iter(self) -> Self::Iter {
        self.iter()
    }
}
#[cfg(not(feature = "parallel"))]
impl<'a, T: 'a> MaybeParRef<'a> for &'a Vec<T> {
    type Iter = std::slice::Iter<'a, T>;
    fn maybe_par_iter(self) -> Self::Iter {
        self.iter()
    }
}
