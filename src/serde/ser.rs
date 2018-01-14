// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

use byteorder::WriteBytesExt;
use encoder::Encoder;
use serde::error::{Error, Result};
use sd::ser::{self, Serialize};

pub struct Serializer<W> {
    encoder: Encoder<W>
}

impl<W: WriteBytesExt> Serializer<W> {
    pub fn new(w: W) -> Self {
        Serializer {
            encoder: Encoder::new(w)
        }
    }
}

impl<'a, W: WriteBytesExt> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, W>;
    type SerializeTuple = SeqSerializer<'a, W>;
    type SerializeTupleStruct = SeqSerializer<'a, W>;
    type SerializeTupleVariant = SeqSerializer<'a, W>;
    type SerializeMap = SeqSerializer<'a, W>;
    type SerializeStruct = SeqSerializer<'a, W>;
    type SerializeStructVariant = SeqSerializer<'a, W>;

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_bool(self, x: bool) -> Result<Self::Ok> {
        self.encoder.bool(x).map_err(From::from)
    }

    fn serialize_i8(self, x: i8) -> Result<Self::Ok> {
        self.encoder.i8(x).map_err(From::from)
    }

    fn serialize_i16(self, x: i16) -> Result<Self::Ok> {
        self.encoder.i16(x).map_err(From::from)
    }

    fn serialize_i32(self, x: i32) -> Result<Self::Ok> {
        self.encoder.i32(x).map_err(From::from)
    }

    fn serialize_i64(self, x: i64) -> Result<Self::Ok> {
        self.encoder.i64(x).map_err(From::from)
    }

    fn serialize_u8(self, x: u8) -> Result<Self::Ok> {
        self.encoder.u8(x).map_err(From::from)
    }

    fn serialize_u16(self, x: u16) -> Result<Self::Ok> {
        self.encoder.u16(x).map_err(From::from)
    }

    fn serialize_u32(self, x: u32) -> Result<Self::Ok> {
        self.encoder.u32(x).map_err(From::from)
    }

    fn serialize_u64(self, x: u64) -> Result<Self::Ok> {
        self.encoder.u64(x).map_err(From::from)
    }

    fn serialize_f32(self, x: f32) -> Result<Self::Ok> {
        self.encoder.f32(x).map_err(From::from)
    }

    fn serialize_f64(self, x: f64) -> Result<Self::Ok> {
        self.encoder.f64(x).map_err(From::from)
    }

    fn serialize_char(self, x: char) -> Result<Self::Ok> {
        self.encoder.u32(x as u32).map_err(From::from)
    }

    fn serialize_str(self, x: &str) -> Result<Self::Ok> {
        self.encoder.text(x).map_err(From::from)
    }

    fn serialize_bytes(self, x: &[u8]) -> Result<Self::Ok> {
        self.encoder.bytes(x).map_err(From::from)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.encoder.null().map_err(From::from)
    }

    fn serialize_some<T: Serialize + ?Sized>(self, x: &T) -> Result<Self::Ok> {
        x.serialize(self)
    }

    fn serialize_unit_struct(self, _n: &'static str) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(self, _n: &'static str, idx: u32, _var: &'static str) -> Result<Self::Ok> {
        self.encoder.u32(idx).map_err(From::from)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _n: &'static str, x: &T) -> Result<Self::Ok>
        where T: Serialize
    {
        x.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _n: &'static str, idx: u32, _var: &'static str, x: &T) -> Result<Self::Ok>
        where T: Serialize
    {
        self.encoder.u32(idx)?;
        x.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(n) => self.encoder.array(n)?,
            None    => self.encoder.array_begin()?
        }
        Ok(SeqSerializer { serializer: self, indefinite: len.is_none() })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeSeq> {
        self.encoder.array(len)?;
        Ok(SeqSerializer { serializer: self, indefinite: false })
    }

    fn serialize_tuple_struct(self, _n: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        self.encoder.array(len)?;
        Ok(SeqSerializer { serializer: self, indefinite: false })
    }

    fn serialize_tuple_variant(self, _n: &'static str, idx: u32, _var: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
        self.encoder.u32(idx)?;
        self.encoder.array(len)?;
        Ok(SeqSerializer { serializer: self, indefinite: false })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        match len {
            Some(n) => self.encoder.object(n)?,
            None    => self.encoder.object_begin()?
        }
        Ok(SeqSerializer { serializer: self, indefinite: len.is_none() })
    }

    fn serialize_struct(self, _n: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.encoder.object(len)?;
        Ok(SeqSerializer { serializer: self, indefinite: false })
    }

    fn serialize_struct_variant(self, _n: &'static str, idx: u32, _var: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
        self.encoder.u32(idx)?;
        self.encoder.object(len)?;
        Ok(SeqSerializer { serializer: self, indefinite: false })
    }
}

pub struct SeqSerializer<'a, W: 'a> {
    serializer: &'a mut Serializer<W>,
    indefinite: bool
}

impl<'a, W: WriteBytesExt> ser::SerializeSeq for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, x: &T) -> Result<()> {
        x.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        if self.indefinite {
            self.serializer.encoder.array_end()?
        }
        Ok(())
    }
}

impl<'a, W: WriteBytesExt> ser::SerializeTuple for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, x: &T) -> Result<()> {
        x.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: WriteBytesExt> ser::SerializeTupleStruct for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, x: &T) -> Result<()> {
        x.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: WriteBytesExt> ser::SerializeTupleVariant for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, x: &T) -> Result<()> {
        x.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: WriteBytesExt> ser::SerializeMap for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, k: &T) -> Result<()> {
        k.serialize(&mut *self.serializer)
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<()> {
        v.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        if self.indefinite {
            self.serializer.encoder.object_end()?
        }
        Ok(())
    }
}

impl<'a, W: WriteBytesExt> ser::SerializeStruct for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, k: &'static str, v: &T) -> Result<()>
        where T: Serialize
    {
        k.serialize(&mut *self.serializer)?;
        v.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: WriteBytesExt> ser::SerializeStructVariant for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, k: &'static str, v: &T) -> Result<()>
        where T: Serialize
    {
        k.serialize(&mut *self.serializer)?;
        v.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
