// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::serialize::serializer::PyObjectSerializer;
use crate::typeref::VALUE_STR;
use crate::typeref::NAME_STR;
use serde::ser::{Serialize, Serializer};
use crate::opt::{ENUM_NAME, Opt};

pub struct EnumSerializer<'a> {
    previous: &'a PyObjectSerializer,
    opts: Opt,
}

impl<'a> EnumSerializer<'a> {
    pub fn new(previous: &'a PyObjectSerializer, opts: Opt) -> Self {
        Self {
            previous: previous,
            opts: opts,
        }
    }
}

impl<'a> Serialize for EnumSerializer<'a> {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = if opt_enabled!(self.opts, ENUM_NAME) {
            ffi!(PyObject_GetAttr(self.previous.ptr, NAME_STR))
        } else {
            ffi!(PyObject_GetAttr(self.previous.ptr, VALUE_STR))
        };
        debug_assert!(ffi!(Py_REFCNT(value)) >= 2);
        let ret = PyObjectSerializer::new(value, self.previous.state, self.previous.default)
            .serialize(serializer);
        ffi!(Py_DECREF(value));
        ret
    }
}
