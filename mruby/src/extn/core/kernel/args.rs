use std::io::Write;
use std::mem;

use crate::sys;
use crate::value::Value;
use crate::{Mrb, MrbError};

#[derive(Debug)]
pub struct Rest {
    pub rest: Vec<Value>,
}

impl Rest {
    pub unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
        let args = mem::uninitialized::<*const sys::mrb_value>();
        let count = mem::uninitialized::<usize>();
        let mut argspec = vec![];
        argspec
            .write_all(sys::specifiers::REST.as_bytes())
            .map_err(|_| MrbError::ArgSpec)?;
        argspec.write_all(b"\0").map_err(|_| MrbError::ArgSpec)?;
        sys::mrb_get_args(
            interp.borrow().mrb,
            argspec.as_ptr() as *const i8,
            &args,
            &count,
        );
        let args = std::slice::from_raw_parts(args, count);
        let args = args
            .iter()
            .map(|value| Value::new(&interp, *value))
            .collect::<Vec<_>>();
        Ok(Self { rest: args })
    }
}
