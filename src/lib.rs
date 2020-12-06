// Copyright (C) 2020 sanfusu
// 
// This file is part of accessor.
// 
// accessor is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// accessor is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with accessor.  If not, see <http://www.gnu.org/licenses/>.

//! # Example
//! ```
//! #![allow(dead_code)]
//! #![allow(unused_variables)]
//!
//! use std::{cell::RefCell, convert::TryInto, ops::Range, rc::Rc};
//! use accessor::{Encode, Field, Getter, Mutable, Setter};
//! // Field1 字段类型为 u8 且可修改，起始于第 0 个字节，终于第 1 个字节（不包括第一个字节）
//! struct Field1(u8);
//! impl Mutable for Field1 {}
//! impl Field for Field1 {
//!     fn range() -> Range<usize> {
//!         0..1
//!     }
//!
//!     fn from_le_bytes(val: &[u8]) -> u8 {
//!         u8::from_le(val[0])
//!     }
//!     fn from_be_bytes(val: &[u8]) -> u8 {
//!         u8::from_be(val[0])
//!     }
//!     type BytesType = [u8; 1];
//!     type FieldType = u8;
//!
//!     fn to_be_bytes(self) -> Self::BytesType {
//!         self.0.to_be_bytes()
//!     }
//!
//!     fn to_le_bytes(self) -> Self::BytesType {
//!         self.0.to_le_bytes()
//!     }
//! }
//! struct Field2(u32);
//! impl Mutable for Field2 {}
//! impl Field for Field2 {
//!     type FieldType = u32;
//!     type BytesType = [u8; 4];
//!
//!     fn range() -> Range<usize> {
//!         1..5
//!     }
//!
//!     fn from_le_bytes(val: &[u8]) -> Self::FieldType {
//!         u32::from_le_bytes(val.try_into().unwrap())
//!     }
//!     fn from_be_bytes(val: &[u8]) -> Self::FieldType {
//!         u32::from_be_bytes(val.try_into().unwrap())
//!     }
//!
//!     fn to_be_bytes(self) -> Self::BytesType {
//!         self.0.to_be_bytes()
//!     }
//!
//!     fn to_le_bytes(self) -> Self::BytesType {
//!         self.0.to_le_bytes()
//!     }
//! }
//!
//! struct Test {
//!     data: Rc<RefCell<[u8]>>,
//!     encode: Encode,
//! }
//! impl Test {
//!     fn new(data: Rc<RefCell<[u8]>>) -> Test {
//!         Test {
//!             data,
//!             encode: Encode::Le,
//!         }
//!     }
//! }
//!
//! impl Getter for Test {
//!     fn getter(&self, encode: Encode) -> Self {
//!         Self {
//!             data: self.data.clone(),
//!             encode,
//!         }
//!     }
//!     fn get<T>(&self) -> T::FieldType
//!     where
//!         T: Field,
//!     {
//!         match self.encode {
//!             Encode::Le => T::from_le_bytes(&self.data.borrow()[T::range()]),
//!             Encode::Be => T::from_be_bytes(&self.data.borrow()[T::range()]),
//!         }
//!     }
//! }
//! impl Setter for Test {
//!     fn setter(&self, encode: Encode) -> Self {
//!         Self {
//!             data: self.data.clone(),
//!             encode,
//!         }
//!     }
//!     fn with<T: Field + Mutable>(&self, value: T) -> &Self {
//!         match self.encode {
//!             Encode::Le => {
//!                 self.data.borrow_mut()[T::range()]
//!                     .copy_from_slice(value.to_le_bytes().as_ref());
//!             }
//!             Encode::Be => {
//!                 self.data.borrow_mut()[T::range()]
//!                     .copy_from_slice(value.to_be_bytes().as_ref());
//!             }
//!         };
//!         self
//!     }
//! }
//! fn main() {
//!     let a = Test::new(Rc::new(RefCell::new([
//!         0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
//!     ])));
//!
//!     let getter = a.getter(Encode::Be);
//!     println!("{:#x?}", getter.get::<Field1>());
//!     println!("{:#x?}", getter.get::<Field2>());
//!
//!     let setter = a.setter(Encode::Le);
//!     setter
//!         .with(Field1(0x12))
//!         .with(Field2(0x12345678))
//!         .setter(Encode::Le)
//!         .with(Field1(0x23));
//!     println!("{:#x?}", getter.get::<Field1>());
//!     println!("{:#x?}", getter.get::<Field2>());
//!
//!     let mut field1: <Field1 as Field>::FieldType = 0;
//!     let mut field2: <Field2 as Field>::FieldType = 0;
//!     getter.out::<Field1>(&mut field1).out::<Field2>(&mut field2);
//!     println!("{:#x?}, {:#x?}", field1, field2);
//! }
//! ```

use std::ops::Range;

/// 有且只有两种字节序：Le(小端)，Be(大端)
pub enum Encode {
    Le,
    Be,
}

pub trait Field {
    /// 字段类型
    type FieldType;
    /// TODO: BytesType 可以直接替换为 [u8; std::mem::size_of::<Self::FieldType>()]，但目前 rust 不支持这种写法
    type BytesType: AsRef<[u8]>;

    fn from_le_bytes(val: &[u8]) -> Self::FieldType;
    fn from_be_bytes(val: &[u8]) -> Self::FieldType;
    fn to_be_bytes(self) -> Self::BytesType;
    fn to_le_bytes(self) -> Self::BytesType;
    /// range 函数一般会被 Layout 中的 with 函数调用，获取 slice 后，在调用 from_le(ge)_bytes 从而获取字段的值。
    fn range() -> Range<usize>;
}

pub trait Getter {
    /// get 函数返回某个字段的值
    /// # Example
    /// ```not_run
    /// let value = self.get::<Field1>();
    /// ```
    fn get<T>(&self) -> T::FieldType
    where
        T: Field;
    fn getter(&self, encode: Encode) -> Self;

    /// out 将字段值赋值给 dest，并返回 Getter 自身的引用，方便链式调用一条语句输出多个值。
    fn out<T: Field>(&self, dest: &mut T::FieldType) -> &Self {
        *dest = self.get::<T>();
        self
    }
}

pub trait Setter {
    /// with 函数一般用于修改二进制格式中的某个字段，
    /// 返回 `&mut Self` 类型，方便链式调用
    /// # Example
    /// ```not_run
    /// self.with::<Field1>(value1).with::<Field2>(value2);
    /// ```
    fn with<T: Field + Mutable>(&self, value: T) -> &Self;
    fn setter(&self, encode: Encode) -> Self;
}

/// 空接口，用于限制 Field
pub trait Mutable {}
