// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

//! Generate random `cborian::Value`s.
//!
use super::types::Tag;
use super::value::{Bytes, Int, Key, Simple, Text, Value};
use quickcheck::{Arbitrary, Gen};
use rand::prelude::*;
use std::collections::{BTreeMap, LinkedList};

/// Generate a random `cborian::Value`.
/// Mostly useful for quickcheck related tests.
/// The parameter `level` denotes the maximum nesting of this value.
pub fn gen_value<GenType: Gen>(level: u16, make_value: &mut GenType) -> Value {
    let mut make_value = make_value;
    match rand::Rng::gen_range(&mut make_value, 0, 20) {
        0 => Value::Null,
        1 => Value::Undefined,
        2 => Value::U8(make_value.gen()),
        3 => Value::U16(make_value.gen()),
        4 => Value::U32(make_value.gen()),
        5 => Value::U64(make_value.gen()),
        6 => Value::I8(make_value.gen()),
        7 => Value::I16(make_value.gen()),
        8 => Value::I32(make_value.gen()),
        9 => Value::I64(make_value.gen()),
        10 => Value::F32(make_value.gen()),
        11 => Value::F64(make_value.gen()),
        12 => Value::Text(gen_text(make_value)),
        13 => Value::Bytes(gen_bytes(make_value)),
        14 => Value::Array(if level > 0 {
            gen_array(level - 1, make_value)
        } else {
            Vec::with_capacity(0)
        }),
        15 => Value::Bool(make_value.gen()),
        16 => Value::Simple(gen_simple(make_value)),
        17 => Value::Map(if level > 0 {
            gen_map(level - 1, make_value)
        } else {
            BTreeMap::new()
        }),
        18 => Value::Int(gen_int(make_value)),
        _ => gen_tagged(make_value),
    }
}

fn gen_array<GenType: Gen>(level: u16, make_value: &mut GenType) -> Vec<Value> {
    let len = make_value.gen_range(0, 64);
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(gen_value(level, make_value))
    }
    vec
}

fn gen_map<G: Gen>(level: u16, g: &mut G) -> BTreeMap<Key, Value> {
    let len = g.gen_range(0, 64);
    let mut map = BTreeMap::new();
    for _ in 0..len {
        map.insert(gen_key(g), gen_value(level, g));
    }
    map
}

fn gen_key<G: Gen>(g: &mut G) -> Key {
    match g.gen_range(0, 4) {
        0 => Key::Int(gen_int(g)),
        1 => Key::Text(gen_text(g)),
        2 => Key::Bytes(gen_bytes(g)),
        _ => Key::Bool(g.gen()),
    }
}

fn gen_int<G: Gen>(g: &mut G) -> Int {
    if g.gen() {
        Int::Pos(g.gen())
    } else {
        Int::Neg(g.gen())
    }
}

fn gen_tag<G: Gen>(g: &mut G) -> Tag {
    match g.gen_range(0, 20) {
        0 => Tag::DateTime,
        1 => Tag::Timestamp,
        2 => Tag::Bignum,
        3 => Tag::NegativeBignum,
        4 => Tag::Decimal,
        5 => Tag::Bigfloat,
        6 => Tag::ToBase64Url,
        7 => Tag::ToBase64,
        8 => Tag::ToBase16,
        9 => Tag::Cbor,
        10 => Tag::Uri,
        11 => Tag::Base64Url,
        12 => Tag::Base64,
        13 => Tag::Regex,
        14 => Tag::Mime,
        15 => Tag::CborSelf,
        tg => Tag::Unassigned(tg),
    }
}

fn gen_text<G: Gen>(g: &mut G) -> Text {
    match g.gen() {
        true => Text::Text(Arbitrary::arbitrary(g)),
        false => Text::Chunks(gen_chunks(g)),
    }
}

fn gen_bytes<G: Gen>(g: &mut G) -> Bytes {
    match g.gen() {
        true => Bytes::Bytes(Arbitrary::arbitrary(g)),
        false => Bytes::Chunks(gen_chunks(g)),
    }
}

fn gen_chunks<A: Arbitrary, G: Gen>(g: &mut G) -> LinkedList<A> {
    let mut xs = LinkedList::new();
    for _ in 0..g.gen_range(0, 64) {
        xs.push_back(Arbitrary::arbitrary(g))
    }
    xs
}

fn gen_simple<G: Gen>(g: &mut G) -> Simple {
    match g.gen() {
        true => match g.gen() {
            n @ 0u8..=19 | n @ 28..=30 => Simple::Unassigned(n),
            n @ 32..=255 => Simple::Unassigned(n),
            _ => Simple::Unassigned(0),
        },
        false => match g.gen() {
            n @ 0u8..=31 => Simple::Reserved(n),
            _ => Simple::Reserved(0),
        },
    }
}

fn gen_tagged<G: Gen>(g: &mut G) -> Value {
    match gen_tag(g) {
        t @ Tag::DateTime => Value::Tagged(t, Box::new(Value::Text(gen_text(g)))),
        t @ Tag::Timestamp => match g.gen_range(0, 10) {
            0 => Value::Tagged(t, Box::new(Value::U8(g.gen()))),
            1 => Value::Tagged(t, Box::new(Value::U16(g.gen()))),
            2 => Value::Tagged(t, Box::new(Value::U32(g.gen()))),
            3 => Value::Tagged(t, Box::new(Value::U64(g.gen()))),
            4 => Value::Tagged(t, Box::new(Value::I8(g.gen()))),
            5 => Value::Tagged(t, Box::new(Value::I16(g.gen()))),
            6 => Value::Tagged(t, Box::new(Value::I32(g.gen()))),
            7 => Value::Tagged(t, Box::new(Value::I64(g.gen()))),
            8 => Value::Tagged(t, Box::new(Value::F32(g.gen()))),
            _ => Value::Tagged(t, Box::new(Value::F64(g.gen()))),
        },
        t @ Tag::Bignum => Value::Tagged(t, Box::new(Value::Bytes(gen_bytes(g)))),
        t @ Tag::NegativeBignum => Value::Tagged(t, Box::new(Value::Bytes(gen_bytes(g)))),
        t @ Tag::Uri => Value::Tagged(t, Box::new(Value::Text(gen_text(g)))),
        t @ Tag::Base64 => Value::Tagged(t, Box::new(Value::Text(gen_text(g)))),
        t @ Tag::ToBase64Url => Value::Tagged(t, Box::new(Value::Text(gen_text(g)))),
        t @ Tag::Regex => Value::Tagged(t, Box::new(Value::Text(gen_text(g)))),
        t @ Tag::Decimal => Value::Tagged(
            t,
            Box::new(Value::Array(vec![Value::U64(g.gen()), Value::U64(g.gen())])),
        ),
        t @ Tag::Bigfloat => Value::Tagged(
            t,
            Box::new(Value::Array(vec![Value::U64(g.gen()), Value::U64(g.gen())])),
        ),
        _ => Value::Tagged(Tag::Mime, Box::new(Value::Text(gen_text(g)))),
    }
}
