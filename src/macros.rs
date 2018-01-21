// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! object_u8 {
    (
      let decoder = $dec: ident;
      $T: ident {
          $($name: ident: $key: expr => $action: expr),+
      }
    ) => {{
        let size = $dec.object()?;
        $(let mut $name = None;)+
        for _ in 0 .. size {
            match $dec.u8()? {
                $($key => { $name = Some($action?) })+
                _      => { $dec.skip()? }
            }
        }
        Ok($T {
            $($name: to_field!("index not found", $name),)+
        })
    }};
}

#[macro_export]
macro_rules! to_field {
    ($msg: expr, $e: expr) => {
        match $e {
            Some(e) => e,
            None    => return Err(::std::convert::From::from($crate::DecodeError::Other($msg.into())))
        }
    }
}

