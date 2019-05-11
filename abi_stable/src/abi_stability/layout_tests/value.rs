#![allow(dead_code)]

use std::{marker::PhantomData, num, ptr, sync::atomic};

#[allow(unused_imports)]
use core_extensions::{matches, prelude::*};

use crate::{
    abi_stability::{
        abi_checking::{AbiInstability,check_abi_stability},
        AbiInfoWrapper, 
        Tag,
    },
    marker_type::UnsafeIgnoredType,
    std_types::*,
    *,
    test_utils::must_panic,
};

mod regular {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        x: u32,
        y: u32,
        w: u16,
        h: u32,
    }
}

mod changed_name {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangleiiiiii {
        x: u32,
        y: u32,
        w: u16,
        h: u32,
    }
}

mod swapped_fields_first {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        y: u32,
        x: u32,
        w: u16,
        h: u32,
    }
}

mod swapped_fields_last {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        x: u32,
        y: u32,
        h: u32,
        w: u16,
    }
}

mod removed_field_first {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        y: u32,
        w: u16,
        h: u32,
    }
}

mod removed_field_last {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        x: u32,
        y: u32,
        w: u16,
    }
}

mod removed_all_fields {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        x: u32,
        y: u32,
        w: u16,
    }
}

mod changed_type_first {
    use super::shadowed;

    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        x: shadowed::u32,
        y: u32,
        w: u16,
        h: u32,
    }
}

mod changed_type_last {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        x: u32,
        y: u32,
        w: u16,
        h: u32,
    }
}

mod changed_alignment {
    #[repr(C, align(16))]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Rectangle {
        x: u32,
        y: u32,
        w: u16,
        h: u32,
    }
}

/// For testing that adding #[repr(C)] makes the derive macro not panic.
mod derive_validity_0 {
    and_stringify! {
        pub(super)const RECTANGLE_DEF_REPR;

        pub struct Rectangle {
            x:u32,
            y:u32,
            w:u16,
            h:u32,
        }
    }
}

mod built_in {
    pub use i32 as std_i32;
    pub use u32 as std_u32;
}

/// Types that have the same type-name and layout as built-in types.
#[allow(non_camel_case_types)]
mod shadowed {
    use super::built_in::*;

    #[repr(transparent)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct u32 {
        integer: std_i32,
    }

    #[repr(transparent)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct i32 {
        integer: std_i32,
    }
}

fn assert_sane_abi_info(abi: &'static AbiInfoWrapper) {
    assert_equal_abi_info(abi, abi);
}

fn assert_equal_abi_info(interface: &'static AbiInfoWrapper, impl_: &'static AbiInfoWrapper) {
    assert_eq!(check_abi_stability(interface, impl_), Ok(()));
}

fn assert_different_abi_info(interface: &'static AbiInfoWrapper, impl_: &'static AbiInfoWrapper) {
    let res=check_abi_stability(interface, impl_);
    assert_ne!(
        res,
        Ok(()),
        "\n\nInterface:{:#?}\n\nimplementation:{:#?}",
        interface,
        impl_,
    );
}

