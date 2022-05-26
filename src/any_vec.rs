use std::alloc::Layout;
use std::any::TypeId;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use crate::{AnyVecMut, AnyVecRef};
use crate::any_value::AnyValue;
use crate::any_vec_raw::AnyVecRaw;
use crate::ops::{AnyValueTemp, Remove, SwapRemove};
use crate::any_vec::traits::{Trait};
use crate::traits::Cloneable;

// TODO: rename mod to marker
/// Trait constraints.
/// Possible variants [`Cloneable`], [`Send`] and [`Sync`], in any combination.
///
/// # Example
/// ```rust
/// use any_vec::AnyVec;
/// use any_vec::traits::*;
/// let v1: AnyVec<dyn Cloneable + Sync + Send> = AnyVec::new::<String>();
/// let v2 = v1.clone();
///
/// ```
pub mod traits{
    mod private{
        pub trait Sealed{}
    }

    /// Does not enforce anything. Default.
    pub trait Trait: private::Sealed {}

    impl Trait for dyn Sync{}
    impl private::Sealed for dyn Sync{}

    impl Trait for dyn Send{}
    impl private::Sealed for dyn Send{}

    impl Trait for dyn Sync + Send{}
    impl private::Sealed for dyn Sync + Send{}

    /// Enforce type [`Clone`]-ability.
    pub trait Cloneable: Trait{}
}

pub trait CheckTraits<Traits: ?Sized>{}

impl<T> CheckTraits<dyn Trait> for T{}
impl<T: Clone> CheckTraits<dyn Cloneable> for T{}
impl<T: Send> CheckTraits<dyn Send> for T{}
impl<T: Sync> CheckTraits<dyn Sync> for T{}

impl<T: Send + Sync> CheckTraits<dyn Send + Sync> for T{}
impl<T: Clone + Send> CheckTraits<dyn Cloneable + Send> for T{}
impl<T: Clone + Sync> CheckTraits<dyn Cloneable + Sync> for T{}

impl<T: Clone + Send + Sync> CheckTraits<dyn Cloneable + Send + Sync> for T{}

/// Type erased vec-like container.
/// All elements have the same type.
///
/// Only destruct operations have indirect call overhead.
///
/// You can make AnyVec [`Send`]-able, [`Sync`]-able, [`Cloneable`], by
/// specifying trait constraints: `AnyVec<dyn Cloneable + Sync + Send>`. See [`crate::traits`].
///
/// Some operations return [`AnyValueTemp<Operation>`], which internally holds &mut to [`AnyVec`].
/// You can drop it, cast to concrete type, or put into another vector. (See [`AnyValue`])
///
/// *`Element: 'static` due to TypeId requirements*
pub struct AnyVec<Traits: ?Sized + Trait = dyn Trait> {
    raw: AnyVecRaw,
    phantom: PhantomData<Traits>
}

impl<Traits: ?Sized + Trait> AnyVec<Traits> {
    /// Element should implement requested Traits
    pub fn new<Element: 'static>() -> Self
        where Element: CheckTraits<Traits>
    {
        Self::with_capacity::<Element>(0)
    }

    /// Element should implement requested Traits
    pub fn with_capacity<Element: 'static>(capacity: usize) -> Self
        where Element: CheckTraits<Traits>
    {
        Self{
            raw: AnyVecRaw::with_capacity::<Element>(capacity),
            phantom: PhantomData
        }
    }

    #[inline]
    pub fn downcast_ref<Element: 'static>(&self) -> Option<AnyVecRef<Element>> {
        self.raw.downcast_ref::<Element>()
    }

    #[inline]
    pub unsafe fn downcast_ref_unchecked<Element: 'static>(&self) -> AnyVecRef<Element> {
        self.raw.downcast_ref_unchecked::<Element>()
    }

    #[inline]
    pub fn downcast_mut<Element: 'static>(&mut self) -> Option<AnyVecMut<Element>> {
        self.raw.downcast_mut::<Element>()
    }

    #[inline]
    pub unsafe fn downcast_mut_unchecked<Element: 'static>(&mut self) -> AnyVecMut<Element> {
        self.raw.downcast_mut_unchecked::<Element>()
    }

    /// # Panics
    ///
    /// * Panics if type mismatch.
    /// * Panics if index is out of bounds.
    /// * Panics if out of memory.
    pub fn insert<V: AnyValue>(&mut self, index: usize, value: V) {
        self.raw.insert(index, value);
    }

    /// # Panics
    ///
    /// * Panics if type mismatch.
    /// * Panics if out of memory.
    #[inline]
    pub fn push<V: AnyValue>(&mut self, value: V) {
        self.raw.push(value);
    }

    /// # Panics
    ///
    /// * Panics if index out of bounds.
    ///
    /// # Leaking
    ///
    /// If the returned [`AnyValueTemp`] goes out of scope without being dropped (due to
    /// [`mem::forget`], for example), the vector may have lost and leaked
    /// elements with indices >= index.
    ///
    #[inline]
    pub fn remove(&mut self, index: usize) -> AnyValueTemp<Remove> {
        self.raw.remove(index)
    }

    /// # Panics
    ///
    /// * Panics if index out of bounds.
    ///
    /// # Leaking
    ///
    /// If the returned [`AnyValueTemp`] goes out of scope without being dropped (due to
    /// [`mem::forget`], for example), the vector may have lost and leaked
    /// elements with indices >= index.
    ///
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> AnyValueTemp<SwapRemove> {
        self.raw.swap_remove(index)
    }

    #[inline]
    pub fn clear(&mut self){
        self.raw.clear()
    }

    /// Element TypeId
    #[inline]
    pub fn element_typeid(&self) -> TypeId{
        self.raw.element_typeid()
    }

    /// Element Layout
    #[inline]
    pub fn element_layout(&self) -> Layout {
        self.raw.element_layout()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.raw.capacity()
    }
}

unsafe impl<Traits: ?Sized + Trait> Send for AnyVec<Traits>
    where Traits: Send
{}

unsafe impl<Traits: ?Sized + Trait> Sync for AnyVec<Traits>
    where Traits: Sync
{}

impl<Traits: ?Sized + Trait> Clone for AnyVec<Traits>
    where Traits: Cloneable
{
    fn clone(&self) -> Self {
        Self{
            raw: unsafe{ self.raw.clone() },
            phantom: PhantomData
        }
    }
}