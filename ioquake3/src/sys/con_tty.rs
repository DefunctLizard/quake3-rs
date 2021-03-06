use ::c2rust_asm_casts;
use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__fd_mask;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__suseconds_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::fd_set;
pub use crate::stdlib::fputs;
pub use crate::stdlib::select;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::stderr;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
pub use ::libc::timeval;
use c2rust_asm_casts::AsmCastTrait;

pub use crate::qcommon_h::field_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::src::qcommon::common::com_ansiColor;
pub use crate::src::qcommon::common::con_autochat;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Field_AutoComplete;
pub use crate::src::qcommon::common::Field_Clear;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use ::libc::termios;

pub use crate::client_h::clientConnection_t;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::src::client::cl_main::clc;
use crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
use crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
use crate::src::sys::sys_main::Sys_AnsiColorPrint;
pub use crate::stdlib::__sighandler_t;
pub use crate::stdlib::cc_t;
use crate::stdlib::memmove;
pub use crate::stdlib::signal;
pub use crate::stdlib::speed_t;
use crate::stdlib::strlen;
pub use crate::stdlib::tcflag_t;

use crate::stdlib::read;
use crate::stdlib::write;
use ::libc::fcntl;
use ::libc::tcflush;
use ::libc::tcgetattr;
use ::libc::tcsetattr;
extern "C" {
    /*
    ===========================================================================
    Copyright (C) 1999-2005 Id Software, Inc.

    This file is part of Quake III Arena source code.

    Quake III Arena source code is free software; you can redistribute it
    and/or modify it under the terms of the GNU General Public License as
    published by the Free Software Foundation; either version 2 of the License,
    or (at your option) any later version.

    Quake III Arena source code is distributed in the hope that it will be
    useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Quake III Arena source code; if not, write to the Free Software
    Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
    ===========================================================================
    */
    /*
    =============================================================
    tty console routines

    NOTE: if the user is editing a line when something gets printed to the early
    console then it won't look good so we provide CON_Hide and CON_Show to be
    called before and after a stdout or stderr output
    =============================================================
    */
    #[no_mangle]
    pub static mut stdinIsATTY: crate::src::qcommon::q_shared::qboolean;
}

static mut stdin_active: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
// general flag to tell about tty console mode

static mut ttycon_on: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;

static mut ttycon_hide: libc::c_int = 0 as libc::c_int;

static mut ttycon_show_overdue: libc::c_int = 0 as libc::c_int;
// some key codes that the terminal may be using, initialised on start up

static mut TTY_erase: libc::c_int = 0;

static mut TTY_eof: libc::c_int = 0;

static mut TTY_tc: ::libc::termios = ::libc::termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};

static mut TTY_con: crate::qcommon_h::field_t = crate::qcommon_h::field_t {
    cursor: 0,
    scroll: 0,
    widthInChars: 0,
    buffer: [0; 256],
};

static mut ttyEditLines: [crate::qcommon_h::field_t; 32] = [crate::qcommon_h::field_t {
    cursor: 0,
    scroll: 0,
    widthInChars: 0,
    buffer: [0; 256],
}; 32];

static mut hist_current: libc::c_int = -(1 as libc::c_int);

static mut hist_count: libc::c_int = 0 as libc::c_int;
/*
==================
CON_Back

Output a backspace

NOTE: it seems on some terminals just sending '\b' is not enough so instead we
send "\b \b"
(FIXME there may be a way to find out if '\b' alone would work though)
==================
*/

unsafe extern "C" fn CON_Back() {
    let mut key: libc::c_char = 0;
    let mut size: crate::stddef_h::size_t = 0;
    key = '\u{8}' as i32 as libc::c_char;
    size = crate::stdlib::write(
        1 as libc::c_int,
        &mut key as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as crate::stddef_h::size_t,
    ) as crate::stddef_h::size_t;
    key = ' ' as i32 as libc::c_char;
    size = crate::stdlib::write(
        1 as libc::c_int,
        &mut key as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as crate::stddef_h::size_t,
    ) as crate::stddef_h::size_t;
    key = '\u{8}' as i32 as libc::c_char;
    size = crate::stdlib::write(
        1 as libc::c_int,
        &mut key as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as crate::stddef_h::size_t,
    ) as crate::stddef_h::size_t;
}
/*
==================
CON_Hide

Clear the display of the line currently edited
bring cursor back to beginning of line
==================
*/