#[cfg_attr(not(miri),test)]
fn same_different_abi_stability() {
    let must_be_equal = vec![
        regular::Rectangle::ABI_INFO,
        swapped_fields_first::Rectangle::ABI_INFO,
        swapped_fields_last::Rectangle::ABI_INFO,
        removed_field_first::Rectangle::ABI_INFO,
        removed_field_last::Rectangle::ABI_INFO,
        removed_all_fields::Rectangle::ABI_INFO,
        changed_type_first::Rectangle::ABI_INFO,
        changed_type_last::Rectangle::ABI_INFO,
        shadowed::u32::ABI_INFO,
        shadowed::i32::ABI_INFO,
    ];

    for this in must_be_equal {
        assert_sane_abi_info(this);
    }

    let list = vec![
        <&mut ()>::ABI_INFO,
        <&mut i32>::ABI_INFO,
        <&()>::ABI_INFO,
        <&i32>::ABI_INFO,
        <&'static &'static ()>::ABI_INFO,
        <&'static mut &'static ()>::ABI_INFO,
        <&'static &'static mut ()>::ABI_INFO,
        <atomic::AtomicPtr<()>>::ABI_INFO,
        <atomic::AtomicPtr<i32>>::ABI_INFO,
        <*const ()>::ABI_INFO,
        <*const i32>::ABI_INFO,
        <*mut ()>::ABI_INFO,
        <*mut i32>::ABI_INFO,
        <[(); 0]>::ABI_INFO,
        <[(); 1]>::ABI_INFO,
        <[(); 2]>::ABI_INFO,
        <[(); 3]>::ABI_INFO,
        <[u32; 3]>::ABI_INFO,
        <i32>::ABI_INFO,
        <u32>::ABI_INFO,
        <bool>::ABI_INFO,
        <atomic::AtomicBool>::ABI_INFO,
        <atomic::AtomicIsize>::ABI_INFO,
        <atomic::AtomicUsize>::ABI_INFO,
        <num::NonZeroU32>::ABI_INFO,
        <num::NonZeroU16>::ABI_INFO,
        <ptr::NonNull<()>>::ABI_INFO,
        <ptr::NonNull<i32>>::ABI_INFO,
        <RVec<()>>::ABI_INFO,
        <RVec<i32>>::ABI_INFO,
        <RSlice<'_, ()>>::ABI_INFO,
        <RSlice<'_, i32>>::ABI_INFO,
        <RSliceMut<'_, ()>>::ABI_INFO,
        <RSliceMut<'_, i32>>::ABI_INFO,
        <Option<&()>>::ABI_INFO,
        <Option<&u32>>::ABI_INFO,
        <Option<extern "C" fn()>>::ABI_INFO,
        <ROption<()>>::ABI_INFO,
        <ROption<u32>>::ABI_INFO,
        <RCow<'_, str>>::ABI_INFO,
        <RCow<'_, [u32]>>::ABI_INFO,
        <RArc<()>>::ABI_INFO,
        <RArc<u32>>::ABI_INFO,
        <RBox<()>>::ABI_INFO,
        <RBox<u32>>::ABI_INFO,
        <RCmpOrdering>::ABI_INFO,
        <PhantomData<()>>::ABI_INFO,
        <PhantomData<RString>>::ABI_INFO,
        <mod_0::Mod>::ABI_INFO,
        <mod_0b::Mod>::ABI_INFO,
        <mod_1::Mod>::ABI_INFO,
        <mod_2::Mod>::ABI_INFO,
        <mod_3::Mod>::ABI_INFO,
        <mod_4::Mod>::ABI_INFO,
        <mod_5::Mod>::ABI_INFO,
        <mod_6::Mod>::ABI_INFO,
        <mod_7::Mod>::ABI_INFO,
        <Tagged<TAG_DEFAULT_1>>::ABI_INFO,
        <Tagged<TAG_DEFAULT_2>>::ABI_INFO,
        <Tagged<TAG_DEFAULT_3>>::ABI_INFO,
        <Tagged<TAG_DEFAULT_4>>::ABI_INFO,
        <Tagged<TAG_DEFAULT_5>>::ABI_INFO,
        <Tagged<TAG_DEFAULT_6>>::ABI_INFO,
        // <&prefix0::Prefix>::ABI_INFO,
        // <*const prefix0::Prefix>::ABI_INFO,
        // <RArc<prefix0::Prefix>>::ABI_INFO,
    ];

    let (_dur, ()) = core_extensions::measure_time::measure(|| {
        for (i, this) in list.iter().cloned().enumerate() {
            for (j, other) in list.iter().cloned().enumerate() {
                if i == j {
                    assert_equal_abi_info(this, other);
                } else {
                    assert_different_abi_info(this, other);
                }
            }
        }

        for this in vec![<UnsafeIgnoredType<()>>::ABI_INFO, <UnsafeIgnoredType<RString>>::ABI_INFO] {
            assert_equal_abi_info(<UnsafeIgnoredType<()>>::ABI_INFO, this)
        }
    });

    // println!("taken {} to check all listed layouts", dur);
}



// Checks that #[repr(Rust)] (the default representation) causes the derive macro
// to panic,and that #[repr(C)] and #[repr(transparent)] do not.
#[cfg_attr(not(miri),test)]
fn check_repr_attrs(){
    use abi_stable_derive_lib::derive_stable_abi_from_str;
    must_panic(file_span!(),||{
        derive_stable_abi_from_str(derive_validity_0::RECTANGLE_DEF_REPR)
    }).unwrap();
    
    must_panic(file_span!(),||{
        let with_repr_rust=format!(
            "#[repr(Rust)]\n{}",
            derive_validity_0::RECTANGLE_DEF_REPR
        );
        derive_stable_abi_from_str(&with_repr_rust)
    }).unwrap();

    let with_repr_c=format!("#[repr(C)]\n{}",derive_validity_0::RECTANGLE_DEF_REPR);
    let with_repr_tranparent=format!(
        "#[repr(transparent)]\n{}",
        derive_validity_0::RECTANGLE_DEF_REPR
    );

    derive_stable_abi_from_str(&with_repr_c);
    derive_stable_abi_from_str(&with_repr_tranparent);
}

