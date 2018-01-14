// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

use decoder::DecodeError;
use encoder::EncodeError;
use sd::ser;
use std::fmt::Display;

pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Encode(e: EncodeError) {
            description("serialization error")
            display("serialization error: {}", e)
            cause(e)
            from()
        }
        Decode(e: DecodeError) {
            description("deserialization error")
            display("deserialization error: {}", e)
            cause(e)
            from()
        }
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Encode(EncodeError::Other(msg.to_string().into()))
    }
}
