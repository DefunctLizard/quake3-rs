use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT32;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_0;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JCS_CMYK;
pub use crate::jpeglib_h::JCS_GRAYSCALE;
pub use crate::jpeglib_h::JCS_RGB;
pub use crate::jpeglib_h::JCS_UNKNOWN;
pub use crate::jpeglib_h::JCS_YCCK;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::src::jpeg_8c::jaricom::jpeg_aritab;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_DROP_SAMPLING;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_LENGTH;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PRECISION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_STATE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jpeg_8c::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jpeg_8c::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jpeg_8c::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jpeg_8c::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_DAC_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_DAC_VALUE;
pub use crate::src::jpeg_8c::jerror::JERR_DHT_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_DQT_INDEX;
pub use crate::src::jpeg_8c::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jpeg_8c::jerror::JERR_EMS_READ;
pub use crate::src::jpeg_8c::jerror::JERR_EMS_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jpeg_8c::jerror::JERR_FILE_READ;
pub use crate::src::jpeg_8c::jerror::JERR_FILE_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jpeg_8c::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jpeg_8c::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jpeg_8c::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jpeg_8c::jerror::JERR_INPUT_EOF;
pub use crate::src::jpeg_8c::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_MISSING_DATA;
pub use crate::src::jpeg_8c::jerror::JERR_MODE_CHANGE;
pub use crate::src::jpeg_8c::jerror::JERR_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JERR_NOT_COMPILED;
pub use crate::src::jpeg_8c::jerror::JERR_NO_ARITH_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_IMAGE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jpeg_8c::jerror::JERR_NO_SOI;
pub use crate::src::jpeg_8c::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jpeg_8c::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jpeg_8c::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jpeg_8c::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jpeg_8c::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_CREATE;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_READ;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_SEEK;
pub use crate::src::jpeg_8c::jerror::JERR_TFILE_WRITE;
pub use crate::src::jpeg_8c::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jpeg_8c::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jpeg_8c::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jpeg_8c::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jpeg_8c::jerror::JERR_XMS_READ;
pub use crate::src::jpeg_8c::jerror::JERR_XMS_WRITE;
pub use crate::src::jpeg_8c::jerror::JMSG_COPYRIGHT;
pub use crate::src::jpeg_8c::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jpeg_8c::jerror::JMSG_NOMESSAGE;
pub use crate::src::jpeg_8c::jerror::JMSG_VERSION;
pub use crate::src::jpeg_8c::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jpeg_8c::jerror::JTRC_ADOBE;
pub use crate::src::jpeg_8c::jerror::JTRC_APP0;
pub use crate::src::jpeg_8c::jerror::JTRC_APP14;
pub use crate::src::jpeg_8c::jerror::JTRC_DAC;
pub use crate::src::jpeg_8c::jerror::JTRC_DHT;
pub use crate::src::jpeg_8c::jerror::JTRC_DQT;
pub use crate::src::jpeg_8c::jerror::JTRC_DRI;
pub use crate::src::jpeg_8c::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_EMS_OPEN;
pub use crate::src::jpeg_8c::jerror::JTRC_EOI;
pub use crate::src::jpeg_8c::jerror::JTRC_HUFFBITS;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jpeg_8c::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jpeg_8c::jerror::JTRC_MISC_MARKER;
pub use crate::src::jpeg_8c::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANTVALS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jpeg_8c::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jpeg_8c::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jpeg_8c::jerror::JTRC_RST;
pub use crate::src::jpeg_8c::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jpeg_8c::jerror::JTRC_SOF;
pub use crate::src::jpeg_8c::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jpeg_8c::jerror::JTRC_SOI;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jpeg_8c::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jpeg_8c::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jpeg_8c::jerror::JTRC_THUMB_RGB;
pub use crate::src::jpeg_8c::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jpeg_8c::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jpeg_8c::jerror::JTRC_XMS_OPEN;
pub use crate::src::jpeg_8c::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE;
pub use crate::src::jpeg_8c::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jpeg_8c::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jpeg_8c::jerror::JWRN_HIT_MARKER;
pub use crate::src::jpeg_8c::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jpeg_8c::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jpeg_8c::jerror::JWRN_JPEG_EOF;
pub use crate::src::jpeg_8c::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jpeg_8c::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jpeg_8c::jerror::JWRN_TOO_MUCH_DATA;
use crate::stdlib::memset;

pub type arith_entropy_ptr = *mut arith_entropy_decoder;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arith_entropy_decoder {
    pub pub_0: crate::jpegint_h::jpeg_entropy_decoder,
    pub c: crate::jmorecfg_h::INT32,
    pub a: crate::jmorecfg_h::INT32,
    pub ct: libc::c_int,
    pub last_dc_val: [libc::c_int; 4],
    pub dc_context: [libc::c_int; 4],
    pub restarts_to_go: libc::c_uint,
    pub dc_stats: [*mut libc::c_uchar; 16],
    pub ac_stats: [*mut libc::c_uchar; 16],
    pub fixed_bin: [libc::c_uchar; 4],
}