// Uncomment this once I reimplement Prefix types.
//
// #[cfg_attr(not(miri),test)]
// fn different_prefixity() {
//     let regular = <&'static regular::Rectangle>::ABI_INFO;
//     let other = <&'static prefixed::Rectangle>::ABI_INFO;
//     let errs = check_abi_stability(regular, other)
//         .unwrap_err()
//         .flatten_errors();
//     assert!(errs
//         .iter()
//         .any(|err| matches!(AbiInstability::IsPrefix{..}=err)));
// }

#[cfg_attr(not(miri),test)]
fn different_zeroness() {
    const ZEROABLE_ABI: &'static AbiInfoWrapper = &{
        let mut abi = *<&()>::ABI_INFO.get();
        abi.is_nonzero = false;
        unsafe { AbiInfoWrapper::new_unchecked(abi) }
    };

    let non_zero = <&()>::ABI_INFO;

    assert!(non_zero.get().is_nonzero);
    assert!(!ZEROABLE_ABI.get().is_nonzero);

    let errs = check_abi_stability(non_zero, ZEROABLE_ABI)
        .unwrap_err()
        .flatten_errors();
    assert!(errs
        .iter()
        .any(|err| matches!(AbiInstability::NonZeroness{..}=err)));
}

#[cfg_attr(not(miri),test)]
fn different_name() {
    let regular = regular::Rectangle::ABI_INFO;
    let other = changed_name::Rectangleiiiiii::ABI_INFO;
    let errs = check_abi_stability(regular, other)
        .unwrap_err()
        .flatten_errors();
    assert!(errs
        .iter()
        .any(|err| matches!(AbiInstability::Name{..}=err)));
}

#[cfg_attr(not(miri),test)]
fn swapped_fields() {
    let regular = regular::Rectangle::ABI_INFO;
    let first = swapped_fields_first::Rectangle::ABI_INFO;
    let last = swapped_fields_first::Rectangle::ABI_INFO;

    for other in vec![first, last] {
        let errs = check_abi_stability(regular, other)
            .unwrap_err()
            .flatten_errors();
        assert!(errs
            .iter()
            .any(|x| matches!(AbiInstability::UnexpectedField{..}=x)))
    }
}

#[cfg_attr(not(miri),test)]
fn removed_fields() {
    let regular = regular::Rectangle::ABI_INFO;
    let list = vec![
        removed_field_first::Rectangle::ABI_INFO,
        removed_field_last::Rectangle::ABI_INFO,
        removed_all_fields::Rectangle::ABI_INFO,
    ];

    for other in list {
        let errs = check_abi_stability(regular, other)
            .unwrap_err()
            .flatten_errors();
        let mut found_field_count_mismatch = false;
        let mut is_size_mismatch = false;
        for err in errs {
            match err {
                AbiInstability::FieldCountMismatch { .. } => {
                    found_field_count_mismatch = true;
                }
                AbiInstability::Size { .. } => {
                    is_size_mismatch = true;
                }
                _ => {}
            }
        }
        assert!(found_field_count_mismatch);
        assert!(is_size_mismatch);
    }
}

#[cfg_attr(not(miri),test)]
fn different_alignment() {
    let regular = regular::Rectangle::ABI_INFO;
    let other = changed_alignment::Rectangle::ABI_INFO;
    let errs = check_abi_stability(regular, other)
        .unwrap_err()
        .flatten_errors();

    let mut found_alignment_mismatch = false;
    for err in errs {
        match err {
            AbiInstability::Alignment { .. } => {
                found_alignment_mismatch = true;
            }
            _ => {}
        }
    }
    assert!(found_alignment_mismatch);
}

//////////////////////////////////////////////////////////
//// Generics
//////////////////////////////////////////////////////////

mod gen_basic {
    use super::PhantomData;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Generics<T: 'static> {
        x: &'static T,
        y: &'static T,
        _marker: PhantomData<(T)>,
    }
}

mod gen_more_lts {
    use super::PhantomData;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    #[sabi(bound = "T:'a")]
    pub struct Generics<'a, T> {
        x: &'a T,
        y: &'a T,
        _marker: PhantomData<(&'a T)>,
    }
}

mod gen_more_tys {
    use super::{PhantomData,Tuple2};
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Generics<T: 'static, U> {
        x: &'static T,
        y: &'static T,
        _marker: PhantomData<Tuple2<T, U>>,
    }
}

