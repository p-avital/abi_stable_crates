//! Traits for types wrapped in `DynTrait<_>`

use crate::std_types::RBoxError;

#[allow(unused_imports)]
use crate::type_level::{
    bools::{False, True},
    impl_enum::{Implementability, Implemented, Unimplemented},
    trait_marker,
};

macro_rules! declare_InterfaceType {
    (

        $(#[$attrs:meta])*

        assoc_types[
            $(
                $(#[$assoc_attrs:meta])*
                type $trait_:ident ;
            )*
        ]
    ) => (
        $(#[$attrs])*
        pub trait InterfaceType: Sized {
            $(
                $(#[$assoc_attrs])*
                type $trait_: Implementability;
            )*

            #[doc(hidden)]
            type define_this_in_the_impl_InterfaceType_macro;
        }


    )
}

declare_InterfaceType! {
    /// Defines the usable/required traits when creating a
    /// [`DynTrait<Pointer<()>, ThisInterfaceType>`](crate::DynTrait).
    ///
    /// This trait can only be implemented using the
    /// [`#[derive(StableAbi)]`](derive@crate::StableAbi)
    /// derive with the
    /// [`#[sabi(impl_InterfaceType(...))]`](derive@crate::StableAbi#sabiimpl_interfacetype)
    /// helper attribute,
    /// defaulting associated types to `Unimplemented<_>`.
    ///
    /// The value of every associated type can be:
    ///
    /// - [`Implemented<_>`](crate::type_level::impl_enum::Implemented):
    /// the trait would be required by, and be usable in `DynTrait`.
    ///
    /// - [`Unimplemented<_>`](crate::type_level::impl_enum::Unimplemented):
    /// the trait would not be required by, and not be usable in `DynTrait`.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// use abi_stable::{erased_types::InterfaceType, type_level::bools::*, StableAbi};
    ///
    /// #[repr(C)]
    /// #[derive(StableAbi)]
    /// #[sabi(impl_InterfaceType(Clone, Debug))]
    /// pub struct FooInterface;
    ///
    /// /*
    /// The `#[sabi(impl_InterfaceType(Clone, Debug))]` helper attribute
    /// (as part of #[derive(StableAbi)]) above is roughly equivalent to this impl:
    ///
    /// impl InterfaceType for FooInterface {
    ///     type Clone = Implemented<trait_marker::Clone>;
    ///
    ///     type Debug = Implemented<trait_marker::Debug>;
    ///
    ///     /////////////////////////////////////
    ///     //// defaulted associated types
    ///     /////////////////////////////////////
    ///
    ///     // Changing this to require/unrequire in minor versions, is an abi breaking change.
    ///     // type Send = Unimplemented<trait_marker::Send>;
    ///
    ///     // Changing this to require/unrequire in minor versions, is an abi breaking change.
    ///     // type Sync = Unimplemented<trait_marker::Sync>;
    ///
    ///     // Changing this to require/unrequire in minor versions, is an abi breaking change.
    ///     // type Unpin = Unimplemented<trait_marker::Unpin>;
    ///
    ///     // type Iterator = Unimplemented<trait_marker::Iterator>;
    ///
    ///     // type DoubleEndedIterator = Unimplemented<trait_marker::DoubleEndedIterator>;
    ///
    ///     // type Default = Unimplemented<trait_marker::Default>;
    ///
    ///     // type Display = Unimplemented<trait_marker::Display>;
    ///
    ///     // type Serialize = Unimplemented<trait_marker::Serialize>;
    ///
    ///     // type Eq = Unimplemented<trait_marker::Eq>;
    ///
    ///     // type PartialEq = Unimplemented<trait_marker::PartialEq>;
    ///
    ///     // type Ord = Unimplemented<trait_marker::Ord>;
    ///
    ///     // type PartialOrd = Unimplemented<trait_marker::PartialOrd>;
    ///
    ///     // type Hash = Unimplemented<trait_marker::Hash>;
    ///
    ///     // type Deserialize = Unimplemented<trait_marker::Deserialize>;
    ///
    ///     // type FmtWrite = Unimplemented<trait_marker::FmtWrite>;
    ///
    ///     // type IoWrite = Unimplemented<trait_marker::IoWrite>;
    ///
    ///     // type IoSeek = Unimplemented<trait_marker::IoSeek>;
    ///
    ///     // type IoRead = Unimplemented<trait_marker::IoRead>;
    ///
    ///     // type IoBufRead = Unimplemented<trait_marker::IoBufRead>;
    ///
    ///     // type Error = Unimplemented<trait_marker::Error>;
    /// }
    /// */
    ///
    /// # fn main(){}
    ///
    ///
    /// ```
    ///
    ///
    ///
    ///
    assoc_types[
        /// Changing this to require/unrequire in minor versions, is an abi breaking change.
        type Send;

        /// Changing this to require/unrequire in minor versions, is an abi breaking change.
        type Sync;

        /// Changing this to require/unrequire in minor versions, is an abi breaking change.
        type Unpin;

        ///
        type Clone;

        ///
        type Default;

        ///
        type Display;

        ///
        type Debug;

        ///
        type Serialize;

        ///
        type Eq;

        ///
        type PartialEq;

        ///
        type Ord;

        ///
        type PartialOrd;

        ///
        type Hash;

        ///
        type Deserialize;

        ///
        type Iterator;

        ///
        type DoubleEndedIterator;

        /// For the `std::fmt::Write` trait
        type FmtWrite;

        /// For the `std::io::Write` trait
        type IoWrite;

        /// For the `std::io::Seek` trait
        type IoSeek;

        /// For the `std::io::Read` trait
        type IoRead;

        /// For the `std::io::BufRead` trait
        type IoBufRead;

        /// For the `std::error::Error` trait
        type Error;
    ]


}

///////////////////////////////////////////////////////////////////////////////

/// Describes how a type is serialized by [`DynTrait`].
///
/// [`DynTrait`]: ../struct.DynTrait.html
pub trait SerializeType<'s> {
    /// An [`InterfaceType`] implementor which determines the
    /// intermediate type through which this is serialized.
    ///
    /// [`InterfaceType`]: ./trait.InterfaceType.html
    type Interface: SerializeProxyType<'s>;

    /// Performs the serialization into the proxy.
    fn serialize_impl(
        &'s self,
    ) -> Result<<Self::Interface as SerializeProxyType<'s>>::Proxy, RBoxError>;
}

/// Determines the intermediate type a [`SerializeType`] implementor is converted into,
/// and is then serialized.
///
/// [`SerializeType`]: ./trait.SerializeType.html
pub trait SerializeProxyType<'borr>: InterfaceType {
    /// The intermediate type.
    type Proxy: 'borr;
}

#[doc(hidden)]
pub trait GetSerializeProxyType<'borr>: InterfaceType {
    type ProxyType;
}

impl<'borr, I, PT> GetSerializeProxyType<'borr> for I
where
    I: InterfaceType,
    I: GetSerializeProxyTypeHelper<'borr, <I as InterfaceType>::Serialize, ProxyType = PT>,
{
    type ProxyType = PT;
}

#[doc(hidden)]
pub trait GetSerializeProxyTypeHelper<'borr, IS>: InterfaceType {
    type ProxyType;
}

