use ::libc;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
use crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf;
use crate::src::opus_1_2_1::silk::tables_other::silk_stereo_only_code_mid_iCDF;
use crate::src::opus_1_2_1::silk::tables_other::silk_stereo_pred_joint_iCDF;
use crate::src::opus_1_2_1::silk::tables_other::silk_stereo_pred_quant_Q13;
use crate::src::opus_1_2_1::silk::tables_other::silk_uniform3_iCDF;
use crate::src::opus_1_2_1::silk::tables_other::silk_uniform5_iCDF;
/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* Decode mid/side predictors */
#[no_mangle]

pub unsafe extern "C" fn silk_stereo_decode_pred(
    mut psRangeDec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut pred_Q13: *mut crate::opus_types_h::opus_int32,
)
/* O    Predictors                                  */
{
    let mut n: libc::c_int = 0;
    let mut ix: [[libc::c_int; 3]; 2] = [[0; 3]; 2];
    let mut low_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut step_Q13: crate::opus_types_h::opus_int32 = 0;
    /* Entropy decoding */
    n = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
        psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        crate::src::opus_1_2_1::silk::tables_other::silk_stereo_pred_joint_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    ix[0 as libc::c_int as usize][2 as libc::c_int as usize] = n / 5 as libc::c_int;
    ix[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        n - 5 as libc::c_int * ix[0 as libc::c_int as usize][2 as libc::c_int as usize];
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ix[n as usize][0 as libc::c_int as usize] =
            crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_other::silk_uniform3_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        ix[n as usize][1 as libc::c_int as usize] =
            crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_other::silk_uniform5_iCDF.as_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
        n += 1
    }
    /* Dequantize */
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ix[n as usize][0 as libc::c_int as usize] +=
            3 as libc::c_int * ix[n as usize][2 as libc::c_int as usize];
        low_Q13 = crate::src::opus_1_2_1::silk::tables_other::silk_stereo_pred_quant_Q13
            [ix[n as usize][0 as libc::c_int as usize] as usize]
            as crate::opus_types_h::opus_int32;
        step_Q13 = ((crate::src::opus_1_2_1::silk::tables_other::silk_stereo_pred_quant_Q13
            [(ix[n as usize][0 as libc::c_int as usize] + 1 as libc::c_int) as usize]
            as libc::c_int
            - low_Q13) as libc::c_longlong
            * (0.5f64 / 5 as libc::c_int as libc::c_double
                * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_int16 as libc::c_longlong
            >> 16 as libc::c_int) as crate::opus_types_h::opus_int32;
        *pred_Q13.offset(n as isize) = low_Q13
            + step_Q13 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                * (2 as libc::c_int * ix[n as usize][1 as libc::c_int as usize] + 1 as libc::c_int)
                    as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32;
        n += 1
    }
    /* Subtract second from first predictor (helps when actually applying these) */
    let ref mut fresh0 = *pred_Q13.offset(0 as libc::c_int as isize);
    *fresh0 -= *pred_Q13.offset(1 as libc::c_int as isize);
}
/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* Convert Left/Right stereo signal to adaptive Mid/Side representation */
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* O    Quantization indices                        */
/* O    Flag: only mid signal coded                 */
/* O    Bitrates for mid and side signals           */
/* I    Total bitrate                               */
/* I    Speech activity level in previous frame     */
/* I    Last frame before a stereo->mono transition */
/* I    Sample rate (kHz)                           */
/* I    Number of samples                           */
/* Convert adaptive Mid/Side representation to Left/Right stereo signal */
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* I    Predictors                                  */
/* I    Samples rate (kHz)                          */
/* I    Number of samples                           */
/* Find least-squares prediction gain for one signal based on another and quantize it */
/* O    Returns predictor in Q13                    */
/* O    Ratio of residual and mid energies          */
/* I    Basis signal                                */
/* I    Target signal                               */
/* I/O  Smoothed mid, residual norms                */
/* I    Number of samples                           */
/* I    Smoothing coefficient                       */
/* Quantize mid/side predictors */
/* I/O  Predictors (out: quantized)                 */
/* O    Quantization indices                        */
/* Entropy code the mid/side quantization indices */
/* I/O  Compressor data structure                   */
/* I    Quantization indices                        */
/* Entropy code the mid-only flag */
/* I/O  Compressor data structure                   */
/* Decode mid/side predictors */
/* I/O  Compressor data structure                   */
/* O    Predictors                                  */
/* Decode mid-only flag */
/* Decode mid-only flag */
#[no_mangle]

pub unsafe extern "C" fn silk_stereo_decode_mid_only(
    mut psRangeDec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut decode_only_mid: *mut libc::c_int,
)
/* O    Flag that only mid channel has been coded   */
{
    /* Decode flag that only mid channel is coded */
    *decode_only_mid = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
        psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        crate::src::opus_1_2_1::silk::tables_other::silk_stereo_only_code_mid_iCDF.as_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
}