// mod gen_more_consts{
// For when const-generics are usable
// #[repr(C)]
// #[derive(StableAbi)]
// #[sabi(inside_abi_stable_crate)]
// pub struct ExtraConstParam<T,const LEN:usize> {
//     x:T,
//     y:T,
//     _marker:PhantomData<(T,[u8;LEN])>,
// }
// }

#[cfg_attr(not(miri),test)]
fn different_generics() {
    let regular = gen_basic::Generics::<()>::ABI_INFO;

    {
        let list = vec![
            gen_more_lts::Generics::<()>::ABI_INFO,
            // gen_more_tys::Generics::<(), ()>::ABI_INFO,
        ];

        for other in list {
            let errs = check_abi_stability(regular, other)
                .unwrap_err()
                .flatten_errors();
            assert!(errs
                .iter()
                .any(|err| matches!(AbiInstability::GenericParamCount{..}=err)));
        }
    }

    {
        let list = vec![gen_more_lts::Generics::<()>::ABI_INFO];

        for other in list {
            let errs = check_abi_stability(regular, other)
                .unwrap_err()
                .flatten_errors();
            assert!(errs
                .iter()
                .any(|err| matches!(AbiInstability::FieldLifetimeMismatch{..}=err)));
        }
    }
}

//////////////////////////////////////////////////////////
////    Enums
//////////////////////////////////////////////////////////

mod basic_enum {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub enum Enum {
        Variant0,
        Variant1 { a: u32 },
    }
}

mod misnamed_variant {
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub enum Enum {
        Variant000000000,
        Variant1 { a: u32 },
    }
}

mod extra_variant {
    use crate::std_types::RString;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub enum Enum {
        Variant0,
        Variant1 { a: u32 },
        Variant3(RString),
    }
}

#[cfg_attr(not(miri),test)]
fn variant_mismatch() {
    let regular = basic_enum::Enum::ABI_INFO;

    {
        let other = misnamed_variant::Enum::ABI_INFO;
        let errs = check_abi_stability(regular, other)
            .unwrap_err()
            .flatten_errors();
        assert!(errs
            .iter()
            .any(|err| matches!(AbiInstability::UnexpectedVariant{..}=err)));
    }

    {
        let other = extra_variant::Enum::ABI_INFO;
        let errs = check_abi_stability(regular, other)
            .unwrap_err()
            .flatten_errors();
        assert!(errs
            .iter()
            .any(|err| matches!(AbiInstability::TooManyVariants{..}=err)));
    }
}



//////////////////////////////////////////////////////////////////////////////
///  Modules,with function pointers

mod mod_0 {
    use crate::std_types::RString;

    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn(&mut u32,u64) -> RString,
        pub function_1: extern "C" fn() -> RString,
    }
}

mod mod_0b {
    use crate::std_types::RString;

    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn(&mut u32,u64,()) -> RString,
        pub function_1: extern "C" fn() -> RString,
    }
}


mod mod_1 {
    use crate::std_types::RString;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn(&mut u32,u64,RString),
        pub function_1: extern "C" fn(RString),
    }
}


mod mod_2 {
    use crate::std_types::RString;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn(&mut u32,u64,RString)->RString,
        pub function_1: extern "C" fn(),
    }
}


mod mod_3 {
    use crate::std_types::RString;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn(&mut u32,u64,RString),
        pub function_1: extern "C" fn()->RString,
    }
}

mod mod_4 {
    use crate::std_types::RString;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn()->RString,
        pub function_1: extern "C" fn(&mut u32,u64,RString),
    }
}


mod mod_5 {
    use crate::std_types::RString;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn()->RString,
        pub function_1: extern "C" fn(&mut u32,u64,RString),
        pub function_2: extern "C" fn(&mut u32,u64,RString),
    }
}

mod mod_6 {
    use crate::std_types::RString;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn()->RString,
    }
}

mod mod_7 {
    use crate::std_types::RString;
    #[repr(C)]
    #[derive(StableAbi)]
    #[sabi(inside_abi_stable_crate)]
    pub struct Mod{
        pub function_0: extern "C" fn()->RString,
        pub function_1: extern "C" fn(&mut u32,u64,RString),
        pub function_2: extern "C" fn((),(),()),
    }
}


//////////////////////////////////////////////////////////////////////////////
////            Tagged values
//////////////////////////////////////////////////////////////////////////////


#[repr(C)]
#[derive(StableAbi)]
#[sabi(
    inside_abi_stable_crate,
    bound="M:ToTagConst",
    tag="<M as ToTagConst>::TAG",
    unconstrained(M),

)]
pub struct Tagged<M>(UnsafeIgnoredType<M>);


