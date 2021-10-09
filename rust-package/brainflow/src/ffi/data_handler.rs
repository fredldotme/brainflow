/* automatically generated by rust-bindgen 0.59.1 */

#![allow(non_camel_case_types)]

extern crate libloading;
pub struct DataHandler {
    __library: ::libloading::Library,
    pub perform_lowpass: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            sampling_rate: ::std::os::raw::c_int,
            cutoff: f64,
            order: ::std::os::raw::c_int,
            filter_type: ::std::os::raw::c_int,
            ripple: f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_highpass: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            sampling_rate: ::std::os::raw::c_int,
            cutoff: f64,
            order: ::std::os::raw::c_int,
            filter_type: ::std::os::raw::c_int,
            ripple: f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_bandpass: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            sampling_rate: ::std::os::raw::c_int,
            center_freq: f64,
            band_width: f64,
            order: ::std::os::raw::c_int,
            filter_type: ::std::os::raw::c_int,
            ripple: f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_bandstop: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            sampling_rate: ::std::os::raw::c_int,
            center_freq: f64,
            band_width: f64,
            order: ::std::os::raw::c_int,
            filter_type: ::std::os::raw::c_int,
            ripple: f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub remove_environmental_noise: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            sampling_rate: ::std::os::raw::c_int,
            noise_type: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_rolling_filter: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            period: ::std::os::raw::c_int,
            agg_operation: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_downsampling: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            period: ::std::os::raw::c_int,
            agg_operation: ::std::os::raw::c_int,
            output_data: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_wavelet_transform: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            wavelet: *const ::std::os::raw::c_char,
            decomposition_level: ::std::os::raw::c_int,
            output_data: *mut f64,
            decomposition_lengths: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_inverse_wavelet_transform: Result<
        unsafe extern "C" fn(
            wavelet_coeffs: *mut f64,
            original_data_len: ::std::os::raw::c_int,
            wavelet: *const ::std::os::raw::c_char,
            decomposition_level: ::std::os::raw::c_int,
            decomposition_lengths: *mut ::std::os::raw::c_int,
            output_data: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_wavelet_denoising: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            wavelet: *const ::std::os::raw::c_char,
            decomposition_level: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub get_csp: Result<
        unsafe extern "C" fn(
            data: *const f64,
            labels: *const f64,
            n_epochs: ::std::os::raw::c_int,
            n_channels: ::std::os::raw::c_int,
            n_times: ::std::os::raw::c_int,
            output_w: *mut f64,
            output_d: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub get_window: Result<
        unsafe extern "C" fn(
            window_function: ::std::os::raw::c_int,
            window_len: ::std::os::raw::c_int,
            output_window: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_fft: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            window_function: ::std::os::raw::c_int,
            output_re: *mut f64,
            output_im: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub perform_ifft: Result<
        unsafe extern "C" fn(
            input_re: *mut f64,
            input_im: *mut f64,
            data_len: ::std::os::raw::c_int,
            restored_data: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub get_nearest_power_of_two: Result<
        unsafe extern "C" fn(
            value: ::std::os::raw::c_int,
            output: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub get_psd: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            sampling_rate: ::std::os::raw::c_int,
            window_function: ::std::os::raw::c_int,
            output_ampl: *mut f64,
            output_freq: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub detrend: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            detrend_operation: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub get_psd_welch: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            data_len: ::std::os::raw::c_int,
            nfft: ::std::os::raw::c_int,
            overlap: ::std::os::raw::c_int,
            sampling_rate: ::std::os::raw::c_int,
            window_function: ::std::os::raw::c_int,
            output_ampl: *mut f64,
            output_freq: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub get_band_power: Result<
        unsafe extern "C" fn(
            ampl: *mut f64,
            freq: *mut f64,
            data_len: ::std::os::raw::c_int,
            freq_start: f64,
            freq_end: f64,
            band_power: *mut f64,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub get_avg_band_powers: Result<
        unsafe extern "C" fn(
            raw_data: *mut f64,
            rows: ::std::os::raw::c_int,
            cols: ::std::os::raw::c_int,
            sampling_rate: ::std::os::raw::c_int,
            apply_filters: ::std::os::raw::c_int,
            avg_band_powers: *mut f64,
            stddev_band_powers: *mut f64,
        ) -> ::std::os::raw::c_int,
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
    pub write_file: Result<
        unsafe extern "C" fn(
            data: *const f64,
            num_rows: ::std::os::raw::c_int,
            num_cols: ::std::os::raw::c_int,
            file_name: *const ::std::os::raw::c_char,
            file_mode: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub read_file: Result<
        unsafe extern "C" fn(
            data: *mut f64,
            num_rows: *mut ::std::os::raw::c_int,
            num_cols: *mut ::std::os::raw::c_int,
            file_name: *const ::std::os::raw::c_char,
            num_elements: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub get_num_elements_in_file: Result<
        unsafe extern "C" fn(
            file_name: *const ::std::os::raw::c_char,
            num_elements: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
}
impl DataHandler {
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
        let perform_lowpass = __library.get(b"perform_lowpass\0").map(|sym| *sym);
        let perform_highpass = __library.get(b"perform_highpass\0").map(|sym| *sym);
        let perform_bandpass = __library.get(b"perform_bandpass\0").map(|sym| *sym);
        let perform_bandstop = __library.get(b"perform_bandstop\0").map(|sym| *sym);
        let remove_environmental_noise = __library
            .get(b"remove_environmental_noise\0")
            .map(|sym| *sym);
        let perform_rolling_filter = __library.get(b"perform_rolling_filter\0").map(|sym| *sym);
        let perform_downsampling = __library.get(b"perform_downsampling\0").map(|sym| *sym);
        let perform_wavelet_transform = __library
            .get(b"perform_wavelet_transform\0")
            .map(|sym| *sym);
        let perform_inverse_wavelet_transform = __library
            .get(b"perform_inverse_wavelet_transform\0")
            .map(|sym| *sym);
        let perform_wavelet_denoising = __library
            .get(b"perform_wavelet_denoising\0")
            .map(|sym| *sym);
        let get_csp = __library.get(b"get_csp\0").map(|sym| *sym);
        let get_window = __library.get(b"get_window\0").map(|sym| *sym);
        let perform_fft = __library.get(b"perform_fft\0").map(|sym| *sym);
        let perform_ifft = __library.get(b"perform_ifft\0").map(|sym| *sym);
        let get_nearest_power_of_two = __library.get(b"get_nearest_power_of_two\0").map(|sym| *sym);
        let get_psd = __library.get(b"get_psd\0").map(|sym| *sym);
        let detrend = __library.get(b"detrend\0").map(|sym| *sym);
        let get_psd_welch = __library.get(b"get_psd_welch\0").map(|sym| *sym);
        let get_band_power = __library.get(b"get_band_power\0").map(|sym| *sym);
        let get_avg_band_powers = __library.get(b"get_avg_band_powers\0").map(|sym| *sym);
        let set_log_level = __library.get(b"set_log_level\0").map(|sym| *sym);
        let set_log_file = __library.get(b"set_log_file\0").map(|sym| *sym);
        let write_file = __library.get(b"write_file\0").map(|sym| *sym);
        let read_file = __library.get(b"read_file\0").map(|sym| *sym);
        let get_num_elements_in_file = __library.get(b"get_num_elements_in_file\0").map(|sym| *sym);
        Ok(DataHandler {
            __library,
            perform_lowpass,
            perform_highpass,
            perform_bandpass,
            perform_bandstop,
            remove_environmental_noise,
            perform_rolling_filter,
            perform_downsampling,
            perform_wavelet_transform,
            perform_inverse_wavelet_transform,
            perform_wavelet_denoising,
            get_csp,
            get_window,
            perform_fft,
            perform_ifft,
            get_nearest_power_of_two,
            get_psd,
            detrend,
            get_psd_welch,
            get_band_power,
            get_avg_band_powers,
            set_log_level,
            set_log_file,
            write_file,
            read_file,
            get_num_elements_in_file,
        })
    }
    pub unsafe fn perform_lowpass(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        sampling_rate: ::std::os::raw::c_int,
        cutoff: f64,
        order: ::std::os::raw::c_int,
        filter_type: ::std::os::raw::c_int,
        ripple: f64,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_lowpass
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            sampling_rate,
            cutoff,
            order,
            filter_type,
            ripple,
        )
    }
    pub unsafe fn perform_highpass(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        sampling_rate: ::std::os::raw::c_int,
        cutoff: f64,
        order: ::std::os::raw::c_int,
        filter_type: ::std::os::raw::c_int,
        ripple: f64,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_highpass
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            sampling_rate,
            cutoff,
            order,
            filter_type,
            ripple,
        )
    }
    pub unsafe fn perform_bandpass(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        sampling_rate: ::std::os::raw::c_int,
        center_freq: f64,
        band_width: f64,
        order: ::std::os::raw::c_int,
        filter_type: ::std::os::raw::c_int,
        ripple: f64,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_bandpass
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            sampling_rate,
            center_freq,
            band_width,
            order,
            filter_type,
            ripple,
        )
    }
    pub unsafe fn perform_bandstop(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        sampling_rate: ::std::os::raw::c_int,
        center_freq: f64,
        band_width: f64,
        order: ::std::os::raw::c_int,
        filter_type: ::std::os::raw::c_int,
        ripple: f64,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_bandstop
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            sampling_rate,
            center_freq,
            band_width,
            order,
            filter_type,
            ripple,
        )
    }
    pub unsafe fn remove_environmental_noise(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        sampling_rate: ::std::os::raw::c_int,
        noise_type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .remove_environmental_noise
            .as_ref()
            .expect("Expected function, got error."))(
            data, data_len, sampling_rate, noise_type
        )
    }
    pub unsafe fn perform_rolling_filter(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        period: ::std::os::raw::c_int,
        agg_operation: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_rolling_filter
            .as_ref()
            .expect("Expected function, got error."))(data, data_len, period, agg_operation)
    }
    pub unsafe fn perform_downsampling(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        period: ::std::os::raw::c_int,
        agg_operation: ::std::os::raw::c_int,
        output_data: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_downsampling
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            period,
            agg_operation,
            output_data,
        )
    }
    pub unsafe fn perform_wavelet_transform(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        wavelet: *const ::std::os::raw::c_char,
        decomposition_level: ::std::os::raw::c_int,
        output_data: *mut f64,
        decomposition_lengths: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_wavelet_transform
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            wavelet,
            decomposition_level,
            output_data,
            decomposition_lengths,
        )
    }
    pub unsafe fn perform_inverse_wavelet_transform(
        &self,
        wavelet_coeffs: *mut f64,
        original_data_len: ::std::os::raw::c_int,
        wavelet: *const ::std::os::raw::c_char,
        decomposition_level: ::std::os::raw::c_int,
        decomposition_lengths: *mut ::std::os::raw::c_int,
        output_data: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_inverse_wavelet_transform
            .as_ref()
            .expect("Expected function, got error."))(
            wavelet_coeffs,
            original_data_len,
            wavelet,
            decomposition_level,
            decomposition_lengths,
            output_data,
        )
    }
    pub unsafe fn perform_wavelet_denoising(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        wavelet: *const ::std::os::raw::c_char,
        decomposition_level: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_wavelet_denoising
            .as_ref()
            .expect("Expected function, got error."))(
            data, data_len, wavelet, decomposition_level
        )
    }
    pub unsafe fn get_csp(
        &self,
        data: *const f64,
        labels: *const f64,
        n_epochs: ::std::os::raw::c_int,
        n_channels: ::std::os::raw::c_int,
        n_times: ::std::os::raw::c_int,
        output_w: *mut f64,
        output_d: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .get_csp
            .as_ref()
            .expect("Expected function, got error."))(
            data, labels, n_epochs, n_channels, n_times, output_w, output_d,
        )
    }
    pub unsafe fn get_window(
        &self,
        window_function: ::std::os::raw::c_int,
        window_len: ::std::os::raw::c_int,
        output_window: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .get_window
            .as_ref()
            .expect("Expected function, got error."))(
            window_function, window_len, output_window
        )
    }
    pub unsafe fn perform_fft(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        window_function: ::std::os::raw::c_int,
        output_re: *mut f64,
        output_im: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_fft
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            window_function,
            output_re,
            output_im,
        )
    }
    pub unsafe fn perform_ifft(
        &self,
        input_re: *mut f64,
        input_im: *mut f64,
        data_len: ::std::os::raw::c_int,
        restored_data: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .perform_ifft
            .as_ref()
            .expect("Expected function, got error."))(
            input_re, input_im, data_len, restored_data
        )
    }
    pub unsafe fn get_nearest_power_of_two(
        &self,
        value: ::std::os::raw::c_int,
        output: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .get_nearest_power_of_two
            .as_ref()
            .expect("Expected function, got error."))(value, output)
    }
    pub unsafe fn get_psd(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        sampling_rate: ::std::os::raw::c_int,
        window_function: ::std::os::raw::c_int,
        output_ampl: *mut f64,
        output_freq: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .get_psd
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            sampling_rate,
            window_function,
            output_ampl,
            output_freq,
        )
    }
    pub unsafe fn detrend(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        detrend_operation: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .detrend
            .as_ref()
            .expect("Expected function, got error."))(data, data_len, detrend_operation)
    }
    pub unsafe fn get_psd_welch(
        &self,
        data: *mut f64,
        data_len: ::std::os::raw::c_int,
        nfft: ::std::os::raw::c_int,
        overlap: ::std::os::raw::c_int,
        sampling_rate: ::std::os::raw::c_int,
        window_function: ::std::os::raw::c_int,
        output_ampl: *mut f64,
        output_freq: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .get_psd_welch
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            data_len,
            nfft,
            overlap,
            sampling_rate,
            window_function,
            output_ampl,
            output_freq,
        )
    }
    pub unsafe fn get_band_power(
        &self,
        ampl: *mut f64,
        freq: *mut f64,
        data_len: ::std::os::raw::c_int,
        freq_start: f64,
        freq_end: f64,
        band_power: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .get_band_power
            .as_ref()
            .expect("Expected function, got error."))(
            ampl, freq, data_len, freq_start, freq_end, band_power,
        )
    }
    pub unsafe fn get_avg_band_powers(
        &self,
        raw_data: *mut f64,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        sampling_rate: ::std::os::raw::c_int,
        apply_filters: ::std::os::raw::c_int,
        avg_band_powers: *mut f64,
        stddev_band_powers: *mut f64,
    ) -> ::std::os::raw::c_int {
        (self
            .get_avg_band_powers
            .as_ref()
            .expect("Expected function, got error."))(
            raw_data,
            rows,
            cols,
            sampling_rate,
            apply_filters,
            avg_band_powers,
            stddev_band_powers,
        )
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
    pub unsafe fn write_file(
        &self,
        data: *const f64,
        num_rows: ::std::os::raw::c_int,
        num_cols: ::std::os::raw::c_int,
        file_name: *const ::std::os::raw::c_char,
        file_mode: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        (self
            .write_file
            .as_ref()
            .expect("Expected function, got error."))(
            data, num_rows, num_cols, file_name, file_mode,
        )
    }
    pub unsafe fn read_file(
        &self,
        data: *mut f64,
        num_rows: *mut ::std::os::raw::c_int,
        num_cols: *mut ::std::os::raw::c_int,
        file_name: *const ::std::os::raw::c_char,
        num_elements: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .read_file
            .as_ref()
            .expect("Expected function, got error."))(
            data,
            num_rows,
            num_cols,
            file_name,
            num_elements,
        )
    }
    pub unsafe fn get_num_elements_in_file(
        &self,
        file_name: *const ::std::os::raw::c_char,
        num_elements: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .get_num_elements_in_file
            .as_ref()
            .expect("Expected function, got error."))(file_name, num_elements)
    }
}
