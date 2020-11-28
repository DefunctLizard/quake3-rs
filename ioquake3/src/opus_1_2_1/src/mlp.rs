// =============== BEGIN mlp_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MLP {
    pub layers: libc::c_int,
    pub topo: *const libc::c_int,
    pub weights: *const libc::c_float,
}
use ::libc;

pub mod arch_h {

    /* This code should reliably detect NaN/inf even when -ffast-math is used.
    Assumes IEEE 754 format. */
    #[inline]

    pub unsafe extern "C" fn celt_isnan(mut x: libc::c_float) -> libc::c_int {
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        return (in_0.i >> 23 as libc::c_int & 0xff as libc::c_int as libc::c_uint
            == 0xff as libc::c_int as libc::c_uint
            && in_0.i & 0x7fffff as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint)
            as libc::c_int;
    }

    /* ARCH_H */
    /* !FIXED_POINT */
    /* This appears to be the same speed as C99's fabsf() but it's more portable. */
}

pub mod tansig_table_h {
    /* This file is auto-generated by gen_tables */

    pub static mut tansig_table: [libc::c_float; 201] = [
        0.000000f32,
        0.039979f32,
        0.079830f32,
        0.119427f32,
        0.158649f32,
        0.197375f32,
        0.235496f32,
        0.272905f32,
        0.309507f32,
        0.345214f32,
        0.379949f32,
        0.413644f32,
        0.446244f32,
        0.477700f32,
        0.507977f32,
        0.537050f32,
        0.564900f32,
        0.591519f32,
        0.616909f32,
        0.641077f32,
        0.664037f32,
        0.685809f32,
        0.706419f32,
        0.725897f32,
        0.744277f32,
        0.761594f32,
        0.777888f32,
        0.793199f32,
        0.807569f32,
        0.821040f32,
        0.833655f32,
        0.845456f32,
        0.856485f32,
        0.866784f32,
        0.876393f32,
        0.885352f32,
        0.893698f32,
        0.901468f32,
        0.908698f32,
        0.915420f32,
        0.921669f32,
        0.927473f32,
        0.932862f32,
        0.937863f32,
        0.942503f32,
        0.946806f32,
        0.950795f32,
        0.954492f32,
        0.957917f32,
        0.961090f32,
        0.964028f32,
        0.966747f32,
        0.969265f32,
        0.971594f32,
        0.973749f32,
        0.975743f32,
        0.977587f32,
        0.979293f32,
        0.980869f32,
        0.982327f32,
        0.983675f32,
        0.984921f32,
        0.986072f32,
        0.987136f32,
        0.988119f32,
        0.989027f32,
        0.989867f32,
        0.990642f32,
        0.991359f32,
        0.992020f32,
        0.992631f32,
        0.993196f32,
        0.993718f32,
        0.994199f32,
        0.994644f32,
        0.995055f32,
        0.995434f32,
        0.995784f32,
        0.996108f32,
        0.996407f32,
        0.996682f32,
        0.996937f32,
        0.997172f32,
        0.997389f32,
        0.997590f32,
        0.997775f32,
        0.997946f32,
        0.998104f32,
        0.998249f32,
        0.998384f32,
        0.998508f32,
        0.998623f32,
        0.998728f32,
        0.998826f32,
        0.998916f32,
        0.999000f32,
        0.999076f32,
        0.999147f32,
        0.999213f32,
        0.999273f32,
        0.999329f32,
        0.999381f32,
        0.999428f32,
        0.999472f32,
        0.999513f32,
        0.999550f32,
        0.999585f32,
        0.999617f32,
        0.999646f32,
        0.999673f32,
        0.999699f32,
        0.999722f32,
        0.999743f32,
        0.999763f32,
        0.999781f32,
        0.999798f32,
        0.999813f32,
        0.999828f32,
        0.999841f32,
        0.999853f32,
        0.999865f32,
        0.999875f32,
        0.999885f32,
        0.999893f32,
        0.999902f32,
        0.999909f32,
        0.999916f32,
        0.999923f32,
        0.999929f32,
        0.999934f32,
        0.999939f32,
        0.999944f32,
        0.999948f32,
        0.999952f32,
        0.999956f32,
        0.999959f32,
        0.999962f32,
        0.999965f32,
        0.999968f32,
        0.999970f32,
        0.999973f32,
        0.999975f32,
        0.999977f32,
        0.999978f32,
        0.999980f32,
        0.999982f32,
        0.999983f32,
        0.999984f32,
        0.999986f32,
        0.999987f32,
        0.999988f32,
        0.999989f32,
        0.999990f32,
        0.999990f32,
        0.999991f32,
        0.999992f32,
        0.999992f32,
        0.999993f32,
        0.999994f32,
        0.999994f32,
        0.999994f32,
        0.999995f32,
        0.999995f32,
        0.999996f32,
        0.999996f32,
        0.999996f32,
        0.999997f32,
        0.999997f32,
        0.999997f32,
        0.999997f32,
        0.999997f32,
        0.999998f32,
        0.999998f32,
        0.999998f32,
        0.999998f32,
        0.999998f32,
        0.999998f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        0.999999f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
        1.000000f32,
    ];
}
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::src::opus_1_2_1::src::mlp::arch_h::celt_isnan;
pub use crate::src::opus_1_2_1::src::mlp::tansig_table_h::tansig_table;
use crate::stdlib::floor;
/* Copyright (c) 2008-2011 Octasic Inc.
Written by Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/*extern const float tansig_table[501];*/
#[inline]