unsafe extern "C" fn get_byte(mut cinfo: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int
/* Read next input byte; we do not support suspension in this module. */ {
    let mut src: *mut crate::jpeglib_h::jpeg_source_mgr = (*cinfo).src;
    if (*src).bytes_in_buffer == 0 as libc::c_int as libc::c_ulong {
        if Some((*src).fill_input_buffer.expect("non-null function pointer"))
            .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JERR_CANT_SUSPEND as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    (*src).bytes_in_buffer = (*src).bytes_in_buffer.wrapping_sub(1);
    let fresh0 = (*src).next_input_byte;
    (*src).next_input_byte = (*src).next_input_byte.offset(1);
    return *fresh0 as libc::c_int;
}
/*
 * The core arithmetic decoding routine (common in JPEG and JBIG).
 * This needs to go as fast as possible.
 * Machine-dependent optimization facilities
 * are not utilized in this portable implementation.
 * However, this code should be fairly efficient and
 * may be a good base for further optimizations anyway.
 *
 * Return value is 0 or 1 (binary decision).
 *
 * Note: I've changed the handling of the code base & bit
 * buffer register C compared to other implementations
 * based on the standards layout & procedures.
 * While it also contains both the actual base of the
 * coding interval (16 bits) and the next-bits buffer,
 * the cut-point between these two parts is floating
 * (instead of fixed) with the bit shift counter CT.
 * Thus, we also need only one (variable instead of
 * fixed size) shift for the LPS/MPS decision, and
 * we can get away with any renormalization update
 * of C (except for new data insertion, of course).
 *
 * I've also introduced a new scheme for accessing
 * the probability estimation state machine table,
 * derived from Markus Kuhn's JBIG implementation.
 */

unsafe extern "C" fn arith_decode(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut st: *mut libc::c_uchar,
) -> libc::c_int {
    let mut e: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut nl: libc::c_uchar = 0;
    let mut nm: libc::c_uchar = 0;
    let mut qe: crate::jmorecfg_h::INT32 = 0;
    let mut temp: crate::jmorecfg_h::INT32 = 0;
    let mut sv: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    /* Renormalization & data input per section D.2.6 */
    while (*e).a < 0x8000 as libc::c_long {
        (*e).ct -= 1;
        if (*e).ct < 0 as libc::c_int {
            /* Need to fetch next data byte */
            if (*cinfo).unread_marker != 0 {
                data = 0 as libc::c_int
            } else {
                /* stuff zero data */
                data = get_byte(cinfo); /* read next input byte */
                if data == 0xff as libc::c_int {
                    loop
                    /* zero stuff or marker code */
                    {
                        data = get_byte(cinfo); /* swallow extra 0xFF bytes */
                        if !(data == 0xff as libc::c_int) {
                            break; /* discard stuffed zero byte */
                        }
                    }
                    if data == 0 as libc::c_int {
                        data = 0xff as libc::c_int
                    } else {
                        /* Note: Different from the Huffman decoder, hitting
                         * a marker while processing the compressed data
                         * segment is legal in arithmetic coding.
                         * The convention is to supply zero data
                         * then until decoding is complete.
                         */
                        (*cinfo).unread_marker = data;
                        data = 0 as libc::c_int
                    }
                }
            }
            /* => e->a = 0x10000L after loop exit */
            (*e).c = (*e).c << 8 as libc::c_int | data as libc::c_long; /* insert data into C register */
            (*e).ct += 8 as libc::c_int;
            if (*e).ct < 0 as libc::c_int {
                /* update bit shift counter */
                /* Need more initial bytes */
                (*e).ct += 1;
                if (*e).ct == 0 as libc::c_int {
                    /* Got 2 initial bytes -> re-init A and exit loop */
                    (*e).a = 0x8000 as libc::c_long
                }
            }
        }
        (*e).a <<= 1 as libc::c_int
    }
    /* Fetch values from our compact representation of Table D.2:
     * Qe values and probability estimation state machine
     */
    sv = *st as libc::c_int; /* => Qe_Value */
    qe = *crate::src::jpeg_8c::jaricom::jpeg_aritab
        .as_ptr()
        .offset((sv & 0x7f as libc::c_int) as isize); /* Next_Index_LPS + Switch_MPS */
    nl = (qe & 0xff as libc::c_int as libc::c_long) as libc::c_uchar; /* Next_Index_MPS */
    qe >>= 8 as libc::c_int;
    nm = (qe & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
    qe >>= 8 as libc::c_int;
    /* Decode & estimation procedures per sections D.2.4 & D.2.5 */
    temp = (*e).a - qe;
    (*e).a = temp;
    temp <<= (*e).ct;
    if (*e).c >= temp {
        (*e).c -= temp;
        /* Conditional LPS (less probable symbol) exchange */
        if (*e).a < qe {
            (*e).a = qe;
            *st = (sv & 0x80 as libc::c_int ^ nm as libc::c_int) as libc::c_uchar
        /* Estimate_after_MPS */
        } else {
            (*e).a = qe;
            /* Exchange LPS/MPS */
            *st = (sv & 0x80 as libc::c_int ^ nl as libc::c_int) as libc::c_uchar; /* Estimate_after_LPS */
            sv ^= 0x80 as libc::c_int
        }
    } else if (*e).a < 0x8000 as libc::c_long {
        /* Conditional MPS (more probable symbol) exchange */
        if (*e).a < qe {
            *st = (sv & 0x80 as libc::c_int ^ nl as libc::c_int) as libc::c_uchar;
            sv ^= 0x80 as libc::c_int /* Estimate_after_LPS */
        /* Exchange LPS/MPS */
        } else {
            *st = (sv & 0x80 as libc::c_int ^ nm as libc::c_int) as libc::c_uchar
            /* Estimate_after_MPS */
        }
    }
    return sv >> 7 as libc::c_int;
}
/*
 * Check for a restart marker & resynchronize decoder.
 */

unsafe extern "C" fn process_restart(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut ci: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    /* Advance past the RSTn marker */
    if Some(
        (*(*cinfo).marker)
            .read_restart_marker
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo)
        == 0
    {
        (*(*cinfo).err).msg_code = crate::src::jpeg_8c::jerror::JERR_CANT_SUSPEND as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Re-initialize statistics areas */
    ci = 0 as libc::c_int;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        if (*cinfo).progressive_mode == 0
            || (*cinfo).Ss == 0 as libc::c_int && (*cinfo).Ah == 0 as libc::c_int
        {
            crate::stdlib::memset(
                (*entropy).dc_stats[(*compptr).dc_tbl_no as usize] as *mut libc::c_void,
                0 as libc::c_int,
                64 as libc::c_int as crate::stddef_h::size_t,
            );
            /* Reset DC predictions to 0 */
            (*entropy).last_dc_val[ci as usize] = 0 as libc::c_int;
            (*entropy).dc_context[ci as usize] = 0 as libc::c_int
        }
        if (*cinfo).progressive_mode == 0 && (*cinfo).lim_Se != 0
            || (*cinfo).progressive_mode != 0 && (*cinfo).Ss != 0
        {
            crate::stdlib::memset(
                (*entropy).ac_stats[(*compptr).ac_tbl_no as usize] as *mut libc::c_void,
                0 as libc::c_int,
                256 as libc::c_int as crate::stddef_h::size_t,
            );
        }
        ci += 1
    }
    /* Reset arithmetic decoding variables */
    (*entropy).c = 0 as libc::c_int as crate::jmorecfg_h::INT32; /* force reading 2 initial bytes to fill C */
    (*entropy).a = 0 as libc::c_int as crate::jmorecfg_h::INT32;
    (*entropy).ct = -(16 as libc::c_int);
    /* Reset restart counter */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
/*
 * Arithmetic MCU decoding.
 * Each of these routines decodes and returns one MCU's worth of
 * arithmetic-compressed coefficients.
 * The coefficients are reordered from zigzag order into natural array order,
 * but are not dequantized.
 *
 * The i'th block of the MCU is stored into the block pointed to by
 * MCU_data[i].  WE ASSUME THIS AREA IS INITIALLY ZEROED BY THE CALLER.
 */
/*
 * MCU decoding for DC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn decode_mcu_DC_first(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut st: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut blkn: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut tbl: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    /* Process restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as libc::c_int as libc::c_uint {
            process_restart(cinfo); /* if error do nothing */
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    if (*entropy).ct == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    /* Outer loop handles each block in the MCU */
    blkn = 0 as libc::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        ci = (*cinfo).MCU_membership[blkn as usize];
        tbl = (*(*cinfo).cur_comp_info[ci as usize]).dc_tbl_no;
        /* Sections F.2.4.1 & F.1.4.4.1: Decoding of DC coefficients */
        /* Table F.4: Point to statistics bin S0 for DC coefficient coding */
        st = (*entropy).dc_stats[tbl as usize].offset((*entropy).dc_context[ci as usize] as isize);
        /* Figure F.19: Decode_DC_DIFF */
        if arith_decode(cinfo, st) == 0 as libc::c_int {
            (*entropy).dc_context[ci as usize] = 0 as libc::c_int
        } else {
            /* Figure F.21: Decoding nonzero value v */
            /* Figure F.22: Decoding the sign of v */
            sign = arith_decode(cinfo, st.offset(1 as libc::c_int as isize));
            st = st.offset(2 as libc::c_int as isize);
            st = st.offset(sign as isize);
            /* Figure F.23: Decoding the magnitude category of v */
            m = arith_decode(cinfo, st); /* Table F.4: X1 = 20 */
            if m != 0 as libc::c_int {
                st = (*entropy).dc_stats[tbl as usize].offset(20 as libc::c_int as isize); /* magnitude overflow */
                while arith_decode(cinfo, st) != 0 {
                    m <<= 1 as libc::c_int;
                    if m == 0x8000 as libc::c_int {
                        (*(*cinfo).err).msg_code =
                            crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE as libc::c_int;
                        Some(
                            (*(*cinfo).err)
                                .emit_message
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                            -(1 as libc::c_int),
                        );
                        (*entropy).ct = -(1 as libc::c_int);
                        return 1 as libc::c_int;
                    }
                    st = st.offset(1 as libc::c_int as isize)
                }
            }
            /* Section F.1.4.4.1.2: Establish dc_context conditioning category */
            if m < ((1 as libc::c_long) << (*cinfo).arith_dc_L[tbl as usize] as libc::c_int
                >> 1 as libc::c_int) as libc::c_int
            {
                /* small diff category */
                (*entropy).dc_context[ci as usize] = 0 as libc::c_int
            } else if m
                > ((1 as libc::c_long) << (*cinfo).arith_dc_U[tbl as usize] as libc::c_int
                    >> 1 as libc::c_int) as libc::c_int
            {
                /* zero diff category */
                (*entropy).dc_context[ci as usize] = 12 as libc::c_int + sign * 4 as libc::c_int
            } else {
                (*entropy).dc_context[ci as usize] = 4 as libc::c_int + sign * 4 as libc::c_int
            } /* large diff category */
            v = m;
            /* Figure F.24: Decoding the magnitude bit pattern of v */
            st = st.offset(14 as libc::c_int as isize);
            loop {
                m >>= 1 as libc::c_int;
                if !(m != 0) {
                    break;
                }
                if arith_decode(cinfo, st) != 0 {
                    v |= m
                }
            }
            v += 1 as libc::c_int;
            if sign != 0 {
                v = -v
            }
            (*entropy).last_dc_val[ci as usize] += v
        }
        /* Scale and output the DC coefficient (assumes jpeg_natural_order[0]=0) */
        (*block)[0 as libc::c_int as usize] =
            ((*entropy).last_dc_val[ci as usize] << (*cinfo).Al) as crate::jmorecfg_h::JCOEF;
        blkn += 1
    }
    return 1 as libc::c_int;
}
/*
 * MCU decoding for AC initial scan (either spectral selection,
 * or first pass of successive approximation).
 */

unsafe extern "C" fn decode_mcu_AC_first(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut st: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tbl: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut natural_order: *const libc::c_int = 0 as *const libc::c_int;
    /* Process restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as libc::c_int as libc::c_uint {
            process_restart(cinfo); /* if error do nothing */
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    if (*entropy).ct == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    natural_order = (*cinfo).natural_order;
    /* There is always only one block per MCU */
    block = *MCU_data.offset(0 as libc::c_int as isize);
    tbl = (*(*cinfo).cur_comp_info[0 as libc::c_int as usize]).ac_tbl_no;
    /* Sections F.2.4.2 & F.1.4.4.2: Decoding of AC coefficients */
    /* Figure F.20: Decode_AC_coefficients */
    k = (*cinfo).Ss; /* EOB flag */
    while k <= (*cinfo).Se {
        st = (*entropy).ac_stats[tbl as usize]
            .offset((3 as libc::c_int * (k - 1 as libc::c_int)) as isize); /* spectral overflow */
        if arith_decode(cinfo, st) != 0 {
            break;
        }
        while arith_decode(cinfo, st.offset(1 as libc::c_int as isize)) == 0 as libc::c_int {
            st = st.offset(3 as libc::c_int as isize);
            k += 1;
            if k > (*cinfo).Se {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    -(1 as libc::c_int),
                );
                (*entropy).ct = -(1 as libc::c_int);
                return 1 as libc::c_int;
            }
        }
        /* Figure F.21: Decoding nonzero value v */
        /* Figure F.22: Decoding the sign of v */
        sign = arith_decode(cinfo, (*entropy).fixed_bin.as_mut_ptr());
        st = st.offset(2 as libc::c_int as isize);
        /* Figure F.23: Decoding the magnitude category of v */
        m = arith_decode(cinfo, st); /* magnitude overflow */
        if m != 0 as libc::c_int {
            if arith_decode(cinfo, st) != 0 {
                m <<= 1 as libc::c_int;
                st = (*entropy).ac_stats[tbl as usize].offset(
                    (if k <= (*cinfo).arith_ac_K[tbl as usize] as libc::c_int {
                        189 as libc::c_int
                    } else {
                        217 as libc::c_int
                    }) as isize,
                );
                while arith_decode(cinfo, st) != 0 {
                    m <<= 1 as libc::c_int;
                    if m == 0x8000 as libc::c_int {
                        (*(*cinfo).err).msg_code =
                            crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE as libc::c_int;
                        Some(
                            (*(*cinfo).err)
                                .emit_message
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                            -(1 as libc::c_int),
                        );
                        (*entropy).ct = -(1 as libc::c_int);
                        return 1 as libc::c_int;
                    }
                    st = st.offset(1 as libc::c_int as isize)
                }
            }
        }
        v = m;
        /* Figure F.24: Decoding the magnitude bit pattern of v */
        st = st.offset(14 as libc::c_int as isize);
        loop {
            m >>= 1 as libc::c_int;
            if !(m != 0) {
                break;
            }
            if arith_decode(cinfo, st) != 0 {
                v |= m
            }
        }
        v += 1 as libc::c_int;
        if sign != 0 {
            v = -v
        }
        /* Scale and output coefficient in natural (dezigzagged) order */
        (*block)[*natural_order.offset(k as isize) as usize] =
            (v << (*cinfo).Al) as crate::jmorecfg_h::JCOEF;
        k += 1
    }
    return 1 as libc::c_int;
}
/*
 * MCU decoding for DC successive approximation refinement scan.
 */

unsafe extern "C" fn decode_mcu_DC_refine(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut st: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p1: libc::c_int = 0;
    let mut blkn: libc::c_int = 0;
    /* Process restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as libc::c_int as libc::c_uint {
            process_restart(cinfo); /* use fixed probability estimation */
        } /* 1 in the bit position being coded */
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    st = (*entropy).fixed_bin.as_mut_ptr();
    p1 = (1 as libc::c_int) << (*cinfo).Al;
    /* Outer loop handles each block in the MCU */
    blkn = 0 as libc::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        /* Encoded data is simply the next bit of the two's-complement DC value */
        if arith_decode(cinfo, st) != 0 {
            let ref mut fresh1 = (*(*MCU_data.offset(blkn as isize))
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
            *fresh1 = (*fresh1 as libc::c_int | p1) as crate::jmorecfg_h::JCOEF
        }
        blkn += 1
    }
    return 1 as libc::c_int;
}
/*
 * MCU decoding for AC successive approximation refinement scan.
 */

