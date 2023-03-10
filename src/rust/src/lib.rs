mod error;
mod integer;
mod protect;
mod real;
mod sexp;
mod string;

use anyhow::Context;
use integer::IntegerSxp;
use libR_sys::{
    cetype_t_CE_UTF8, REprintf, R_NilValue, Rf_allocVector, Rf_errorcall, Rf_mkCharLenCE, Rprintf,
    SET_INTEGER_ELT, SET_REAL_ELT, SET_STRING_ELT, SEXP,
};
use real::RealSxp;
use std::ffi::CString;
use string::StringSxp;

// TODO: make this r_println! macro
fn r_print(msg: String) {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        Rprintf(msg_c_string.as_ptr());
    }
}

fn r_eprint(msg: String) {
    unsafe {
        let msg_c_string = CString::new(msg).unwrap();
        REprintf(msg_c_string.as_ptr());
    }
}

// This wrapper function handles Error and panicks, and flag it by setting the
// lowest bit to 1. The lowest bit is supposed to be detected (and then removed)
// on the corresponding C function.
//
// cf. https://en.wikipedia.org/wiki/Tagged_pointer
pub fn wrapper<F>(f: F) -> SEXP
where
    F: FnOnce() -> anyhow::Result<SEXP>,
    F: std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        // NOTE: At first, I wrote `(res as usize & !1) as SEXP` to ensure the
        // error flag is off, but it's unnecessary because an SEXP should be an
        // aligned address, otherwise it should have failed before this point,
        // and unaligned address cannot be restored on the C function's side
        // anyway.
        Ok(Ok(res)) => res,

        // Case of an expected error
        Ok(Err(e)) => unsafe {
            let msg = e.to_string();
            let r_error = Rf_mkCharLenCE(
                msg.as_ptr() as *const i8,
                msg.len() as i32,
                cetype_t_CE_UTF8,
            );

            // set the error flag
            (r_error as usize | 1) as SEXP
        },

        // Case of an unexpected error (i.e., panic)
        Err(e) => unsafe {
            let msg = format!("{e:?}");
            let r_error = Rf_mkCharLenCE(
                msg.as_ptr() as *const i8,
                msg.len() as i32,
                cetype_t_CE_UTF8,
            );

            // set the error flag
            (r_error as usize | 1) as SEXP
        },
    }
}

unsafe fn to_upper_inner(x: SEXP) -> anyhow::Result<SEXP> {
    let x = StringSxp::try_from(x)?;

    let out = Rf_allocVector(libR_sys::STRSXP, x.len() as _);

    // // Do I need to protect here? Or, as this will be passed to R's side, it's not needed?
    // protect::PRESERVED_LIST.insert(out);

    for (i, e) in x.iter().enumerate() {
        let e_upper = e.to_uppercase();

        // Rf_mkCharLenCE() probably allocates
        let r_str = Rf_mkCharLenCE(
            e_upper.as_ptr() as *const i8,
            e_upper.len() as i32,
            cetype_t_CE_UTF8,
        );

        SET_STRING_ELT(out, i as isize, r_str);
    }

    Ok(out)
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_to_upper(x: SEXP) -> SEXP {
    wrapper(|| to_upper_inner(x))
}

unsafe fn times_two_int_inner(x: SEXP) -> anyhow::Result<SEXP> {
    let x = IntegerSxp::try_from(x)?;

    let out = Rf_allocVector(libR_sys::INTSXP, x.len() as _);

    for (i, e) in x.iter().enumerate() {
        SET_INTEGER_ELT(out, i as isize, e * 2);
    }

    Ok(out)
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_times_two_int(x: SEXP) -> SEXP {
    wrapper(|| times_two_int_inner(x))
}

unsafe fn times_two_numeric_inner(x: SEXP) -> anyhow::Result<SEXP> {
    let x = RealSxp::try_from(x)?;

    let out = Rf_allocVector(libR_sys::REALSXP, x.len() as _);

    for (i, e) in x.iter().enumerate() {
        SET_REAL_ELT(out, i as isize, e * 2.0);
    }

    Ok(out)
}

#[no_mangle]
pub unsafe extern "C" fn unextendr_times_two_numeric(x: SEXP) -> SEXP {
    wrapper(|| times_two_numeric_inner(x))
}