unsafe extern "C" fn tansig_approx(mut x: libc::c_float) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_float = 0.;
    let mut dy: libc::c_float = 0.;
    let mut sign: libc::c_float = 1 as libc::c_int as libc::c_float;
    /* Tests are reversed to catch NaNs */
    if !(x < 8 as libc::c_int as libc::c_float) {
        return 1 as libc::c_int as libc::c_float;
    }
    if !(x > -(8 as libc::c_int) as libc::c_float) {
        return -(1 as libc::c_int) as libc::c_float;
    }
    /* Another check in case of -ffast-math */
    if celt_isnan(x) != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    if x < 0 as libc::c_int as libc::c_float {
        x = -x;
        sign = -(1 as libc::c_int) as libc::c_float
    }
    i = crate::stdlib::floor((0.5f32 + 25 as libc::c_int as libc::c_float * x) as libc::c_double)
        as libc::c_int;
    x -= 0.04f32 * i as libc::c_float;
    y = tansig_table[i as usize];
    dy = 1 as libc::c_int as libc::c_float - y * y;
    y = y + x * dy * (1 as libc::c_int as libc::c_float - y * x);
    return sign * y;
}
#[no_mangle]

pub unsafe extern "C" fn mlp_process(
    mut m: *const crate::src::opus_1_2_1::src::mlp::MLP,
    mut in_0: *const libc::c_float,
    mut out: *mut libc::c_float,
) {
    let mut j: libc::c_int = 0;
    let mut hidden: [libc::c_float; 100] = [0.; 100];
    let mut W: *const libc::c_float = (*m).weights;
    /* Copy to tmp_in */
    j = 0 as libc::c_int;
    while j < *(*m).topo.offset(1 as libc::c_int as isize) {
        let mut k: libc::c_int = 0;
        let fresh0 = W;
        W = W.offset(1);
        let mut sum: libc::c_float = *fresh0;
        k = 0 as libc::c_int;
        while k < *(*m).topo.offset(0 as libc::c_int as isize) {
            let fresh1 = W;
            W = W.offset(1);
            sum = sum + *in_0.offset(k as isize) * *fresh1;
            k += 1
        }
        hidden[j as usize] = tansig_approx(sum);
        j += 1
    }
    j = 0 as libc::c_int;
    while j < *(*m).topo.offset(2 as libc::c_int as isize) {
        let mut k_0: libc::c_int = 0;
        let fresh2 = W;
        W = W.offset(1);
        let mut sum_0: libc::c_float = *fresh2;
        k_0 = 0 as libc::c_int;
        while k_0 < *(*m).topo.offset(1 as libc::c_int as isize) {
            let fresh3 = W;
            W = W.offset(1);
            sum_0 = sum_0 + hidden[k_0 as usize] * *fresh3;
            k_0 += 1
        }
        *out.offset(j as isize) = tansig_approx(sum_0);
        j += 1
    }
}