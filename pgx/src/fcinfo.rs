use crate::pg_sys;

#[cfg(any(feature = "pg10", feature = "pg11"))]
mod pg_10_11 {
    use crate::{pg_sys, DatumCompatible, PgDatum};

    #[inline]
    pub fn pg_getarg<T>(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> PgDatum<T>
    where
        T: DatumCompatible<T>,
    {
        PgDatum::<T>::new(
            unsafe { fcinfo.as_ref() }.unwrap().arg[num],
            unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool,
        )
    }

    #[inline]
    pub fn pg_arg_is_null(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> bool {
        unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool
    }

    #[inline]
    pub fn pg_getarg_datum(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> Option<pg_sys::Datum> {
        if pg_arg_is_null(fcinfo, num) {
            None
        } else {
            Some(unsafe { fcinfo.as_ref() }.unwrap().arg[num])
        }
    }

    #[inline]
    pub fn pg_getarg_datum_raw(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> pg_sys::Datum {
        unsafe { fcinfo.as_ref() }.unwrap().arg[num]
    }

    #[inline]
    pub fn pg_return_null(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        unsafe { fcinfo.as_mut() }.unwrap().isnull = true;
        0 as pg_sys::Datum
    }
}

#[cfg(feature = "pg12")]
mod pg_12 {
    use crate::{pg_sys, DatumCompatible, PgDatum};

    #[inline]
    pub fn pg_getarg<T>(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> PgDatum<T>
    where
        T: DatumCompatible<T>,
    {
        let datum = get_nullable_datum(fcinfo, num);
        PgDatum::<T>::new(datum.value, datum.isnull)
    }

    #[inline]
    pub fn pg_arg_is_null(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> bool {
        get_nullable_datum(fcinfo, num).isnull
    }

    #[inline]
    pub fn pg_getarg_datum(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> Option<pg_sys::Datum> {
        if pg_arg_is_null(fcinfo, num) {
            None
        } else {
            Some(get_nullable_datum(fcinfo, num).value)
        }
    }

    #[inline]
    pub fn pg_getarg_datum_raw(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> pg_sys::Datum {
        get_nullable_datum(fcinfo, num).value
    }

    #[inline]
    fn get_nullable_datum(
        fcinfo: pg_sys::FunctionCallInfo,
        num: usize,
    ) -> pg_sys::pg12_specific::NullableDatum {
        let fcinfo = unsafe { fcinfo.as_mut() }.unwrap();
        unsafe {
            let nargs = fcinfo.nargs;
            let len = std::mem::size_of::<pg_sys::pg12_specific::NullableDatum>() * nargs as usize;
            fcinfo.args.as_slice(len)[num]
        }
    }

    #[inline]
    pub fn pg_return_null(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        let fcinfo = unsafe { fcinfo.as_mut() }.unwrap();
        fcinfo.isnull = true;
        0 as pg_sys::Datum
    }
}

//
// common
//

#[cfg(any(feature = "pg10", feature = "pg11"))]
pub use pg_10_11::*;

#[cfg(feature = "pg12")]
pub use pg_12::*;
use std::convert::TryInto;

#[inline]
pub fn pg_getarg_pointer<T>(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> Option<*mut T> {
    match pg_getarg_datum(fcinfo, num) {
        Some(datum) => Some(datum as *mut T),
        None => None,
    }
}

/// this is intended for Postgres functions that take an actual `cstring` argument, not for getting
/// a varlena argument type as a CStr.
#[inline]
pub fn pg_getarg_cstr<'a>(
    fcinfo: pg_sys::FunctionCallInfo,
    num: usize,
) -> Option<&'a std::ffi::CStr> {
    match pg_getarg_pointer(fcinfo, num) {
        Some(ptr) => Some(unsafe { std::ffi::CStr::from_ptr(ptr) }),
        None => None,
    }
}

#[inline]
pub fn pg_return_void() -> pg_sys::Datum {
    0 as pg_sys::Datum
}

/// As `#[pg_extern]` functions are wrapped with a different signature, this
/// allows you to directly call them.
///
/// This mimics the functionality of Postgres' `DirectFunctionCall` macros, allowing you to call
/// Rust-defined functions.  Unlike Postgres' macros, the directly called function is allowed to
/// return a NULL datum.
///
/// You'll just need to account for that when using `.try_into()` to convert the datum into a rust
/// type.
///
/// ## Note
///
/// You must suffix the function name with `_wrapper`, as shown in the example below.
///
/// ## Examples
/// ```rust,no_run
/// use pgx::*;
///
/// #[pg_extern]
/// fn add_two_numbers(a: i32, b: i32) -> i32 {
///    a + b
/// }
///
/// fn some_func() {
///     let result = direct_function_call(add_two_numbers_wrapper, vec!(PgDatum::from(2), PgDatum::from(3)));
///     let sum:i32 = result.try_into().unwrap();   // don't care to check for NULL datum here
///     assert_eq!(sum, 5);
/// }
/// ```
pub fn direct_function_call<F: FnOnce(pg_sys::FunctionCallInfo) -> pg_sys::Datum>(
    func: F,
    args: Vec<crate::PgDatum<pg_sys::Datum>>,
) -> crate::PgDatum<pg_sys::Datum> {
    let mut null_array = [false; 100usize];
    let mut arg_array = [0 as pg_sys::Datum; 100usize];
    let nargs = args.len();

    for (i, datum) in args.into_iter().enumerate() {
        null_array[i] = datum.is_null();
        arg_array[i] = datum.into_datum();
    }

    let fcid = make_function_call_info(nargs, arg_array, null_array);
    let result = func(fcid);
    crate::PgDatum::new(result, unsafe { fcid.as_ref() }.unwrap().isnull)
        .try_into()
        .unwrap()
}

#[cfg(feature = "pg10")]
fn make_function_call_info(
    nargs: usize,
    arg_array: [usize; 100],
    null_array: [bool; 100],
) -> pg_sys::FunctionCallInfo {
    crate::PgBox::new_from_struct(pg_sys::pg10_specific::FunctionCallInfoData {
        flinfo: std::ptr::null_mut(),
        context: std::ptr::null_mut(),
        resultinfo: std::ptr::null_mut(),
        fncollation: crate::pg_sys::InvalidOid,
        isnull: false,
        nargs: nargs as i16,
        arg: arg_array,
        argnull: null_array,
    })
    .to_pg()
}

#[cfg(feature = "pg11")]
fn make_function_call_info(
    nargs: usize,
    arg_array: [usize; 100],
    null_array: [bool; 100],
) -> pg_sys::FunctionCallInfo {
    crate::PgBox::new_from_struct(pg_sys::pg11_specific::FunctionCallInfoData {
        flinfo: std::ptr::null_mut(),
        context: std::ptr::null_mut(),
        resultinfo: std::ptr::null_mut(),
        fncollation: crate::pg_sys::InvalidOid,
        isnull: false,
        nargs: nargs as i16,
        arg: arg_array,
        argnull: null_array,
    })
    .to_pg()
}

#[cfg(feature = "pg12")]
fn make_function_call_info(
    nargs: usize,
    arg_array: [usize; 100],
    null_array: [bool; 100],
) -> pg_sys::FunctionCallInfo {
    let fcid: *mut pg_sys::pg12_specific::FunctionCallInfoBaseData = unsafe {
        pg_sys::palloc0(
            std::mem::size_of::<pg_sys::pg12_specific::FunctionCallInfoBaseData>()
                + nargs * std::mem::size_of::<pg_sys::pg12_specific::NullableDatum>(),
        ) as *mut pg_sys::pg12_specific::FunctionCallInfoBaseData
    };

    let fcid = unsafe { fcid.as_mut() }.unwrap();
    fcid.nargs = nargs as i16;

    let slice = unsafe { fcid.args.as_mut_slice(nargs) };
    for i in 0..nargs {
        slice[i] = pg_sys::pg12_specific::NullableDatum {
            value: arg_array[i],
            isnull: null_array[i],
        }
    }

    fcid
}