unsafe extern "C" fn decode_mcu_AC_refine(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut thiscoef: crate::jpeglib_h::JCOEFPTR = 0 as *mut crate::jmorecfg_h::JCOEF;
    let mut st: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tbl: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kex: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut natural_order: *const libc::c_int = 0 as *const libc::c_int;
    /* Process restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as libc::c_int as libc::c_uint {
            process_restart(cinfo); /* if error do nothing */
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    if (*entropy).ct == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    natural_order = (*cinfo).natural_order;
    /* There is always only one block per MCU */
    block = *MCU_data.offset(0 as libc::c_int as isize); /* 1 in the bit position being coded */
    tbl = (*(*cinfo).cur_comp_info[0 as libc::c_int as usize]).ac_tbl_no; /* -1 in the bit position being coded */
    p1 = (1 as libc::c_int) << (*cinfo).Al;
    m1 = (-(1 as libc::c_int)) << (*cinfo).Al;
    /* Establish EOBx (previous stage end-of-block) index */
    kex = (*cinfo).Se; /* EOB flag */
    while kex > 0 as libc::c_int {
        if (*block)[*natural_order.offset(kex as isize) as usize] != 0 {
            break;
        }
        kex -= 1
    }
    k = (*cinfo).Ss;
    while k <= (*cinfo).Se {
        st = (*entropy).ac_stats[tbl as usize]
            .offset((3 as libc::c_int * (k - 1 as libc::c_int)) as isize);
        if k > kex {
            if arith_decode(cinfo, st) != 0 {
                break;
            }
        }
        loop {
            thiscoef = (*block)
                .as_mut_ptr()
                .offset(*natural_order.offset(k as isize) as isize);
            if *thiscoef != 0 {
                /* previously nonzero coef */
                if arith_decode(cinfo, st.offset(2 as libc::c_int as isize)) != 0 {
                    if (*thiscoef as libc::c_int) < 0 as libc::c_int {
                        *thiscoef = (*thiscoef as libc::c_int + m1) as crate::jmorecfg_h::JCOEF
                    } else {
                        *thiscoef = (*thiscoef as libc::c_int + p1) as crate::jmorecfg_h::JCOEF
                    }
                }
                break;
            } else if arith_decode(cinfo, st.offset(1 as libc::c_int as isize)) != 0 {
                /* newly nonzero coef */
                if arith_decode(cinfo, (*entropy).fixed_bin.as_mut_ptr()) != 0 {
                    *thiscoef = m1 as crate::jmorecfg_h::JCOEF
                } else {
                    *thiscoef = p1 as crate::jmorecfg_h::JCOEF
                } /* spectral overflow */
                break;
            } else {
                st = st.offset(3 as libc::c_int as isize);
                k += 1;
                if k > (*cinfo).Se {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE as libc::c_int;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        -(1 as libc::c_int),
                    );
                    (*entropy).ct = -(1 as libc::c_int);
                    return 1 as libc::c_int;
                }
            }
        }
        k += 1
    }
    return 1 as libc::c_int;
}
/*
 * Decode one MCU's worth of arithmetic-compressed coefficients.
 */

