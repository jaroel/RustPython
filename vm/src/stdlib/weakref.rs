//! Implementation in line with the python `weakref` module.
//!
//! See also:
//! - [python weakref module](https://docs.python.org/3/library/weakref.html)
//! - [rust weak struct](https://doc.rust-lang.org/std/rc/struct.Weak.html)
//!

use crate::pyobject::{
    PyContext, PyFuncArgs, PyObject, PyObjectPayload, PyObjectRef, PyObjectWeakRef, PyResult,
    TypeProtocol,
};
use crate::VirtualMachine;
use std::rc::Rc;

pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
    let py_ref_class = py_class!(ctx, "ref", ctx.object(), {
        "__new__" => ctx.new_rustfunc(ref_new),
        "__call__" => ctx.new_rustfunc(ref_call)
    });

    py_module!(ctx, "_weakref", {
        "ref" => py_ref_class
    })
}

fn ref_new(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    // TODO: check first argument for subclass of `ref`.
    arg_check!(vm, args, required = [(cls, None), (referent, None)]);
    let referent = Rc::downgrade(referent);
    Ok(PyObject::new(
        PyObjectPayload::WeakRef { referent },
        cls.clone(),
    ))
}

/// Dereference the weakref, and check if we still refer something.
fn ref_call(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    // TODO: check first argument for subclass of `ref`.
    arg_check!(vm, args, required = [(cls, None)]);
    let referent = get_value(cls);
    let py_obj = if let Some(obj) = referent.upgrade() {
        obj
    } else {
        vm.get_none()
    };
    Ok(py_obj)
}

fn get_value(obj: &PyObjectRef) -> PyObjectWeakRef {
    if let PyObjectPayload::WeakRef { referent } = &obj.payload {
        referent.clone()
    } else {
        panic!("Inner error getting weak ref {:?}", obj);
    }
}