unsafe extern "C" fn CON_Hide() {
    if ttycon_on as u64 != 0 {
        let mut i: libc::c_int = 0;
        if ttycon_hide != 0 {
            ttycon_hide += 1;
            return;
        }
        if TTY_con.cursor > 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < TTY_con.cursor {
                CON_Back();
                i += 1
            }
        }
        // Delete prompt
        i = crate::stdlib::strlen(b"tty]\x00" as *const u8 as *const libc::c_char) as libc::c_int;
        while i > 0 as libc::c_int {
            CON_Back();
            i -= 1
        }
        ttycon_hide += 1
    };
}
/*
==================
CON_Show

Show the current line
FIXME need to position the cursor if needed?
==================
*/

unsafe extern "C" fn CON_Show() {
    if ttycon_on as u64 != 0 {
        let mut i: libc::c_int = 0;
        ttycon_hide -= 1;
        if ttycon_hide == 0 as libc::c_int {
            let mut size: crate::stddef_h::size_t = 0;
            size = crate::stdlib::write(
                1 as libc::c_int,
                b"tty]\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                crate::stdlib::strlen(b"tty]\x00" as *const u8 as *const libc::c_char),
            ) as crate::stddef_h::size_t;
            if TTY_con.cursor != 0 {
                i = 0 as libc::c_int;
                while i < TTY_con.cursor {
                    size = crate::stdlib::write(
                        1 as libc::c_int,
                        TTY_con.buffer.as_mut_ptr().offset(i as isize) as *const libc::c_void,
                        1 as libc::c_int as crate::stddef_h::size_t,
                    ) as crate::stddef_h::size_t;
                    i += 1
                }
            }
        }
    };
}
/*
==================
CON_Shutdown

Never exit without calling this, or your terminal will be left in a pretty bad state
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_Shutdown() {
    if ttycon_on as u64 != 0 {
        CON_Hide();
        ::libc::tcsetattr(
            0 as libc::c_int,
            1 as libc::c_int,
            &mut TTY_tc as *mut _ as *const ::libc::termios,
        );
    }
    // Restore blocking to stdin reads
    ::libc::fcntl(
        0 as libc::c_int,
        4 as libc::c_int,
        ::libc::fcntl(0 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int)
            & !(0o4000 as libc::c_int),
    );
}
/*
==================
Hist_Add
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Hist_Add(mut field: *mut crate::qcommon_h::field_t) {
    let mut i: libc::c_int = 0;
    // Don't save blank lines in history.
    if (*field).cursor == 0 {
        return;
    }
    // make some room
    i = 32 as libc::c_int - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        ttyEditLines[i as usize] = ttyEditLines[(i - 1 as libc::c_int) as usize];
        i -= 1
    }
    ttyEditLines[0 as libc::c_int as usize] = *field;
    if hist_count < 32 as libc::c_int {
        hist_count += 1
    }
    hist_current = -(1 as libc::c_int);
    // re-init
}
/*
==================
Hist_Prev
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Hist_Prev() -> *mut crate::qcommon_h::field_t {
    let mut hist_prev: libc::c_int = 0;
    hist_prev = hist_current + 1 as libc::c_int;
    if hist_prev >= hist_count {
        return 0 as *mut crate::qcommon_h::field_t;
    }
    hist_current += 1;
    return &mut *ttyEditLines.as_mut_ptr().offset(hist_current as isize)
        as *mut crate::qcommon_h::field_t;
}
/*
==================
Hist_Next
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Hist_Next() -> *mut crate::qcommon_h::field_t {
    if hist_current >= 0 as libc::c_int {
        hist_current -= 1
    }
    if hist_current == -(1 as libc::c_int) {
        return 0 as *mut crate::qcommon_h::field_t;
    }
    return &mut *ttyEditLines.as_mut_ptr().offset(hist_current as isize)
        as *mut crate::qcommon_h::field_t;
}
/*
==================
CON_SigCont
Reinitialize console input after receiving SIGCONT, as on Linux the terminal seems to lose all
set attributes if user did CTRL+Z and then does fg again.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_SigCont(mut signum: libc::c_int) {
    CON_Init();
}
/*
==================
CON_Init

Initialize the console input (tty mode if possible)
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_Init() {
    let mut tc: ::libc::termios = ::libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    // If the process is backgrounded (running non interactively)
    // then SIGTTIN or SIGTOU is emitted, if not caught, turns into a SIGSTP
    crate::stdlib::signal(
        21 as libc::c_int,
        ::std::mem::transmute::<libc::intptr_t, crate::stdlib::__sighandler_t>(
            1 as libc::c_int as libc::intptr_t,
        ),
    );
    crate::stdlib::signal(
        22 as libc::c_int,
        ::std::mem::transmute::<libc::intptr_t, crate::stdlib::__sighandler_t>(
            1 as libc::c_int as libc::intptr_t,
        ),
    );
    // If SIGCONT is received, reinitialize console
    crate::stdlib::signal(
        18 as libc::c_int,
        Some(CON_SigCont as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
    // Make stdin reads non-blocking
    ::libc::fcntl(
        0 as libc::c_int,
        4 as libc::c_int,
        ::libc::fcntl(0 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int) | 0o4000 as libc::c_int,
    );
    if stdinIsATTY as u64 == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"tty console mode disabled\n\x00" as *const u8 as *const libc::c_char,
        );
        ttycon_on = crate::src::qcommon::q_shared::qfalse;
        stdin_active = crate::src::qcommon::q_shared::qtrue;
        return;
    }
    crate::src::qcommon::common::Field_Clear(
        &mut TTY_con as *mut _ as *mut crate::qcommon_h::field_t,
    );
    ::libc::tcgetattr(
        0 as libc::c_int,
        &mut TTY_tc as *mut _ as *mut ::libc::termios,
    );
    TTY_erase = TTY_tc.c_cc[2 as libc::c_int as usize] as libc::c_int;
    TTY_eof = TTY_tc.c_cc[4 as libc::c_int as usize] as libc::c_int;
    tc = TTY_tc;
    /*
    ECHO: don't echo input characters
    ICANON: enable canonical mode.  This  enables  the  special
    characters  EOF,  EOL,  EOL2, ERASE, KILL, REPRINT,
    STATUS, and WERASE, and buffers by lines.
    ISIG: when any of the characters  INTR,  QUIT,  SUSP,  or
    DSUSP are received, generate the corresponding signal
    */
    tc.c_lflag &= !(0o10 as libc::c_int | 0o2 as libc::c_int) as libc::c_uint;
    /*
    ISTRIP strip off bit 8
    INPCK enable input parity checking
    */
    tc.c_iflag &= !(0o40 as libc::c_int | 0o20 as libc::c_int) as libc::c_uint; // Mark as hidden, so prompt is shown in CON_Show
    tc.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as crate::stdlib::cc_t;
    tc.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::cc_t;
    ::libc::tcsetattr(
        0 as libc::c_int,
        1 as libc::c_int,
        &mut tc as *mut _ as *const ::libc::termios,
    );
    ttycon_on = crate::src::qcommon::q_shared::qtrue;
    ttycon_hide = 1 as libc::c_int;
    CON_Show();
}
/*
==================
CON_Input
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_Input() -> *mut libc::c_char {
    // we use this when sending back commands
    static mut text: [libc::c_char; 256] = [0; 256];
    let mut avail: libc::c_int = 0;
    let mut key: libc::c_char = 0;
    let mut history: *mut crate::qcommon_h::field_t = 0 as *mut crate::qcommon_h::field_t;
    let mut size: crate::stddef_h::size_t = 0;
    if ttycon_on as u64 != 0 {
        avail = crate::stdlib::read(
            0 as libc::c_int,
            &mut key as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as crate::stddef_h::size_t,
        ) as libc::c_int;
        if avail != -(1 as libc::c_int) {
            // we have something
            // backspace?
            // NOTE TTimo testing a lot of values .. seems it's the only way to get it to work everywhere
            if key as libc::c_int == TTY_erase
                || key as libc::c_int == 127 as libc::c_int
                || key as libc::c_int == 8 as libc::c_int
            {
                if TTY_con.cursor > 0 as libc::c_int {
                    TTY_con.cursor -= 1;
                    TTY_con.buffer[TTY_con.cursor as usize] = '\u{0}' as i32 as libc::c_char;
                    CON_Back();
                }
                return 0 as *mut libc::c_char;
            }
            // check if this is a control char
            if key as libc::c_int != 0 && (key as libc::c_int) < ' ' as i32 {
                if key as libc::c_int == '\n' as i32 {
                    // if not in the game explicitly prepend a slash if needed
                    if crate::src::client::cl_main::clc.state as libc::c_uint
                        != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
                        && (*crate::src::qcommon::common::con_autochat).integer != 0
                        && TTY_con.cursor != 0
                        && TTY_con.buffer[0 as libc::c_int as usize] as libc::c_int != '/' as i32
                        && TTY_con.buffer[0 as libc::c_int as usize] as libc::c_int != '\\' as i32
                    {
                        crate::stdlib::memmove(
                            TTY_con
                                .buffer
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize)
                                as *mut libc::c_void,
                            TTY_con.buffer.as_mut_ptr() as *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        TTY_con.buffer[0 as libc::c_int as usize] = '\\' as i32 as libc::c_char;
                        TTY_con.cursor += 1
                    }
                    if TTY_con.buffer[0 as libc::c_int as usize] as libc::c_int == '/' as i32
                        || TTY_con.buffer[0 as libc::c_int as usize] as libc::c_int == '\\' as i32
                    {
                        crate::src::qcommon::q_shared::Q_strncpyz(
                            text.as_mut_ptr(),
                            TTY_con
                                .buffer
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                                as libc::c_int,
                        );
                    } else if TTY_con.cursor != 0 {
                        if (*crate::src::qcommon::common::con_autochat).integer != 0 {
                            crate::src::qcommon::q_shared::Com_sprintf(
                                text.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                                    as libc::c_int,
                                b"cmd say %s\x00" as *const u8 as *const libc::c_char,
                                TTY_con.buffer.as_mut_ptr(),
                            );
                        } else {
                            crate::src::qcommon::q_shared::Q_strncpyz(
                                text.as_mut_ptr(),
                                TTY_con.buffer.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                                    as libc::c_int,
                            );
                        }
                    } else {
                        text[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
                    }
                    // push it in history
                    Hist_Add(&mut TTY_con);
                    CON_Hide();
                    crate::src::qcommon::common::Com_Printf(
                        b"%s%s\n\x00" as *const u8 as *const libc::c_char,
                        b"tty]\x00" as *const u8 as *const libc::c_char,
                        TTY_con.buffer.as_mut_ptr(),
                    );
                    crate::src::qcommon::common::Field_Clear(
                        &mut TTY_con as *mut _ as *mut crate::qcommon_h::field_t,
                    );
                    CON_Show();
                    return text.as_mut_ptr();
                }
                if key as libc::c_int == '\t' as i32 {
                    CON_Hide();
                    crate::src::qcommon::common::Field_AutoComplete(
                        &mut TTY_con as *mut _ as *mut crate::qcommon_h::field_t,
                    );
                    CON_Show();
                    return 0 as *mut libc::c_char;
                }
                avail = crate::stdlib::read(
                    0 as libc::c_int,
                    &mut key as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int as crate::stddef_h::size_t,
                ) as libc::c_int;
                if avail != -(1 as libc::c_int) {
                    // VT 100 keys
                    if key as libc::c_int == '[' as i32 || key as libc::c_int == 'O' as i32 {
                        avail = crate::stdlib::read(
                            0 as libc::c_int,
                            &mut key as *mut libc::c_char as *mut libc::c_void,
                            1 as libc::c_int as crate::stddef_h::size_t,
                        ) as libc::c_int;
                        if avail != -(1 as libc::c_int) {
                            match key as libc::c_int {
                                65 => {
                                    history = Hist_Prev();
                                    if !history.is_null() {
                                        CON_Hide();
                                        TTY_con = *history;
                                        CON_Show();
                                    }
                                    ::libc::tcflush(0 as libc::c_int, 0 as libc::c_int);
                                    return 0 as *mut libc::c_char;
                                }
                                66 => {
                                    history = Hist_Next();
                                    CON_Hide();
                                    if !history.is_null() {
                                        TTY_con = *history
                                    } else {
                                        crate::src::qcommon::common::Field_Clear(
                                            &mut TTY_con as *mut _
                                                as *mut crate::qcommon_h::field_t,
                                        );
                                    }
                                    CON_Show();
                                    ::libc::tcflush(0 as libc::c_int, 0 as libc::c_int);
                                    return 0 as *mut libc::c_char;
                                }
                                67 => return 0 as *mut libc::c_char,
                                68 => return 0 as *mut libc::c_char,
                                _ => {}
                            }
                        }
                    }
                }
                crate::src::qcommon::common::Com_DPrintf(
                    b"droping ISCTL sequence: %d, TTY_erase: %d\n\x00" as *const u8
                        as *const libc::c_char,
                    key as libc::c_int,
                    TTY_erase,
                );
                ::libc::tcflush(0 as libc::c_int, 0 as libc::c_int);
                return 0 as *mut libc::c_char;
            }
            if TTY_con.cursor as libc::c_ulong
                >= (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                return 0 as *mut libc::c_char;
            }
            // push regular character
            TTY_con.buffer[TTY_con.cursor as usize] = key; // next char will always be '\0'
            TTY_con.cursor += 1;
            // print the current line (this is differential)
            size = crate::stdlib::write(
                1 as libc::c_int,
                &mut key as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as crate::stddef_h::size_t,
            ) as crate::stddef_h::size_t
        } // stdin
        return 0 as *mut libc::c_char;
    } else {
        if stdin_active as u64 != 0 {
            let mut len: libc::c_int = 0;
            let mut fdset: crate::stdlib::fd_set = crate::stdlib::fd_set {
                __fds_bits: [0; 16],
            };
            let mut timeout: ::libc::timeval = ::libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            let mut __d0: libc::c_int = 0;
            let mut __d1: libc::c_int = 0;
            let fresh0 = &mut __d0;
            let fresh1;
            let fresh2 = &mut __d1;
            let fresh3;
            let fresh4 = (::std::mem::size_of::<crate::stdlib::fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong);
            let fresh5 = &mut *fdset
                .__fds_bits
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize)
                as *mut crate::stdlib::__fd_mask;
            asm!("cld; rep; stosq" : "={cx}" (fresh1), "={di}" (fresh3) : "{ax}"
     (0 as libc::c_int), "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh4)), "1"
     (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh5)) : "memory" :
     "volatile");
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh4, fresh1);
            c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh5, fresh3);
            fdset.__fds_bits[(0 as libc::c_int
                / (8 as libc::c_int
                    * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize] |= ((1 as libc::c_ulong)
                << 0 as libc::c_int
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong
                            as libc::c_int))
                as crate::stdlib::__fd_mask;
            timeout.tv_sec = 0 as libc::c_int as crate::stdlib::__time_t;
            timeout.tv_usec = 0 as libc::c_int as crate::stdlib::__suseconds_t;
            if crate::stdlib::select(
                0 as libc::c_int + 1 as libc::c_int,
                &mut fdset,
                0 as *mut crate::stdlib::fd_set,
                0 as *mut crate::stdlib::fd_set,
                &mut timeout,
            ) == -(1 as libc::c_int)
                || !(fdset.__fds_bits[(0 as libc::c_int
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << 0 as libc::c_int
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<crate::stdlib::__fd_mask>() as libc::c_ulong
                                    as libc::c_int))
                        as crate::stdlib::__fd_mask
                    != 0 as libc::c_int as libc::c_long)
            {
                return 0 as *mut libc::c_char;
            }
            len = crate::stdlib::read(
                0 as libc::c_int,
                text.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            ) as libc::c_int;
            if len == 0 as libc::c_int {
                // eof!
                stdin_active = crate::src::qcommon::q_shared::qfalse; // rip off the /n and terminate
                return 0 as *mut libc::c_char;
            }
            if len < 1 as libc::c_int {
                return 0 as *mut libc::c_char;
            }
            text[(len - 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
            return text.as_mut_ptr();
        }
    }
    return 0 as *mut libc::c_char;
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// Require a minimum version of SDL
// Console
/*
==================
CON_Print
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_Print(mut msg: *const libc::c_char) {
    if *msg.offset(0 as libc::c_int as isize) == 0 {
        return;
    }
    CON_Hide();
    if !crate::src::qcommon::common::com_ansiColor.is_null()
        && (*crate::src::qcommon::common::com_ansiColor).integer != 0
    {
        crate::src::sys::sys_main::Sys_AnsiColorPrint(msg);
    } else {
        crate::stdlib::fputs(msg, crate::stdlib::stderr);
    }
    if ttycon_on as u64 == 0 {
        // CON_Hide didn't do anything.
        return;
    }
    // Only print prompt when msg ends with a newline, otherwise the console
    //   might get garbled when output does not fit on one line.
    if *msg
        .offset(crate::stdlib::strlen(msg).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int
        == '\n' as i32
    {
        CON_Show();
        // Run CON_Show the number of times it was deferred.
        while ttycon_show_overdue > 0 as libc::c_int {
            CON_Show();
            ttycon_show_overdue -= 1
        }
    } else {
        // Defer calling CON_Show
        ttycon_show_overdue += 1
    };
}