unsafe extern "C" fn decode_mcu(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut MCU_data: *mut crate::jpeglib_h::JBLOCKROW,
) -> crate::jmorecfg_h::boolean {
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut block: crate::jpeglib_h::JBLOCKROW = 0 as *mut crate::jpeglib_h::JBLOCK;
    let mut st: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut blkn: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut tbl: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut natural_order: *const libc::c_int = 0 as *const libc::c_int;
    /* Process restart marker if needed */
    if (*cinfo).restart_interval != 0 {
        if (*entropy).restarts_to_go == 0 as libc::c_int as libc::c_uint {
            process_restart(cinfo); /* if error do nothing */
        }
        (*entropy).restarts_to_go = (*entropy).restarts_to_go.wrapping_sub(1)
    }
    if (*entropy).ct == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    natural_order = (*cinfo).natural_order;
    /* Outer loop handles each block in the MCU */
    blkn = 0 as libc::c_int;
    while blkn < (*cinfo).blocks_in_MCU {
        block = *MCU_data.offset(blkn as isize);
        ci = (*cinfo).MCU_membership[blkn as usize];
        compptr = (*cinfo).cur_comp_info[ci as usize];
        /* Sections F.2.4.1 & F.1.4.4.1: Decoding of DC coefficients */
        tbl = (*compptr).dc_tbl_no;
        /* Table F.4: Point to statistics bin S0 for DC coefficient coding */
        st = (*entropy).dc_stats[tbl as usize].offset((*entropy).dc_context[ci as usize] as isize);
        /* Figure F.19: Decode_DC_DIFF */
        if arith_decode(cinfo, st) == 0 as libc::c_int {
            (*entropy).dc_context[ci as usize] = 0 as libc::c_int
        } else {
            /* Figure F.21: Decoding nonzero value v */
            /* Figure F.22: Decoding the sign of v */
            sign = arith_decode(cinfo, st.offset(1 as libc::c_int as isize));
            st = st.offset(2 as libc::c_int as isize);
            st = st.offset(sign as isize);
            /* Figure F.23: Decoding the magnitude category of v */
            m = arith_decode(cinfo, st); /* Table F.4: X1 = 20 */
            if m != 0 as libc::c_int {
                st = (*entropy).dc_stats[tbl as usize].offset(20 as libc::c_int as isize); /* magnitude overflow */
                while arith_decode(cinfo, st) != 0 {
                    m <<= 1 as libc::c_int;
                    if m == 0x8000 as libc::c_int {
                        (*(*cinfo).err).msg_code =
                            crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE as libc::c_int;
                        Some(
                            (*(*cinfo).err)
                                .emit_message
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                            -(1 as libc::c_int),
                        );
                        (*entropy).ct = -(1 as libc::c_int);
                        return 1 as libc::c_int;
                    }
                    st = st.offset(1 as libc::c_int as isize)
                }
            }
            /* Section F.1.4.4.1.2: Establish dc_context conditioning category */
            if m < ((1 as libc::c_long) << (*cinfo).arith_dc_L[tbl as usize] as libc::c_int
                >> 1 as libc::c_int) as libc::c_int
            {
                /* small diff category */
                (*entropy).dc_context[ci as usize] = 0 as libc::c_int
            } else if m
                > ((1 as libc::c_long) << (*cinfo).arith_dc_U[tbl as usize] as libc::c_int
                    >> 1 as libc::c_int) as libc::c_int
            {
                /* zero diff category */
                (*entropy).dc_context[ci as usize] = 12 as libc::c_int + sign * 4 as libc::c_int
            } else {
                (*entropy).dc_context[ci as usize] = 4 as libc::c_int + sign * 4 as libc::c_int
            } /* large diff category */
            v = m;
            /* Figure F.24: Decoding the magnitude bit pattern of v */
            st = st.offset(14 as libc::c_int as isize);
            loop {
                m >>= 1 as libc::c_int;
                if !(m != 0) {
                    break;
                }
                if arith_decode(cinfo, st) != 0 {
                    v |= m
                }
            }
            v += 1 as libc::c_int;
            if sign != 0 {
                v = -v
            }
            (*entropy).last_dc_val[ci as usize] += v
        }
        (*block)[0 as libc::c_int as usize] =
            (*entropy).last_dc_val[ci as usize] as crate::jmorecfg_h::JCOEF;
        /* Sections F.2.4.2 & F.1.4.4.2: Decoding of AC coefficients */
        tbl = (*compptr).ac_tbl_no;
        /* Figure F.20: Decode_AC_coefficients */
        k = 1 as libc::c_int; /* EOB flag */
        while k <= (*cinfo).lim_Se {
            st = (*entropy).ac_stats[tbl as usize]
                .offset((3 as libc::c_int * (k - 1 as libc::c_int)) as isize); /* spectral overflow */
            if arith_decode(cinfo, st) != 0 {
                break;
            }
            while arith_decode(cinfo, st.offset(1 as libc::c_int as isize)) == 0 as libc::c_int {
                st = st.offset(3 as libc::c_int as isize);
                k += 1;
                if k > (*cinfo).lim_Se {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE as libc::c_int;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        -(1 as libc::c_int),
                    );
                    (*entropy).ct = -(1 as libc::c_int);
                    return 1 as libc::c_int;
                }
            }
            /* Figure F.21: Decoding nonzero value v */
            /* Figure F.22: Decoding the sign of v */
            sign = arith_decode(cinfo, (*entropy).fixed_bin.as_mut_ptr());
            st = st.offset(2 as libc::c_int as isize);
            /* Figure F.23: Decoding the magnitude category of v */
            m = arith_decode(cinfo, st); /* magnitude overflow */
            if m != 0 as libc::c_int {
                if arith_decode(cinfo, st) != 0 {
                    m <<= 1 as libc::c_int;
                    st = (*entropy).ac_stats[tbl as usize].offset(
                        (if k <= (*cinfo).arith_ac_K[tbl as usize] as libc::c_int {
                            189 as libc::c_int
                        } else {
                            217 as libc::c_int
                        }) as isize,
                    );
                    while arith_decode(cinfo, st) != 0 {
                        m <<= 1 as libc::c_int;
                        if m == 0x8000 as libc::c_int {
                            (*(*cinfo).err).msg_code =
                                crate::src::jpeg_8c::jerror::JWRN_ARITH_BAD_CODE as libc::c_int;
                            Some(
                                (*(*cinfo).err)
                                    .emit_message
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                cinfo as crate::jpeglib_h::j_common_ptr,
                                -(1 as libc::c_int),
                            );
                            (*entropy).ct = -(1 as libc::c_int);
                            return 1 as libc::c_int;
                        }
                        st = st.offset(1 as libc::c_int as isize)
                    }
                }
            }
            v = m;
            /* Figure F.24: Decoding the magnitude bit pattern of v */
            st = st.offset(14 as libc::c_int as isize);
            loop {
                m >>= 1 as libc::c_int;
                if !(m != 0) {
                    break;
                }
                if arith_decode(cinfo, st) != 0 {
                    v |= m
                }
            }
            v += 1 as libc::c_int;
            if sign != 0 {
                v = -v
            }
            (*block)[*natural_order.offset(k as isize) as usize] = v as crate::jmorecfg_h::JCOEF;
            k += 1
        }
        blkn += 1
    }
    return 1 as libc::c_int;
}
/*
 * Initialize for an arithmetic-compressed scan.
 */

