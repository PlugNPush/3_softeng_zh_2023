use core::str::Utf8Error;

const MAX_PARAMS: usize = 8;

pub trait Cobs<'a> {
    /// Encode a command with the COBS algorithm.
    ///
    /// Return a tuple of the structure (<encoded_length>, <output_array>)
    fn as_cobs<const OUTPUT: usize>(&self) -> (usize, [u8; OUTPUT]);
}

pub trait Cmd<'a> {
    fn from_bytes<const INPUT: usize>(
        input: [u8; INPUT],
        buffer: &'a mut [u8; INPUT],
    ) -> Result<Self, Utf8Error>
    where
        Self: Sized;

    /// Read parameters from a byte array (not COBS encoded).
    ///
    /// Every parameter is interpreted as an UTF-8 string.
    ///
    /// Return a string array of size `MAX_PARAMS`.
    fn params_from_bytes<const INPUT: usize, const PARAMS: usize>(
        input: [u8; INPUT],
        buffer: &mut [u8; INPUT],
    ) -> Result<[&str; MAX_PARAMS], Utf8Error> {
        buffer.copy_from_slice(&input);

        let param_count = buffer[0] as usize;
        assert_eq!(param_count, PARAMS);
        assert!(param_count <= MAX_PARAMS);

        let param_offset = param_count + 1;
        let mut param_idx = 0;
        let mut params: [&str; MAX_PARAMS] = [""; MAX_PARAMS];
        for i in 1..=param_count {
            let param_length = buffer[i] as usize;
            let param_start = param_idx + param_offset;
            let param = &buffer[param_start..param_start + param_length];
            let param = core::str::from_utf8(param)?;
            params[i - 1] = param;

            param_idx += param_length;
        }

        Ok(params)
    }
}
