use ::libc;

pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The number of bits to use for the range-coded part of unsigned integers.*/
    /*The resolution of fractional-precision bit usage measurements, i.e.,
    3 => 1/8th bits.*/
    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    /*Returns the number of bits "used" by the encoded or decoded symbols so far.
    This same number can be computed in either the encoder or the decoder, and is
     suitable for making coding decisions.
    Return: The number of bits.
            This will always be slightly larger than the exact value (e.g., all
             rounding error is in the positive direction).*/
    #[inline]

    pub unsafe extern "C" fn ec_tell(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> libc::c_int {
        return (*_this).nbits_total
            - (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - (*_this).rng.leading_zeros() as i32);
    }
}

pub mod SigProc_FIX_h {
    /* Allocate opus_int16 aligned to 4-byte memory address */
    /* Useful Macros that can be adjusted to other platforms */
    /* Fixed point macros */
    /* (a32 * b32) output have to be 32bit int */
    /* (a32 * b32) output have to be 32bit uint */
    /* a32 + (b32 * c32) output have to be 32bit int */
    /* a32 + (b32 * c32) output have to be 32bit uint */
    /* ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* a32 + ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* (a32 * b32) */
    /*(opus_int64)*/
    /* Adds two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Subtractss two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Multiply-accumulate macros that allow overflow in the addition (ie, no asserts in debug mode) */
    /* These macros enables checking for overflow in silk_API_Debug.h*/
    /* Saturation for positive input values */
    /* Add with saturation for positive input values */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* saturates before shifting */
    /* shift >= 0, allowed to overflow */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* Requires that shift > 0 */
    /* Number of rightshift required to fit the multiplication */
    /* Macro to convert floating-point constants to fixed-point */
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_min_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
        return if a < b { a } else { b };
    }

    /* SILK_SIGPROC_FIX_H */
    /*    silk_SMMUL: Signed top word multiply.
    ARMv6        2 instruction cycles.
    ARMv3M+      3 instruction cycles. use SMULL and ignore LSB registers.(except xM)*/
    /*#define silk_SMMUL(a32, b32)                (opus_int32)silk_RSHIFT(silk_SMLAL(silk_SMULWB((a32), (b32)), (a32), silk_RSHIFT_ROUND((b32), 16)), 16)*/
    /* the following seems faster on x86 */
    /*  Add some multiplication functions that can be easily mapped to ARM. */
    /* PSEUDO-RANDOM GENERATOR                                                          */
    /* Make sure to store the result as the seed for the next call (also in between     */
    /* frames), otherwise result won't be random at all. When only using some of the    */
    /* bits, take the most significant bits by right-shifting.                          */
    /* Be careful, silk_abs returns wrong when input equals to silk_intXX_MIN */
}

pub mod SigProc_FLP_h {
    /* integer to floating-point conversion */
    #[inline]

    pub unsafe extern "C" fn silk_short2float_array(
        mut out: *mut libc::c_float,
        mut in_0: *const crate::opus_types_h::opus_int16,
        mut length: crate::opus_types_h::opus_int32,
    ) {
        let mut k: crate::opus_types_h::opus_int32 = 0;
        k = length - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            *out.offset(k as isize) = *in_0.offset(k as isize) as libc::c_float;
            k -= 1
        }
    }
    use crate::opus_types_h::opus_int32;
    /* SILK_SIGPROC_FLP_H */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::silk::float::encode_frame_FLP::entcode_h::ec_tell;
pub use crate::structs_FLP_h::silk_encoder_control_FLP;
pub use crate::structs_FLP_h::silk_encoder_state_FLP;
pub use crate::structs_FLP_h::silk_shape_state_FLP;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::SideInfoIndices;
use ::libc::abs;

pub use crate::src::opus_1_2_1::silk::float::encode_frame_FLP::SigProc_FIX_h::silk_min_int;
pub use crate::src::opus_1_2_1::silk::float::encode_frame_FLP::SigProc_FLP_h::silk_short2float_array;
pub use crate::src::opus_1_2_1::silk::log2lin::silk_log2lin;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;

use crate::src::opus_1_2_1::silk::encode_indices::silk_encode_indices;
use crate::src::opus_1_2_1::silk::encode_pulses::silk_encode_pulses;
use crate::src::opus_1_2_1::silk::float::find_pitch_lags_FLP::silk_find_pitch_lags_FLP;
use crate::src::opus_1_2_1::silk::float::find_pred_coefs_FLP::silk_find_pred_coefs_FLP;
use crate::src::opus_1_2_1::silk::float::noise_shape_analysis_FLP::silk_noise_shape_analysis_FLP;
use crate::src::opus_1_2_1::silk::float::process_gains_FLP::silk_process_gains_FLP;
use crate::src::opus_1_2_1::silk::float::wrappers_FLP::silk_NSQ_wrapper_FLP;
use crate::src::opus_1_2_1::silk::gain_quant::silk_gains_ID;
use crate::src::opus_1_2_1::silk::gain_quant::silk_gains_dequant;
use crate::src::opus_1_2_1::silk::gain_quant::silk_gains_quant;
use crate::src::opus_1_2_1::silk::LP_variable_cutoff::silk_LP_variable_cutoff;
use crate::src::opus_1_2_1::silk::VAD::silk_VAD_GetSA_Q8_c;
#[no_mangle]