unsafe extern "C" fn start_pass(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut current_block: u64;
    let mut entropy: arith_entropy_ptr = (*cinfo).entropy as arith_entropy_ptr;
    let mut ci: libc::c_int = 0;
    let mut tbl: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    if (*cinfo).progressive_mode != 0 {
        /* Validate progressive scan parameters */
        if (*cinfo).Ss == 0 as libc::c_int {
            if (*cinfo).Se != 0 as libc::c_int {
                current_block = 1247655281935294708;
            } else {
                current_block = 8515828400728868193;
            }
        } else if (*cinfo).Se < (*cinfo).Ss || (*cinfo).Se > (*cinfo).lim_Se {
            current_block = 1247655281935294708;
        } else if (*cinfo).comps_in_scan != 1 as libc::c_int {
            current_block = 1247655281935294708;
        } else {
            current_block = 8515828400728868193;
        }
        match current_block {
            8515828400728868193 => {
                if (*cinfo).Ah != 0 as libc::c_int {
                    /* need not check Ss/Se < 0 since they came from unsigned bytes */
                    /* AC scans may have only one component */
                    /* Successive approximation refinement scan: must have Al = Ah-1. */
                    if (*cinfo).Ah - 1 as libc::c_int != (*cinfo).Al {
                        current_block = 1247655281935294708;
                    } else {
                        current_block = 7746791466490516765;
                    }
                } else {
                    current_block = 7746791466490516765;
                }
                match current_block {
                    1247655281935294708 => {}
                    _ => {
                        if (*cinfo).Al > 13 as libc::c_int {
                            current_block = 1247655281935294708;
                        } else {
                            current_block = 12599329904712511516;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            1247655281935294708 =>
            /* need not check for < 0 */
            {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_BAD_PROGRESSION as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*cinfo).Ss;
                (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = (*cinfo).Se;
                (*(*cinfo).err).msg_parm.i[2 as libc::c_int as usize] = (*cinfo).Ah;
                (*(*cinfo).err).msg_parm.i[3 as libc::c_int as usize] = (*cinfo).Al;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            _ => {}
        }
        /* Update progression status, and verify that scan order is legal.
         * Note that inter-scan inconsistencies are treated as warnings
         * not fatal errors ... not clear if this is right way to behave.
         */
        ci = 0 as libc::c_int;
        while ci < (*cinfo).comps_in_scan {
            let mut coefi: libc::c_int = 0;
            let mut cindex: libc::c_int = (*(*cinfo).cur_comp_info[ci as usize]).component_index;
            let mut coef_bit_ptr: *mut libc::c_int =
                &mut *(*(*cinfo).coef_bits.offset(cindex as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_int;
            if (*cinfo).Ss != 0
                && *coef_bit_ptr.offset(0 as libc::c_int as isize) < 0 as libc::c_int
            {
                /* AC without prior DC scan */
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JWRN_BOGUS_PROGRESSION as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = cindex;
                (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = 0 as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    -(1 as libc::c_int),
                );
            }
            coefi = (*cinfo).Ss;
            while coefi <= (*cinfo).Se {
                let mut expected: libc::c_int =
                    if *coef_bit_ptr.offset(coefi as isize) < 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        *coef_bit_ptr.offset(coefi as isize)
                    };
                if (*cinfo).Ah != expected {
                    (*(*cinfo).err).msg_code =
                        crate::src::jpeg_8c::jerror::JWRN_BOGUS_PROGRESSION as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = cindex;
                    (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = coefi;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        -(1 as libc::c_int),
                    );
                }
                *coef_bit_ptr.offset(coefi as isize) = (*cinfo).Al;
                coefi += 1
            }
            ci += 1
        }
        /* Select MCU decoding routine */
        if (*cinfo).Ah == 0 as libc::c_int {
            if (*cinfo).Ss == 0 as libc::c_int {
                (*entropy).pub_0.decode_mcu = Some(
                    decode_mcu_DC_first
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::JBLOCKROW,
                        )
                            -> crate::jmorecfg_h::boolean,
                )
            } else {
                (*entropy).pub_0.decode_mcu = Some(
                    decode_mcu_AC_first
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::JBLOCKROW,
                        )
                            -> crate::jmorecfg_h::boolean,
                )
            }
        } else if (*cinfo).Ss == 0 as libc::c_int {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_DC_refine
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: *mut crate::jpeglib_h::JBLOCKROW,
                    ) -> crate::jmorecfg_h::boolean,
            )
        } else {
            (*entropy).pub_0.decode_mcu = Some(
                decode_mcu_AC_refine
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: *mut crate::jpeglib_h::JBLOCKROW,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
    } else {
        /* Check that the scan parameters Ss, Se, Ah/Al are OK for sequential JPEG.
         * This ought to be an error condition, but we make it a warning.
         */
        if (*cinfo).Ss != 0 as libc::c_int
            || (*cinfo).Ah != 0 as libc::c_int
            || (*cinfo).Al != 0 as libc::c_int
            || (*cinfo).Se < 64 as libc::c_int && (*cinfo).Se != (*cinfo).lim_Se
        {
            (*(*cinfo).err).msg_code =
                crate::src::jpeg_8c::jerror::JWRN_NOT_SEQUENTIAL as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                -(1 as libc::c_int),
            );
        }
        /* Select MCU decoding routine */
        (*entropy).pub_0.decode_mcu = Some(
            decode_mcu
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_decompress_ptr,
                    _: *mut crate::jpeglib_h::JBLOCKROW,
                ) -> crate::jmorecfg_h::boolean,
        )
    }
    /* Allocate & initialize requested statistics areas */
    ci = 0 as libc::c_int;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        if (*cinfo).progressive_mode == 0
            || (*cinfo).Ss == 0 as libc::c_int && (*cinfo).Ah == 0 as libc::c_int
        {
            tbl = (*compptr).dc_tbl_no;
            if tbl < 0 as libc::c_int || tbl >= 16 as libc::c_int {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_NO_ARITH_TABLE as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = tbl;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if (*entropy).dc_stats[tbl as usize].is_null() {
                (*entropy).dc_stats[tbl as usize] = Some(
                    (*(*cinfo).mem)
                        .alloc_small
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as libc::c_int,
                    64 as libc::c_int as crate::stddef_h::size_t,
                ) as *mut libc::c_uchar
            }
            crate::stdlib::memset(
                (*entropy).dc_stats[tbl as usize] as *mut libc::c_void,
                0 as libc::c_int,
                64 as libc::c_int as crate::stddef_h::size_t,
            );
            /* Initialize DC predictions to 0 */
            (*entropy).last_dc_val[ci as usize] = 0 as libc::c_int;
            (*entropy).dc_context[ci as usize] = 0 as libc::c_int
        }
        if (*cinfo).progressive_mode == 0 && (*cinfo).lim_Se != 0
            || (*cinfo).progressive_mode != 0 && (*cinfo).Ss != 0
        {
            tbl = (*compptr).ac_tbl_no;
            if tbl < 0 as libc::c_int || tbl >= 16 as libc::c_int {
                (*(*cinfo).err).msg_code =
                    crate::src::jpeg_8c::jerror::JERR_NO_ARITH_TABLE as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = tbl;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if (*entropy).ac_stats[tbl as usize].is_null() {
                (*entropy).ac_stats[tbl as usize] = Some(
                    (*(*cinfo).mem)
                        .alloc_small
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr,
                    1 as libc::c_int,
                    256 as libc::c_int as crate::stddef_h::size_t,
                ) as *mut libc::c_uchar
            }
            crate::stdlib::memset(
                (*entropy).ac_stats[tbl as usize] as *mut libc::c_void,
                0 as libc::c_int,
                256 as libc::c_int as crate::stddef_h::size_t,
            );
        }
        ci += 1
    }
    /* Initialize arithmetic decoding variables */
    (*entropy).c = 0 as libc::c_int as crate::jmorecfg_h::INT32; /* force reading 2 initial bytes to fill C */
    (*entropy).a = 0 as libc::c_int as crate::jmorecfg_h::INT32;
    (*entropy).ct = -(16 as libc::c_int);
    /* Initialize restart counter */
    (*entropy).restarts_to_go = (*cinfo).restart_interval;
}
/* It is useful to allow each component to have a separate IDCT method. */
/* Upsampling (note that upsampler must also call color converter) */
/* TRUE if need rows above & below */
/* Colorspace conversion */
/* Color quantization or color precision reduction */
/* Miscellaneous useful macros */
/* We assume that right shift corresponds to signed division by 2 with
 * rounding towards minus infinity.  This is correct for typical "arithmetic
 * shift" instructions that shift in copies of the sign bit.  But some
 * C compilers implement >> with an unsigned shift.  For these machines you
 * must define RIGHT_SHIFT_IS_UNSIGNED.
 * RIGHT_SHIFT provides a proper signed right shift of an INT32 quantity.
 * It is only applied with constant shift counts.  SHIFT_TEMPS must be
 * included in the variables of any routine using RIGHT_SHIFT.
 */
/* Short forms of external names for systems with brain-damaged linkers. */
/* NEED_SHORT_EXTERNAL_NAMES */
/* Compression module initialization routines */
/* Decompression module initialization routines */
/*
 * Module initialization routine for arithmetic entropy decoding.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_arith_decoder(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut entropy: arith_entropy_ptr = 0 as *mut arith_entropy_decoder;
    let mut i: libc::c_int = 0;
    entropy = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        1 as libc::c_int,
        ::std::mem::size_of::<arith_entropy_decoder>() as libc::c_ulong,
    ) as arith_entropy_ptr;
    (*cinfo).entropy = entropy as *mut crate::jpegint_h::jpeg_entropy_decoder;
    (*entropy).pub_0.start_pass =
        Some(start_pass as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ());
    /* Mark tables unallocated */
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*entropy).dc_stats[i as usize] = 0 as *mut libc::c_uchar;
        (*entropy).ac_stats[i as usize] = 0 as *mut libc::c_uchar;
        i += 1
    }
    /* Initialize index for fixed probability estimation */
    (*entropy).fixed_bin[0 as libc::c_int as usize] = 113 as libc::c_int as libc::c_uchar;
    if (*cinfo).progressive_mode != 0 {
        /* Create progression status table */
        let mut coef_bit_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut ci: libc::c_int = 0;
        (*cinfo).coef_bits = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            1 as libc::c_int,
            (((*cinfo).num_components * 64 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut [libc::c_int; 64];
        coef_bit_ptr = &mut *(*(*cinfo).coef_bits.offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_int;
        ci = 0 as libc::c_int;
        while ci < (*cinfo).num_components {
            i = 0 as libc::c_int;
            while i < 64 as libc::c_int {
                let fresh2 = coef_bit_ptr;
                coef_bit_ptr = coef_bit_ptr.offset(1);
                *fresh2 = -(1 as libc::c_int);
                i += 1
            }
            ci += 1
        }
    };
}