pub trait ToTagConst{
    const TAG:Tag;
}

macro_rules! declare_tags {
    (
        $(const $marker_ty:ident = $tag:expr;)*
    ) => (
        $(
            pub struct $marker_ty;

            impl ToTagConst for $marker_ty {
                const TAG:Tag=$tag;
            }
        )*
    )
}


declare_tags!{
    const TAG_DEFAULT_0=Tag::null();
    const TAG_DEFAULT_1=Tag::bool_(false);
    const TAG_DEFAULT_2=Tag::int(0);
    const TAG_DEFAULT_3=Tag::uint(0);
    const TAG_DEFAULT_4=Tag::str("");
    const TAG_DEFAULT_5=Tag::arr(&[]);
    const TAG_DEFAULT_6=Tag::set(&[]);
    
    const TAG_EMPTY_SET=Tag::set(&[]);
    
    const TAG_SET_A0=Tag::set(&[
        Tag::str("Sync"),
    ]);
    const TAG_SET_A1=Tag::set(&[
        Tag::str("Send"),
    ]);
    const TAG_SET_A2=Tag::set(&[
        Tag::str("Copy"),
    ]);
    const TAG_SET_A3=Tag::set(&[
        Tag::str("Clone"),
    ]);
    const TAG_SET_B0=Tag::set(&[
        Tag::str("Send"),
        Tag::str("Sync"),
    ]);
    const TAG_SET_B1=Tag::set(&[
        Tag::str("Copy"),
        Tag::str("Clone"),
    ]);

    const TAG_SET_C0=Tag::set(&[
        Tag::str("Send"),
        Tag::str("Sync"),
        Tag::str("Copy"),
        Tag::str("Clone"),
    ]);

    const TAG_SET_C1=Tag::set(&[
        Tag::str("Debug"),
        Tag::str("Display"),
    ]);
}


trait TaggedExt{
    const GET_AI:&'static AbiInfoWrapper;
}


impl<T> TaggedExt for T
where 
    Tagged<T>:StableAbi,
{
    const GET_AI:&'static AbiInfoWrapper=
        <Tagged<T> as StableAbi>::ABI_INFO;
}



#[test]
fn test_tag_subsets(){
    let valid_subsets=vec![
        vec![TAG_EMPTY_SET::GET_AI, TAG_SET_A0::GET_AI, TAG_SET_B0::GET_AI, TAG_SET_C0::GET_AI],
        vec![TAG_EMPTY_SET::GET_AI, TAG_SET_A1::GET_AI, TAG_SET_B0::GET_AI, TAG_SET_C0::GET_AI],
        vec![TAG_EMPTY_SET::GET_AI, TAG_SET_A2::GET_AI, TAG_SET_B1::GET_AI, TAG_SET_C0::GET_AI],
        vec![TAG_EMPTY_SET::GET_AI, TAG_SET_A3::GET_AI, TAG_SET_B1::GET_AI, TAG_SET_C0::GET_AI],
    ];


    for subset in &valid_subsets {
        for (l_i,l_abi) in subset.iter().enumerate() {
            for (r_i,r_abi) in subset.iter().enumerate() {

                let res=check_abi_stability(l_abi,r_abi);

                if l_i <= r_i {
                    assert_eq!(res,Ok(()));
                }else{
                    let errs=res.unwrap_err().flatten_errors();
                    assert!(
                        errs
                        .iter()
                        .any(|err| matches!(AbiInstability::TagError{..}=err) )
                    );
                }
            }
        }
    }
}

#[test]
fn test_tag_incompatible(){
    let incompatible_sets=vec![
        vec![
            TAG_SET_A0::GET_AI,
            TAG_SET_A1::GET_AI,
            TAG_SET_A2::GET_AI,
            TAG_SET_A3::GET_AI,
            TAG_SET_C1::GET_AI,
        ],
        vec![TAG_SET_B0::GET_AI, TAG_SET_B1::GET_AI],
        vec![TAG_SET_C0::GET_AI, TAG_SET_C1::GET_AI],
    ];

    for subset in &incompatible_sets {
        for (l_i,l_abi) in subset.iter().enumerate() {
            for (r_i,r_abi) in subset.iter().enumerate() {
                let res=check_abi_stability(l_abi,r_abi);

                if l_i == r_i {
                    assert_eq!(res,Ok(()));
                }else{
                    let errs=res.unwrap_err().flatten_errors();
                    assert!(
                        errs
                        .iter()
                        .any(|err| matches!(AbiInstability::TagError{..}=err) )
                    );
                }
            }
        }
    }
}