impl<'borr, I> GetSerializeProxyTypeHelper<'borr, Implemented<trait_marker::Serialize>> for I
where
    I: SerializeProxyType<'borr>,
{
    type ProxyType = <I as SerializeProxyType<'borr>>::Proxy;
}

impl<'borr, I> GetSerializeProxyTypeHelper<'borr, Unimplemented<trait_marker::Serialize>> for I
where
    I: InterfaceType,
{
    type ProxyType = ();
}

///////////////////////////////////////

/// Describes how `D` is deserialized, using a proxy to do so.
///
/// Generally this delegates to a library function,
/// so that the implementation can be delegated
/// to the `implementation crate`.
///
pub trait DeserializeDyn<'borr, D> {
    /// The type that is deserialized and then converted into `D`,
    /// with `DeserializeDyn::deserialize_dyn`.
    type Proxy;

    /// Converts the proxy type into `D`.
    fn deserialize_dyn(s: Self::Proxy) -> Result<D, RBoxError>;
}

#[doc(hidden)]
pub trait GetDeserializeDynProxy<'borr, D>: InterfaceType {
    type ProxyType;
}

impl<'borr, I, D, PT> GetDeserializeDynProxy<'borr, D> for I
where
    I: InterfaceType,
    I: GetDeserializeDynProxyHelper<'borr, D, <I as InterfaceType>::Deserialize, ProxyType = PT>,
{
    type ProxyType = PT;
}

#[doc(hidden)]
pub trait GetDeserializeDynProxyHelper<'borr, D, IS>: InterfaceType {
    type ProxyType;
}

impl<'borr, I, D> GetDeserializeDynProxyHelper<'borr, D, Implemented<trait_marker::Deserialize>>
    for I
where
    I: InterfaceType,
    I: DeserializeDyn<'borr, D>,
{
    type ProxyType = <I as DeserializeDyn<'borr, D>>::Proxy;
}

impl<'borr, I, D> GetDeserializeDynProxyHelper<'borr, D, Unimplemented<trait_marker::Deserialize>>
    for I
where
    I: InterfaceType,
{
    type ProxyType = ();
}

/////////////////////////////////////////////////////////////////////

/// The way to specify the expected `Iterator::Item` type for an `InterfaceType`.
///
/// This is a separate trait to allow iterators that yield borrowed elements.
pub trait IteratorItem<'a>: InterfaceType {
    /// The iterator item type.
    type Item;
}

/// Gets the expected `Iterator::Item` type for an `InterfaceType`,
/// defaulting to `()` if it doesn't require `Iterator` to be implemented.
///
/// Used by `DynTrait`'s vtable to give its iterator methods a defaulted return type.
pub trait IteratorItemOrDefault<'borr>: InterfaceType {
    /// The iterator item type.
    type Item;
}

impl<'borr, I, Item> IteratorItemOrDefault<'borr> for I
where
    I: InterfaceType,
    I: IteratorItemOrDefaultHelper<'borr, <I as InterfaceType>::Iterator, Item = Item>,
{
    type Item = Item;
}

#[doc(hidden)]
pub trait IteratorItemOrDefaultHelper<'borr, ImplIsRequired> {
    type Item;
}

impl<'borr, I, Item> IteratorItemOrDefaultHelper<'borr, Implemented<trait_marker::Iterator>> for I
where
    I: IteratorItem<'borr, Item = Item>,
{
    type Item = Item;
}

impl<'borr, I> IteratorItemOrDefaultHelper<'borr, Unimplemented<trait_marker::Iterator>> for I {
    type Item = ();
}

/////////////////////////////////////////////////////////////////////

crate::impl_InterfaceType! {
    impl crate::erased_types::InterfaceType for () {
        type Send= True;
        type Sync= True;
    }
}