pub unsafe extern "C" fn silk_encode_do_VAD_FLP(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
)
/* I/O  Encoder state FLP                           */
{
    /* ***************************/
    /* Voice Activity Detection */
    /* ***************************/
    crate::src::opus_1_2_1::silk::VAD::silk_VAD_GetSA_Q8_c(
        &mut (*psEnc).sCmn as *mut _ as *mut crate::structs_h::silk_encoder_state,
        (*psEnc)
            .sCmn
            .inputBuf
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize) as *const crate::opus_types_h::opus_int16,
    );
    /* *************************************************/
    /* Convert speech activity into VAD and DTX flags */
    /* *************************************************/
    if (*psEnc).sCmn.speech_activity_Q8
        < ((0.05f32 * ((1 as libc::c_int as libc::c_longlong) << 8 as libc::c_int) as libc::c_float)
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    {
        (*psEnc).sCmn.indices.signalType = 0 as libc::c_int as libc::c_schar;
        (*psEnc).sCmn.noSpeechCounter += 1;
        if (*psEnc).sCmn.noSpeechCounter < 10 as libc::c_int {
            (*psEnc).sCmn.inDTX = 0 as libc::c_int
        } else if (*psEnc).sCmn.noSpeechCounter > 20 as libc::c_int + 10 as libc::c_int {
            (*psEnc).sCmn.noSpeechCounter = 10 as libc::c_int;
            (*psEnc).sCmn.inDTX = 0 as libc::c_int
        }
        (*psEnc).sCmn.VAD_flags[(*psEnc).sCmn.nFramesEncoded as usize] =
            0 as libc::c_int as libc::c_schar
    } else {
        (*psEnc).sCmn.noSpeechCounter = 0 as libc::c_int;
        (*psEnc).sCmn.inDTX = 0 as libc::c_int;
        (*psEnc).sCmn.indices.signalType = 1 as libc::c_int as libc::c_schar;
        (*psEnc).sCmn.VAD_flags[(*psEnc).sCmn.nFramesEncoded as usize] =
            1 as libc::c_int as libc::c_schar
    };
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
/* ********************/
/* Encoder Functions */
/* ********************/
/* High-pass filter with cutoff frequency adaptation based on pitch lag statistics */
/* I/O  Encoder states                              */
/* Encoder main function */
/* I/O  Encoder state FLP                           */
/* Encoder main function */
/* ***************/
/* Encode frame */
/* ***************/
#[no_mangle]

pub unsafe extern "C" fn silk_encode_frame_FLP(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
    mut pnBytesOut: *mut crate::opus_types_h::opus_int32,
    mut psRangeEnc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut condCoding: libc::c_int,
    mut maxBits: libc::c_int,
    mut useCBR: libc::c_int,
) -> libc::c_int
/* I    Flag to force constant-bitrate operation    */ {
    let mut sEncCtrl: crate::structs_FLP_h::silk_encoder_control_FLP =
        crate::structs_FLP_h::silk_encoder_control_FLP {
            Gains: [0.; 4],
            PredCoef: [[0.; 16]; 2],
            LTPCoef: [0.; 20],
            LTP_scale: 0.,
            pitchL: [0; 4],
            AR: [0.; 96],
            LF_MA_shp: [0.; 4],
            LF_AR_shp: [0.; 4],
            Tilt: [0.; 4],
            HarmShapeGain: [0.; 4],
            Lambda: 0.,
            input_quality: 0.,
            coding_quality: 0.,
            predGain: 0.,
            LTPredCodGain: 0.,
            ResNrg: [0.; 4],
            GainsUnq_Q16: [0; 4],
            lastGainIndexPrev: 0,
        };
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut maxIter: libc::c_int = 0;
    let mut found_upper: libc::c_int = 0;
    let mut found_lower: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut x_frame: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut res_pitch_frame: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut res_pitch: [libc::c_float; 672] = [0.; 672];
    let mut sRangeEnc_copy: crate::src::opus_1_2_1::celt::entcode::ec_enc =
        crate::src::opus_1_2_1::celt::entcode::ec_enc {
            buf: 0 as *mut libc::c_uchar,
            storage: 0,
            end_offs: 0,
            end_window: 0,
            nend_bits: 0,
            nbits_total: 0,
            offs: 0,
            rng: 0,
            val: 0,
            ext: 0,
            rem: 0,
            error: 0,
        };
    let mut sRangeEnc_copy2: crate::src::opus_1_2_1::celt::entcode::ec_enc =
        crate::src::opus_1_2_1::celt::entcode::ec_enc {
            buf: 0 as *mut libc::c_uchar,
            storage: 0,
            end_offs: 0,
            end_window: 0,
            nend_bits: 0,
            nbits_total: 0,
            offs: 0,
            rng: 0,
            val: 0,
            ext: 0,
            rem: 0,
            error: 0,
        };
    let mut sNSQ_copy: crate::structs_h::silk_nsq_state = crate::structs_h::silk_nsq_state {
        xq: [0; 640],
        sLTP_shp_Q14: [0; 640],
        sLPC_Q14: [0; 96],
        sAR2_Q14: [0; 24],
        sLF_AR_shp_Q14: 0,
        sDiff_shp_Q14: 0,
        lagPrev: 0,
        sLTP_buf_idx: 0,
        sLTP_shp_buf_idx: 0,
        rand_seed: 0,
        prev_gain_Q16: 0,
        rewhite_flag: 0,
    };
    let mut sNSQ_copy2: crate::structs_h::silk_nsq_state = crate::structs_h::silk_nsq_state {
        xq: [0; 640],
        sLTP_shp_Q14: [0; 640],
        sLPC_Q14: [0; 96],
        sAR2_Q14: [0; 24],
        sLF_AR_shp_Q14: 0,
        sDiff_shp_Q14: 0,
        lagPrev: 0,
        sLTP_buf_idx: 0,
        sLTP_shp_buf_idx: 0,
        rand_seed: 0,
        prev_gain_Q16: 0,
        rewhite_flag: 0,
    };
    let mut seed_copy: crate::opus_types_h::opus_int32 = 0;
    let mut nBits: crate::opus_types_h::opus_int32 = 0;
    let mut nBits_lower: crate::opus_types_h::opus_int32 = 0;
    let mut nBits_upper: crate::opus_types_h::opus_int32 = 0;
    let mut gainMult_lower: crate::opus_types_h::opus_int32 = 0;
    let mut gainMult_upper: crate::opus_types_h::opus_int32 = 0;
    let mut gainsID: crate::opus_types_h::opus_int32 = 0;
    let mut gainsID_lower: crate::opus_types_h::opus_int32 = 0;
    let mut gainsID_upper: crate::opus_types_h::opus_int32 = 0;
    let mut gainMult_Q8: crate::opus_types_h::opus_int16 = 0;
    let mut ec_prevLagIndex_copy: crate::opus_types_h::opus_int16 = 0;
    let mut ec_prevSignalType_copy: libc::c_int = 0;
    let mut LastGainIndex_copy2: libc::c_schar = 0;
    let mut pGains_Q16: [crate::opus_types_h::opus_int32; 4] = [0; 4];
    let mut ec_buf_copy: [libc::c_uchar; 1275] = [0; 1275];
    let mut gain_lock: [libc::c_int; 4] = [0 as libc::c_int, 0, 0, 0];
    let mut best_gain_mult: [crate::opus_types_h::opus_int16; 4] = [0; 4];
    let mut best_sum: [libc::c_int; 4] = [0; 4];
    /* This is totally unnecessary but many compilers (including gcc) are too dumb to realise it */
    gainMult_upper = 0 as libc::c_int;
    gainMult_lower = gainMult_upper;
    nBits_upper = gainMult_lower;
    nBits_lower = nBits_upper;
    LastGainIndex_copy2 = nBits_lower as libc::c_schar;
    let fresh0 = (*psEnc).sCmn.frameCounter;
    (*psEnc).sCmn.frameCounter = (*psEnc).sCmn.frameCounter + 1;
    (*psEnc).sCmn.indices.Seed = (fresh0 & 3 as libc::c_int) as libc::c_schar;
    /* *************************************************************/
    /* Set up Input Pointers, and insert frame in input buffer    */
    /* *************************************************************/
    /* pointers aligned with start of frame to encode */
    x_frame = (*psEnc)
        .x_buf
        .as_mut_ptr()
        .offset((*psEnc).sCmn.ltp_mem_length as isize); /* start of frame to encode */
    res_pitch_frame = res_pitch
        .as_mut_ptr()
        .offset((*psEnc).sCmn.ltp_mem_length as isize); /* start of pitch LPC residual frame */
    /* **************************************/
    /* Ensure smooth bandwidth transitions */
    /* **************************************/
    crate::src::opus_1_2_1::silk::LP_variable_cutoff::silk_LP_variable_cutoff(
        &mut (*psEnc).sCmn.sLP as *mut _ as *mut crate::structs_h::silk_LP_state,
        (*psEnc)
            .sCmn
            .inputBuf
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        (*psEnc).sCmn.frame_length,
    );
    /* ******************************************/
    /* Copy new frame to front of input buffer */
    /* ******************************************/
    silk_short2float_array(
        x_frame.offset((5 as libc::c_int * (*psEnc).sCmn.fs_kHz) as isize),
        (*psEnc)
            .sCmn
            .inputBuf
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        (*psEnc).sCmn.frame_length,
    );
    /* Add tiny signal to avoid high CPU load from denormalized floating point numbers */
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *x_frame.offset(
            (5 as libc::c_int * (*psEnc).sCmn.fs_kHz
                + i * ((*psEnc).sCmn.frame_length >> 3 as libc::c_int)) as isize,
        ) += (1 as libc::c_int - (i & 2 as libc::c_int)) as libc::c_float * 1e-6f32;
        i += 1
    }
    if (*psEnc).sCmn.prefillFlag == 0 {
        /* ****************************************/
        /* Find pitch lags, initial LPC analysis */
        /* ****************************************/
        crate::src::opus_1_2_1::silk::float::find_pitch_lags_FLP::silk_find_pitch_lags_FLP(
            psEnc as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
            &mut sEncCtrl as *mut _ as *mut crate::structs_FLP_h::silk_encoder_control_FLP,
            res_pitch.as_mut_ptr(),
            x_frame as *const libc::c_float,
            (*psEnc).sCmn.arch,
        );
        /* ***********************/
        /* Noise shape analysis */
        /* ***********************/
        crate::src::opus_1_2_1::silk::float::noise_shape_analysis_FLP::silk_noise_shape_analysis_FLP(psEnc as *mut crate::structs_FLP_h::silk_encoder_state_FLP,  &mut sEncCtrl as *mut _ as *mut crate::structs_FLP_h::silk_encoder_control_FLP, res_pitch_frame,
                                      x_frame);
        /* **************************************************/
        /* Find linear prediction coefficients (LPC + LTP) */
        /* **************************************************/
        crate::src::opus_1_2_1::silk::float::find_pred_coefs_FLP::silk_find_pred_coefs_FLP(
            psEnc as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
            &mut sEncCtrl as *mut _ as *mut crate::structs_FLP_h::silk_encoder_control_FLP,
            res_pitch_frame as *const libc::c_float,
            x_frame as *const libc::c_float,
            condCoding,
        );
        /* ***************************************/
        /* Process gains                        */
        /* ***************************************/
        crate::src::opus_1_2_1::silk::float::process_gains_FLP::silk_process_gains_FLP(
            psEnc as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
            &mut sEncCtrl as *mut _ as *mut crate::structs_FLP_h::silk_encoder_control_FLP,
            condCoding,
        );
        /* ***************************************/
        /* Low Bitrate Redundant Encoding       */
        /* ***************************************/
        silk_LBRR_encode_FLP(
            psEnc,
            &mut sEncCtrl,
            x_frame as *const libc::c_float,
            condCoding,
        );
        /* Loop over quantizer and entroy coding to control bitrate */
        maxIter = 6 as libc::c_int;
        gainMult_Q8 = ((1 as libc::c_int as libc::c_longlong
            * ((1 as libc::c_int as libc::c_longlong) << 8 as libc::c_int))
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_int16;
        found_lower = 0 as libc::c_int;
        found_upper = 0 as libc::c_int;
        gainsID = crate::src::opus_1_2_1::silk::gain_quant::silk_gains_ID(
            (*psEnc).sCmn.indices.GainsIndices.as_mut_ptr() as *const libc::c_schar,
            (*psEnc).sCmn.nb_subfr,
        );
        gainsID_lower = -(1 as libc::c_int);
        gainsID_upper = -(1 as libc::c_int);
        /* Copy part of the input state */
        crate::stdlib::memcpy(
            &mut sRangeEnc_copy as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                as *mut libc::c_void,
            psRangeEnc as *const libc::c_void,
            ::std::mem::size_of::<crate::src::opus_1_2_1::celt::entcode::ec_enc>() as libc::c_ulong,
        );
        crate::stdlib::memcpy(
            &mut sNSQ_copy as *mut crate::structs_h::silk_nsq_state as *mut libc::c_void,
            &mut (*psEnc).sCmn.sNSQ as *mut crate::structs_h::silk_nsq_state as *const libc::c_void,
            ::std::mem::size_of::<crate::structs_h::silk_nsq_state>() as libc::c_ulong,
        );
        seed_copy = (*psEnc).sCmn.indices.Seed as crate::opus_types_h::opus_int32;
        ec_prevLagIndex_copy = (*psEnc).sCmn.ec_prevLagIndex;
        ec_prevSignalType_copy = (*psEnc).sCmn.ec_prevSignalType;
        iter = 0 as libc::c_int;
        loop {
            if gainsID == gainsID_lower {
                nBits = nBits_lower
            } else if gainsID == gainsID_upper {
                nBits = nBits_upper
            } else {
                /* Restore part of the input state */
                if iter > 0 as libc::c_int {
                    crate::stdlib::memcpy(
                        psRangeEnc as *mut libc::c_void,
                        &mut sRangeEnc_copy as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::src::opus_1_2_1::celt::entcode::ec_enc>()
                            as libc::c_ulong,
                    );
                    crate::stdlib::memcpy(
                        &mut (*psEnc).sCmn.sNSQ as *mut crate::structs_h::silk_nsq_state
                            as *mut libc::c_void,
                        &mut sNSQ_copy as *mut crate::structs_h::silk_nsq_state
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::structs_h::silk_nsq_state>() as libc::c_ulong,
                    );
                    (*psEnc).sCmn.indices.Seed = seed_copy as libc::c_schar;
                    (*psEnc).sCmn.ec_prevLagIndex = ec_prevLagIndex_copy;
                    (*psEnc).sCmn.ec_prevSignalType = ec_prevSignalType_copy
                }
                /* ****************************************/
                /* Noise shaping quantization            */
                /* ****************************************/
                crate::src::opus_1_2_1::silk::float::wrappers_FLP::silk_NSQ_wrapper_FLP(
                    psEnc as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
                    &mut sEncCtrl as *mut _ as *mut crate::structs_FLP_h::silk_encoder_control_FLP,
                    &mut (*psEnc).sCmn.indices as *mut _ as *mut crate::structs_h::SideInfoIndices,
                    &mut (*psEnc).sCmn.sNSQ as *mut _ as *mut crate::structs_h::silk_nsq_state,
                    (*psEnc).sCmn.pulses.as_mut_ptr(),
                    x_frame as *const libc::c_float,
                );
                if iter == maxIter && found_lower == 0 {
                    crate::stdlib::memcpy(
                        &mut sRangeEnc_copy2 as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                            as *mut libc::c_void,
                        psRangeEnc as *const libc::c_void,
                        ::std::mem::size_of::<crate::src::opus_1_2_1::celt::entcode::ec_enc>()
                            as libc::c_ulong,
                    );
                }
                /* ***************************************/
                /* Encode Parameters                    */
                /* ***************************************/
                crate::src::opus_1_2_1::silk::encode_indices::silk_encode_indices(
                    &mut (*psEnc).sCmn as *mut _ as *mut crate::structs_h::silk_encoder_state,
                    psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    (*psEnc).sCmn.nFramesEncoded,
                    0 as libc::c_int,
                    condCoding,
                );
                /* ***************************************/
                /* Encode Excitation Signal             */
                /* ***************************************/
                crate::src::opus_1_2_1::silk::encode_pulses::silk_encode_pulses(
                    psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    (*psEnc).sCmn.indices.signalType as libc::c_int,
                    (*psEnc).sCmn.indices.quantOffsetType as libc::c_int,
                    (*psEnc).sCmn.pulses.as_mut_ptr(),
                    (*psEnc).sCmn.frame_length,
                );
                nBits = ec_tell(psRangeEnc);
                /* If we still bust after the last iteration, do some damage control. */
                if iter == maxIter && found_lower == 0 && nBits > maxBits {
                    crate::stdlib::memcpy(
                        psRangeEnc as *mut libc::c_void,
                        &mut sRangeEnc_copy2 as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::src::opus_1_2_1::celt::entcode::ec_enc>()
                            as libc::c_ulong,
                    );
                    /* Keep gains the same as the last frame. */
                    (*psEnc).sShape.LastGainIndex = sEncCtrl.lastGainIndexPrev;
                    i = 0 as libc::c_int;
                    while i < (*psEnc).sCmn.nb_subfr {
                        (*psEnc).sCmn.indices.GainsIndices[i as usize] =
                            4 as libc::c_int as libc::c_schar;
                        i += 1
                    }
                    if condCoding != 2 as libc::c_int {
                        (*psEnc).sCmn.indices.GainsIndices[0 as libc::c_int as usize] =
                            sEncCtrl.lastGainIndexPrev
                    }
                    (*psEnc).sCmn.ec_prevLagIndex = ec_prevLagIndex_copy;
                    (*psEnc).sCmn.ec_prevSignalType = ec_prevSignalType_copy;
                    /* Clear all pulses. */
                    i = 0 as libc::c_int;
                    while i < (*psEnc).sCmn.frame_length {
                        (*psEnc).sCmn.pulses[i as usize] = 0 as libc::c_int as libc::c_schar;
                        i += 1
                    }
                    crate::src::opus_1_2_1::silk::encode_indices::silk_encode_indices(
                        &mut (*psEnc).sCmn as *mut _ as *mut crate::structs_h::silk_encoder_state,
                        psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        (*psEnc).sCmn.nFramesEncoded,
                        0 as libc::c_int,
                        condCoding,
                    );
                    crate::src::opus_1_2_1::silk::encode_pulses::silk_encode_pulses(
                        psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        (*psEnc).sCmn.indices.signalType as libc::c_int,
                        (*psEnc).sCmn.indices.quantOffsetType as libc::c_int,
                        (*psEnc).sCmn.pulses.as_mut_ptr(),
                        (*psEnc).sCmn.frame_length,
                    );
                    nBits = ec_tell(psRangeEnc)
                }
                if useCBR == 0 as libc::c_int && iter == 0 as libc::c_int && nBits <= maxBits {
                    break;
                }
            }
            if iter == maxIter {
                if found_lower != 0 && (gainsID == gainsID_lower || nBits > maxBits) {
                    /* Restore output state from earlier iteration that did meet the bitrate budget */
                    crate::stdlib::memcpy(
                        psRangeEnc as *mut libc::c_void,
                        &mut sRangeEnc_copy2 as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::src::opus_1_2_1::celt::entcode::ec_enc>()
                            as libc::c_ulong,
                    );
                    crate::stdlib::memcpy(
                        (*psRangeEnc).buf as *mut libc::c_void,
                        ec_buf_copy.as_mut_ptr() as *const libc::c_void,
                        sRangeEnc_copy2.offs as libc::c_ulong,
                    );
                    crate::stdlib::memcpy(
                        &mut (*psEnc).sCmn.sNSQ as *mut crate::structs_h::silk_nsq_state
                            as *mut libc::c_void,
                        &mut sNSQ_copy2 as *mut crate::structs_h::silk_nsq_state
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::structs_h::silk_nsq_state>() as libc::c_ulong,
                    );
                    (*psEnc).sShape.LastGainIndex = LastGainIndex_copy2
                }
                break;
            } else {
                if nBits > maxBits {
                    if found_lower == 0 as libc::c_int && iter >= 2 as libc::c_int {
                        /* Adjust the quantizer's rate/distortion tradeoff and discard previous "upper" results */
                        sEncCtrl.Lambda = if sEncCtrl.Lambda * 1.5f32 > 1.5f32 {
                            (sEncCtrl.Lambda) * 1.5f32
                        } else {
                            1.5f32
                        };
                        /* Reducing dithering can help us hit the target. */
                        (*psEnc).sCmn.indices.quantOffsetType = 0 as libc::c_int as libc::c_schar;
                        found_upper = 0 as libc::c_int;
                        gainsID_upper = -(1 as libc::c_int)
                    } else {
                        found_upper = 1 as libc::c_int;
                        nBits_upper = nBits;
                        gainMult_upper = gainMult_Q8 as crate::opus_types_h::opus_int32;
                        gainsID_upper = gainsID
                    }
                } else {
                    if !(nBits < maxBits - 5 as libc::c_int) {
                        break;
                    }
                    found_lower = 1 as libc::c_int;
                    nBits_lower = nBits;
                    gainMult_lower = gainMult_Q8 as crate::opus_types_h::opus_int32;
                    if gainsID != gainsID_lower {
                        gainsID_lower = gainsID;
                        /* Copy part of the output state */
                        crate::stdlib::memcpy(
                            &mut sRangeEnc_copy2
                                as *mut crate::src::opus_1_2_1::celt::entcode::ec_enc
                                as *mut libc::c_void,
                            psRangeEnc as *const libc::c_void,
                            ::std::mem::size_of::<crate::src::opus_1_2_1::celt::entcode::ec_enc>()
                                as libc::c_ulong,
                        );
                        crate::stdlib::memcpy(
                            ec_buf_copy.as_mut_ptr() as *mut libc::c_void,
                            (*psRangeEnc).buf as *const libc::c_void,
                            (*psRangeEnc).offs as libc::c_ulong,
                        );
                        crate::stdlib::memcpy(
                            &mut sNSQ_copy2 as *mut crate::structs_h::silk_nsq_state
                                as *mut libc::c_void,
                            &mut (*psEnc).sCmn.sNSQ as *mut crate::structs_h::silk_nsq_state
                                as *const libc::c_void,
                            ::std::mem::size_of::<crate::structs_h::silk_nsq_state>()
                                as libc::c_ulong,
                        );
                        LastGainIndex_copy2 = (*psEnc).sShape.LastGainIndex
                    }
                }
                if found_lower == 0 && nBits > maxBits {
                    let mut j: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i < (*psEnc).sCmn.nb_subfr {
                        let mut sum: libc::c_int = 0 as libc::c_int;
                        j = i * (*psEnc).sCmn.subfr_length;
                        while j < (i + 1 as libc::c_int) * (*psEnc).sCmn.subfr_length {
                            sum += ::libc::abs((*psEnc).sCmn.pulses[j as usize] as libc::c_int);
                            j += 1
                        }
                        if iter == 0 as libc::c_int
                            || sum < best_sum[i as usize] && gain_lock[i as usize] == 0
                        {
                            best_sum[i as usize] = sum;
                            best_gain_mult[i as usize] = gainMult_Q8
                        } else {
                            gain_lock[i as usize] = 1 as libc::c_int
                        }
                        i += 1
                    }
                }
                if found_lower & found_upper == 0 as libc::c_int {
                    /* Adjust gain according to high-rate rate/distortion curve */
                    if nBits > maxBits {
                        if (gainMult_Q8 as libc::c_int) < 16384 as libc::c_int {
                            gainMult_Q8 = (gainMult_Q8 as libc::c_int * 2 as libc::c_int)
                                as crate::opus_types_h::opus_int16
                        } else {
                            gainMult_Q8 = 32767 as libc::c_int as crate::opus_types_h::opus_int16
                        }
                    } else {
                        let mut gain_factor_Q16: crate::opus_types_h::opus_int32 = 0;
                        gain_factor_Q16 = crate::src::opus_1_2_1::silk::log2lin::silk_log2lin(
                            (((nBits - maxBits) as crate::opus_types_h::opus_uint32)
                                << 7 as libc::c_int)
                                as crate::opus_types_h::opus_int32
                                / (*psEnc).sCmn.frame_length
                                + ((16 as libc::c_int as libc::c_longlong
                                    * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int))
                                    as libc::c_double
                                    + 0.5f64)
                                    as crate::opus_types_h::opus_int32,
                        );
                        gainMult_Q8 = (gain_factor_Q16 as libc::c_longlong
                            * gainMult_Q8 as libc::c_longlong
                            >> 16 as libc::c_int)
                            as crate::opus_types_h::opus_int32
                            as crate::opus_types_h::opus_int16
                    }
                } else {
                    /* Adjust gain by interpolating */
                    gainMult_Q8 = (gainMult_lower
                        + (gainMult_upper - gainMult_lower) * (maxBits - nBits_lower)
                            / (nBits_upper - nBits_lower))
                        as crate::opus_types_h::opus_int16;
                    /* New gain multplier must be between 25% and 75% of old range (note that gainMult_upper < gainMult_lower) */
                    if gainMult_Q8 as libc::c_int
                        > gainMult_lower + (gainMult_upper - gainMult_lower >> 2 as libc::c_int)
                    {
                        gainMult_Q8 = (gainMult_lower
                            + (gainMult_upper - gainMult_lower >> 2 as libc::c_int))
                            as crate::opus_types_h::opus_int16
                    } else if (gainMult_Q8 as libc::c_int)
                        < gainMult_upper - (gainMult_upper - gainMult_lower >> 2 as libc::c_int)
                    {
                        gainMult_Q8 = (gainMult_upper
                            - (gainMult_upper - gainMult_lower >> 2 as libc::c_int))
                            as crate::opus_types_h::opus_int16
                    }
                }
                i = 0 as libc::c_int;
                while i < (*psEnc).sCmn.nb_subfr {
                    let mut tmp: crate::opus_types_h::opus_int16 = 0;
                    if gain_lock[i as usize] != 0 {
                        tmp = best_gain_mult[i as usize]
                    } else {
                        tmp = gainMult_Q8
                    }
                    pGains_Q16[i as usize] =
                        (((if 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                            >> 8 as libc::c_int
                            > 0x7fffffff as libc::c_int >> 8 as libc::c_int
                        {
                            (if (sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_longlong
                                * tmp as libc::c_longlong
                                >> 16 as libc::c_int)
                                as crate::opus_types_h::opus_int32
                                > 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                                    >> 8 as libc::c_int
                            {
                                (0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32)
                                    >> 8 as libc::c_int
                            } else {
                                (if ((sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_longlong
                                    * tmp as libc::c_longlong
                                    >> 16 as libc::c_int)
                                    as crate::opus_types_h::opus_int32)
                                    < 0x7fffffff as libc::c_int >> 8 as libc::c_int
                                {
                                    (0x7fffffff as libc::c_int) >> 8 as libc::c_int
                                } else {
                                    (sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_longlong
                                        * tmp as libc::c_longlong
                                        >> 16 as libc::c_int)
                                        as crate::opus_types_h::opus_int32
                                })
                            })
                        } else {
                            (if (sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_longlong
                                * tmp as libc::c_longlong
                                >> 16 as libc::c_int)
                                as crate::opus_types_h::opus_int32
                                > 0x7fffffff as libc::c_int >> 8 as libc::c_int
                            {
                                (0x7fffffff as libc::c_int) >> 8 as libc::c_int
                            } else {
                                (if ((sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_longlong
                                    * tmp as libc::c_longlong
                                    >> 16 as libc::c_int)
                                    as crate::opus_types_h::opus_int32)
                                    < 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                                        >> 8 as libc::c_int
                                {
                                    (0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32)
                                        >> 8 as libc::c_int
                                } else {
                                    (sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_longlong
                                        * tmp as libc::c_longlong
                                        >> 16 as libc::c_int)
                                        as crate::opus_types_h::opus_int32
                                })
                            })
                        }) as crate::opus_types_h::opus_uint32)
                            << 8 as libc::c_int)
                            as crate::opus_types_h::opus_int32;
                    i += 1
                }
                /* Quantize gains */
                (*psEnc).sShape.LastGainIndex = sEncCtrl.lastGainIndexPrev;
                crate::src::opus_1_2_1::silk::gain_quant::silk_gains_quant(
                    (*psEnc).sCmn.indices.GainsIndices.as_mut_ptr(),
                    pGains_Q16.as_mut_ptr(),
                    &mut (*psEnc).sShape.LastGainIndex,
                    (condCoding == 2 as libc::c_int) as libc::c_int,
                    (*psEnc).sCmn.nb_subfr,
                );
                /* Unique identifier of gains vector */
                gainsID = crate::src::opus_1_2_1::silk::gain_quant::silk_gains_ID(
                    (*psEnc).sCmn.indices.GainsIndices.as_mut_ptr() as *const libc::c_schar,
                    (*psEnc).sCmn.nb_subfr,
                );
                /* Overwrite unquantized gains with quantized gains and convert back to Q0 from Q16 */
                i = 0 as libc::c_int;
                while i < (*psEnc).sCmn.nb_subfr {
                    sEncCtrl.Gains[i as usize] =
                        pGains_Q16[i as usize] as libc::c_float / 65536.0f32;
                    i += 1
                }
                iter += 1
            }
        }
    }
    /* Update input buffer */
    crate::stdlib::memmove(
        (*psEnc).x_buf.as_mut_ptr() as *mut libc::c_void,
        &mut *(*psEnc)
            .x_buf
            .as_mut_ptr()
            .offset((*psEnc).sCmn.frame_length as isize) as *mut libc::c_float
            as *const libc::c_void,
        (((*psEnc).sCmn.ltp_mem_length + 5 as libc::c_int * (*psEnc).sCmn.fs_kHz) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    /* Exit without entropy coding */
    if (*psEnc).sCmn.prefillFlag != 0 {
        /* No payload */
        *pnBytesOut = 0 as libc::c_int;
        return ret;
    }
    /* Parameters needed for next frame */
    (*psEnc).sCmn.prevLag = sEncCtrl.pitchL[((*psEnc).sCmn.nb_subfr - 1 as libc::c_int) as usize];
    (*psEnc).sCmn.prevSignalType = (*psEnc).sCmn.indices.signalType;
    /* ***************************************/
    /* Finalize payload                     */
    /* ***************************************/
    (*psEnc).sCmn.first_frame_after_reset = 0 as libc::c_int;
    /* Payload size */
    *pnBytesOut = ec_tell(psRangeEnc) + 7 as libc::c_int >> 3 as libc::c_int;
    return ret;
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
/* Low Bitrate Redundancy (LBRR) encoding. Reuse all parameters but encode with lower bitrate */
/* Low-Bitrate Redundancy (LBRR) encoding. Reuse all parameters but encode excitation at lower bitrate  */
#[inline]

unsafe extern "C" fn silk_LBRR_encode_FLP(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
    mut psEncCtrl: *mut crate::structs_FLP_h::silk_encoder_control_FLP,
    mut xfw: *const libc::c_float,
    mut condCoding: libc::c_int,
)
/* I    The type of conditional coding used so far for this frame */
{
    let mut k: libc::c_int = 0;
    let mut Gains_Q16: [crate::opus_types_h::opus_int32; 4] = [0; 4];
    let mut TempGains: [libc::c_float; 4] = [0.; 4];
    let mut psIndices_LBRR: *mut crate::structs_h::SideInfoIndices = &mut *(*psEnc)
        .sCmn
        .indices_LBRR
        .as_mut_ptr()
        .offset((*psEnc).sCmn.nFramesEncoded as isize)
        as *mut crate::structs_h::SideInfoIndices;
    let mut sNSQ_LBRR: crate::structs_h::silk_nsq_state = crate::structs_h::silk_nsq_state {
        xq: [0; 640],
        sLTP_shp_Q14: [0; 640],
        sLPC_Q14: [0; 96],
        sAR2_Q14: [0; 24],
        sLF_AR_shp_Q14: 0,
        sDiff_shp_Q14: 0,
        lagPrev: 0,
        sLTP_buf_idx: 0,
        sLTP_shp_buf_idx: 0,
        rand_seed: 0,
        prev_gain_Q16: 0,
        rewhite_flag: 0,
    };
    /* ******************************************/
    /* Control use of inband LBRR              */
    /* ******************************************/
    if (*psEnc).sCmn.LBRR_enabled != 0
        && (*psEnc).sCmn.speech_activity_Q8
            > ((0.3f32
                * ((1 as libc::c_int as libc::c_longlong) << 8 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
    {
        (*psEnc).sCmn.LBRR_flags[(*psEnc).sCmn.nFramesEncoded as usize] = 1 as libc::c_int;
        /* Copy noise shaping quantizer state and quantization indices from regular encoding */
        crate::stdlib::memcpy(
            &mut sNSQ_LBRR as *mut crate::structs_h::silk_nsq_state as *mut libc::c_void,
            &mut (*psEnc).sCmn.sNSQ as *mut crate::structs_h::silk_nsq_state as *const libc::c_void,
            ::std::mem::size_of::<crate::structs_h::silk_nsq_state>() as libc::c_ulong,
        );
        crate::stdlib::memcpy(
            psIndices_LBRR as *mut libc::c_void,
            &mut (*psEnc).sCmn.indices as *mut crate::structs_h::SideInfoIndices
                as *const libc::c_void,
            ::std::mem::size_of::<crate::structs_h::SideInfoIndices>() as libc::c_ulong,
        );
        /* Save original gains */
        crate::stdlib::memcpy(
            TempGains.as_mut_ptr() as *mut libc::c_void,
            (*psEncCtrl).Gains.as_mut_ptr() as *const libc::c_void,
            ((*psEnc).sCmn.nb_subfr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        if (*psEnc).sCmn.nFramesEncoded == 0 as libc::c_int
            || (*psEnc).sCmn.LBRR_flags[((*psEnc).sCmn.nFramesEncoded - 1 as libc::c_int) as usize]
                == 0 as libc::c_int
        {
            /* First frame in packet or previous frame not LBRR coded */
            (*psEnc).sCmn.LBRRprevLastGainIndex = (*psEnc).sShape.LastGainIndex;
            /* Increase Gains to get target LBRR rate */
            (*psIndices_LBRR).GainsIndices[0 as libc::c_int as usize] =
                ((*psIndices_LBRR).GainsIndices[0 as libc::c_int as usize] as libc::c_int
                    + (*psEnc).sCmn.LBRR_GainIncreases) as libc::c_schar;
            (*psIndices_LBRR).GainsIndices[0 as libc::c_int as usize] = silk_min_int(
                (*psIndices_LBRR).GainsIndices[0 as libc::c_int as usize] as libc::c_int,
                64 as libc::c_int - 1 as libc::c_int,
            )
                as libc::c_schar
        }
        /* Decode to get gains in sync with decoder */
        crate::src::opus_1_2_1::silk::gain_quant::silk_gains_dequant(
            Gains_Q16.as_mut_ptr(),
            (*psIndices_LBRR).GainsIndices.as_mut_ptr() as *const libc::c_schar,
            &mut (*psEnc).sCmn.LBRRprevLastGainIndex,
            (condCoding == 2 as libc::c_int) as libc::c_int,
            (*psEnc).sCmn.nb_subfr,
        );
        /* Overwrite unquantized gains with quantized gains and convert back to Q0 from Q16 */
        k = 0 as libc::c_int;
        while k < (*psEnc).sCmn.nb_subfr {
            (*psEncCtrl).Gains[k as usize] =
                Gains_Q16[k as usize] as libc::c_float * (1.0f32 / 65536.0f32);
            k += 1
        }
        /* ****************************************/
        /* Noise shaping quantization            */
        /* ****************************************/
        crate::src::opus_1_2_1::silk::float::wrappers_FLP::silk_NSQ_wrapper_FLP(
            psEnc as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
            psEncCtrl as *mut crate::structs_FLP_h::silk_encoder_control_FLP,
            psIndices_LBRR as *mut crate::structs_h::SideInfoIndices,
            &mut sNSQ_LBRR as *mut _ as *mut crate::structs_h::silk_nsq_state,
            (*psEnc).sCmn.pulses_LBRR[(*psEnc).sCmn.nFramesEncoded as usize].as_mut_ptr(),
            xfw,
        );
        /* Restore original gains */
        crate::stdlib::memcpy(
            (*psEncCtrl).Gains.as_mut_ptr() as *mut libc::c_void,
            TempGains.as_mut_ptr() as *const libc::c_void,
            ((*psEnc).sCmn.nb_subfr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
    };
}
