/* automatically generated by rust-bindgen 0.59.1 */

#![allow(non_camel_case_types)]

extern crate libloading;
pub struct MlModule {
    __library: ::libloading::Library,
    pub prepare: Result<
        unsafe extern "C" fn(json_params: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub predict: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            output: *mut f64,
            json_params: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub release: Result<
        unsafe extern "C" fn(json_params: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub set_log_level: Result<
        unsafe extern "C" fn(log_level: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub set_log_file: Result<
        unsafe extern "C" fn(log_file: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
}
impl MlModule {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let prepare = __library.get(b"prepare\0").map(|sym| *sym);
        let predict = __library.get(b"predict\0").map(|sym| *sym);
        let release = __library.get(b"release\0").map(|sym| *sym);
        let set_log_level = __library.get(b"set_log_level\0").map(|sym| *sym);
        let set_log_file = __library.get(b"set_log_file\0").map(|sym| *sym);
        Ok(MlModule {
            __library,
            prepare,
            predict,
            release,
            set_log_level,
            set_log_file,
        })
    }
    pub unsafe fn prepare(
        &self,
        json_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        (self
            .prepare
            .as_ref()
            .expect("Expected function, got error."))(json_params)
    }
    pub unsafe fn predict(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        output: *mut f64,
        json_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        (self
            .predict
            .as_ref()
            .expect("Expected function, got error."))(data, data_len, output, json_params)
    }
    pub unsafe fn release(
        &self,
        json_params: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        (self
            .release
            .as_ref()
            .expect("Expected function, got error."))(json_params)
    }
    pub unsafe fn set_log_level(&self, log_level: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        (self
            .set_log_level
            .as_ref()
            .expect("Expected function, got error."))(log_level)
    }
    pub unsafe fn set_log_file(
        &self,
        log_file: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        (self
            .set_log_file
            .as_ref()
            .expect("Expected function, got error."))(log_file)
    }
}
