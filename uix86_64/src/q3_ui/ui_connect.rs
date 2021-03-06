use ::libc;

pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
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
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_public_h::uiClientState_t;

pub use crate::keycodes_h::K_ALT;
pub use crate::keycodes_h::K_AUX1;
pub use crate::keycodes_h::K_AUX10;
pub use crate::keycodes_h::K_AUX11;
pub use crate::keycodes_h::K_AUX12;
pub use crate::keycodes_h::K_AUX13;
pub use crate::keycodes_h::K_AUX14;
pub use crate::keycodes_h::K_AUX15;
pub use crate::keycodes_h::K_AUX16;
pub use crate::keycodes_h::K_AUX2;
pub use crate::keycodes_h::K_AUX3;
pub use crate::keycodes_h::K_AUX4;
pub use crate::keycodes_h::K_AUX5;
pub use crate::keycodes_h::K_AUX6;
pub use crate::keycodes_h::K_AUX7;
pub use crate::keycodes_h::K_AUX8;
pub use crate::keycodes_h::K_AUX9;
pub use crate::keycodes_h::K_BACKSPACE;
pub use crate::keycodes_h::K_BREAK;
pub use crate::keycodes_h::K_CAPSLOCK;
pub use crate::keycodes_h::K_COMMAND;
pub use crate::keycodes_h::K_COMPOSE;
pub use crate::keycodes_h::K_CONSOLE;
pub use crate::keycodes_h::K_CTRL;
pub use crate::keycodes_h::K_DEL;
pub use crate::keycodes_h::K_DOWNARROW;
pub use crate::keycodes_h::K_END;
pub use crate::keycodes_h::K_ENTER;
pub use crate::keycodes_h::K_ESCAPE;
pub use crate::keycodes_h::K_EURO;
pub use crate::keycodes_h::K_F1;
pub use crate::keycodes_h::K_F10;
pub use crate::keycodes_h::K_F11;
pub use crate::keycodes_h::K_F12;
pub use crate::keycodes_h::K_F13;
pub use crate::keycodes_h::K_F14;
pub use crate::keycodes_h::K_F15;
pub use crate::keycodes_h::K_F2;
pub use crate::keycodes_h::K_F3;
pub use crate::keycodes_h::K_F4;
pub use crate::keycodes_h::K_F5;
pub use crate::keycodes_h::K_F6;
pub use crate::keycodes_h::K_F7;
pub use crate::keycodes_h::K_F8;
pub use crate::keycodes_h::K_F9;
pub use crate::keycodes_h::K_HELP;
pub use crate::keycodes_h::K_HOME;
pub use crate::keycodes_h::K_INS;
pub use crate::keycodes_h::K_JOY1;
pub use crate::keycodes_h::K_JOY10;
pub use crate::keycodes_h::K_JOY11;
pub use crate::keycodes_h::K_JOY12;
pub use crate::keycodes_h::K_JOY13;
pub use crate::keycodes_h::K_JOY14;
pub use crate::keycodes_h::K_JOY15;
pub use crate::keycodes_h::K_JOY16;
pub use crate::keycodes_h::K_JOY17;
pub use crate::keycodes_h::K_JOY18;
pub use crate::keycodes_h::K_JOY19;
pub use crate::keycodes_h::K_JOY2;
pub use crate::keycodes_h::K_JOY20;
pub use crate::keycodes_h::K_JOY21;
pub use crate::keycodes_h::K_JOY22;
pub use crate::keycodes_h::K_JOY23;
pub use crate::keycodes_h::K_JOY24;
pub use crate::keycodes_h::K_JOY25;
pub use crate::keycodes_h::K_JOY26;
pub use crate::keycodes_h::K_JOY27;
pub use crate::keycodes_h::K_JOY28;
pub use crate::keycodes_h::K_JOY29;
pub use crate::keycodes_h::K_JOY3;
pub use crate::keycodes_h::K_JOY30;
pub use crate::keycodes_h::K_JOY31;
pub use crate::keycodes_h::K_JOY32;
pub use crate::keycodes_h::K_JOY4;
pub use crate::keycodes_h::K_JOY5;
pub use crate::keycodes_h::K_JOY6;
pub use crate::keycodes_h::K_JOY7;
pub use crate::keycodes_h::K_JOY8;
pub use crate::keycodes_h::K_JOY9;
pub use crate::keycodes_h::K_KP_5;
pub use crate::keycodes_h::K_KP_DEL;
pub use crate::keycodes_h::K_KP_DOWNARROW;
pub use crate::keycodes_h::K_KP_END;
pub use crate::keycodes_h::K_KP_ENTER;
pub use crate::keycodes_h::K_KP_EQUALS;
pub use crate::keycodes_h::K_KP_HOME;
pub use crate::keycodes_h::K_KP_INS;
pub use crate::keycodes_h::K_KP_LEFTARROW;
pub use crate::keycodes_h::K_KP_MINUS;
pub use crate::keycodes_h::K_KP_NUMLOCK;
pub use crate::keycodes_h::K_KP_PGDN;
pub use crate::keycodes_h::K_KP_PGUP;
pub use crate::keycodes_h::K_KP_PLUS;
pub use crate::keycodes_h::K_KP_RIGHTARROW;
pub use crate::keycodes_h::K_KP_SLASH;
pub use crate::keycodes_h::K_KP_STAR;
pub use crate::keycodes_h::K_KP_UPARROW;
pub use crate::keycodes_h::K_LEFTARROW;
pub use crate::keycodes_h::K_MENU;
pub use crate::keycodes_h::K_MODE;
pub use crate::keycodes_h::K_MOUSE1;
pub use crate::keycodes_h::K_MOUSE2;
pub use crate::keycodes_h::K_MOUSE3;
pub use crate::keycodes_h::K_MOUSE4;
pub use crate::keycodes_h::K_MOUSE5;
pub use crate::keycodes_h::K_MWHEELDOWN;
pub use crate::keycodes_h::K_MWHEELUP;
pub use crate::keycodes_h::K_PAD0_A;
pub use crate::keycodes_h::K_PAD0_B;
pub use crate::keycodes_h::K_PAD0_BACK;
pub use crate::keycodes_h::K_PAD0_DPAD_DOWN;
pub use crate::keycodes_h::K_PAD0_DPAD_LEFT;
pub use crate::keycodes_h::K_PAD0_DPAD_RIGHT;
pub use crate::keycodes_h::K_PAD0_DPAD_UP;
pub use crate::keycodes_h::K_PAD0_GUIDE;
pub use crate::keycodes_h::K_PAD0_LEFTSHOULDER;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_LEFTTRIGGER;
pub use crate::keycodes_h::K_PAD0_RIGHTSHOULDER;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_RIGHTTRIGGER;
pub use crate::keycodes_h::K_PAD0_START;
pub use crate::keycodes_h::K_PAD0_X;
pub use crate::keycodes_h::K_PAD0_Y;
pub use crate::keycodes_h::K_PAUSE;
pub use crate::keycodes_h::K_PGDN;
pub use crate::keycodes_h::K_PGUP;
pub use crate::keycodes_h::K_POWER;
pub use crate::keycodes_h::K_PRINT;
pub use crate::keycodes_h::K_RIGHTARROW;
pub use crate::keycodes_h::K_SCROLLOCK;
pub use crate::keycodes_h::K_SHIFT;
pub use crate::keycodes_h::K_SPACE;
pub use crate::keycodes_h::K_SUPER;
pub use crate::keycodes_h::K_SYSREQ;
pub use crate::keycodes_h::K_TAB;
pub use crate::keycodes_h::K_UNDO;
pub use crate::keycodes_h::K_UPARROW;
pub use crate::keycodes_h::K_WORLD_0;
pub use crate::keycodes_h::K_WORLD_1;
pub use crate::keycodes_h::K_WORLD_10;
pub use crate::keycodes_h::K_WORLD_11;
pub use crate::keycodes_h::K_WORLD_12;
pub use crate::keycodes_h::K_WORLD_13;
pub use crate::keycodes_h::K_WORLD_14;
pub use crate::keycodes_h::K_WORLD_15;
pub use crate::keycodes_h::K_WORLD_16;
pub use crate::keycodes_h::K_WORLD_17;
pub use crate::keycodes_h::K_WORLD_18;
pub use crate::keycodes_h::K_WORLD_19;
pub use crate::keycodes_h::K_WORLD_2;
pub use crate::keycodes_h::K_WORLD_20;
pub use crate::keycodes_h::K_WORLD_21;
pub use crate::keycodes_h::K_WORLD_22;
pub use crate::keycodes_h::K_WORLD_23;
pub use crate::keycodes_h::K_WORLD_24;
pub use crate::keycodes_h::K_WORLD_25;
pub use crate::keycodes_h::K_WORLD_26;
pub use crate::keycodes_h::K_WORLD_27;
pub use crate::keycodes_h::K_WORLD_28;
pub use crate::keycodes_h::K_WORLD_29;
pub use crate::keycodes_h::K_WORLD_3;
pub use crate::keycodes_h::K_WORLD_30;
pub use crate::keycodes_h::K_WORLD_31;
pub use crate::keycodes_h::K_WORLD_32;
pub use crate::keycodes_h::K_WORLD_33;
pub use crate::keycodes_h::K_WORLD_34;
pub use crate::keycodes_h::K_WORLD_35;
pub use crate::keycodes_h::K_WORLD_36;
pub use crate::keycodes_h::K_WORLD_37;
pub use crate::keycodes_h::K_WORLD_38;
pub use crate::keycodes_h::K_WORLD_39;
pub use crate::keycodes_h::K_WORLD_4;
pub use crate::keycodes_h::K_WORLD_40;
pub use crate::keycodes_h::K_WORLD_41;
pub use crate::keycodes_h::K_WORLD_42;
pub use crate::keycodes_h::K_WORLD_43;
pub use crate::keycodes_h::K_WORLD_44;
pub use crate::keycodes_h::K_WORLD_45;
pub use crate::keycodes_h::K_WORLD_46;
pub use crate::keycodes_h::K_WORLD_47;
pub use crate::keycodes_h::K_WORLD_48;
pub use crate::keycodes_h::K_WORLD_49;
pub use crate::keycodes_h::K_WORLD_5;
pub use crate::keycodes_h::K_WORLD_50;
pub use crate::keycodes_h::K_WORLD_51;
pub use crate::keycodes_h::K_WORLD_52;
pub use crate::keycodes_h::K_WORLD_53;
pub use crate::keycodes_h::K_WORLD_54;
pub use crate::keycodes_h::K_WORLD_55;
pub use crate::keycodes_h::K_WORLD_56;
pub use crate::keycodes_h::K_WORLD_57;
pub use crate::keycodes_h::K_WORLD_58;
pub use crate::keycodes_h::K_WORLD_59;
pub use crate::keycodes_h::K_WORLD_6;
pub use crate::keycodes_h::K_WORLD_60;
pub use crate::keycodes_h::K_WORLD_61;
pub use crate::keycodes_h::K_WORLD_62;
pub use crate::keycodes_h::K_WORLD_63;
pub use crate::keycodes_h::K_WORLD_64;
pub use crate::keycodes_h::K_WORLD_65;
pub use crate::keycodes_h::K_WORLD_66;
pub use crate::keycodes_h::K_WORLD_67;
pub use crate::keycodes_h::K_WORLD_68;
pub use crate::keycodes_h::K_WORLD_69;
pub use crate::keycodes_h::K_WORLD_7;
pub use crate::keycodes_h::K_WORLD_70;
pub use crate::keycodes_h::K_WORLD_71;
pub use crate::keycodes_h::K_WORLD_72;
pub use crate::keycodes_h::K_WORLD_73;
pub use crate::keycodes_h::K_WORLD_74;
pub use crate::keycodes_h::K_WORLD_75;
pub use crate::keycodes_h::K_WORLD_76;
pub use crate::keycodes_h::K_WORLD_77;
pub use crate::keycodes_h::K_WORLD_78;
pub use crate::keycodes_h::K_WORLD_79;
pub use crate::keycodes_h::K_WORLD_8;
pub use crate::keycodes_h::K_WORLD_80;
pub use crate::keycodes_h::K_WORLD_81;
pub use crate::keycodes_h::K_WORLD_82;
pub use crate::keycodes_h::K_WORLD_83;
pub use crate::keycodes_h::K_WORLD_84;
pub use crate::keycodes_h::K_WORLD_85;
pub use crate::keycodes_h::K_WORLD_86;
pub use crate::keycodes_h::K_WORLD_87;
pub use crate::keycodes_h::K_WORLD_88;
pub use crate::keycodes_h::K_WORLD_89;
pub use crate::keycodes_h::K_WORLD_9;
pub use crate::keycodes_h::K_WORLD_90;
pub use crate::keycodes_h::K_WORLD_91;
pub use crate::keycodes_h::K_WORLD_92;
pub use crate::keycodes_h::K_WORLD_93;
pub use crate::keycodes_h::K_WORLD_94;
pub use crate::keycodes_h::K_WORLD_95;
pub use crate::keycodes_h::MAX_KEYS;
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_DrawHandlePic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString_AutoWrapped;
pub use crate::src::q3_ui::ui_atoms::UI_ProportionalSizeScale;
pub use crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth;
pub use crate::src::q3_ui::ui_atoms::UI_SetColor;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::menu_text_color;
pub use crate::src::q3_ui::ui_qmenu::Menu_Cache;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_GetClientState;
pub use crate::src::ui::ui_syscalls::trap_GetConfigString;
use crate::stdlib::strlen;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::mfield_t;
pub use crate::ui_local_h::uiStatic_t;
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
//
/*
===============================================================================

CONNECTION SCREEN

===============================================================================
*/
#[no_mangle]

pub static mut passwordNeeded: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qtrue;
#[no_mangle]

pub static mut passwordField: crate::ui_local_h::menufield_s = crate::ui_local_h::menufield_s {
    generic: crate::ui_local_h::menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        id: 0,
        x: 0,
        y: 0,
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
        parent: 0 as *const crate::ui_local_h::menuframework_s
            as *mut crate::ui_local_h::menuframework_s,
        menuPosition: 0,
        flags: 0,
        callback: None,
        statusbar: None,
        ownerdraw: None,
    },
    field: crate::ui_local_h::mfield_t {
        cursor: 0,
        scroll: 0,
        widthInChars: 0,
        buffer: [0; 256],
        maxchars: 0,
    },
};

static mut lastConnState: crate::src::qcommon::q_shared::connstate_t =
    crate::src::qcommon::q_shared::CA_UNINITIALIZED;

static mut lastLoadingText: [libc::c_char; 1024] = [0; 1024];

unsafe extern "C" fn UI_ReadableSize(
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut value: libc::c_int,
) {
    if value > 1024 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int {
        // gigs
        crate::src::qcommon::q_shared::Com_sprintf(
            buf,
            bufsize,
            b"%d\x00" as *const u8 as *const libc::c_char,
            value / (1024 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int),
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            buf.offset(crate::stdlib::strlen(buf) as isize),
            (bufsize as libc::c_ulong).wrapping_sub(crate::stdlib::strlen(buf)) as libc::c_int,
            b".%02d GB\x00" as *const u8 as *const libc::c_char,
            value % (1024 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                * 100 as libc::c_int
                / (1024 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int),
        );
    } else if value > 1024 as libc::c_int * 1024 as libc::c_int {
        // megs
        crate::src::qcommon::q_shared::Com_sprintf(
            buf,
            bufsize,
            b"%d\x00" as *const u8 as *const libc::c_char,
            value / (1024 as libc::c_int * 1024 as libc::c_int),
        ); // bytes
        crate::src::qcommon::q_shared::Com_sprintf(
            buf.offset(crate::stdlib::strlen(buf) as isize),
            (bufsize as libc::c_ulong).wrapping_sub(crate::stdlib::strlen(buf)) as libc::c_int,
            b".%02d MB\x00" as *const u8 as *const libc::c_char,
            value % (1024 as libc::c_int * 1024 as libc::c_int) * 100 as libc::c_int
                / (1024 as libc::c_int * 1024 as libc::c_int),
        );
    } else if value > 1024 as libc::c_int {
        // kilos
        crate::src::qcommon::q_shared::Com_sprintf(
            buf,
            bufsize,
            b"%d KB\x00" as *const u8 as *const libc::c_char,
            value / 1024 as libc::c_int,
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            buf,
            bufsize,
            b"%d bytes\x00" as *const u8 as *const libc::c_char,
            value,
        );
    };
}
// Assumes time is in msec

unsafe extern "C" fn UI_PrintTime(
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut time: libc::c_int,
) {
    time /= 1000 as libc::c_int; // change to seconds
    if time > 3600 as libc::c_int {
        // in the hours range
        crate::src::qcommon::q_shared::Com_sprintf(
            buf,
            bufsize,
            b"%d hr %d min\x00" as *const u8 as *const libc::c_char,
            time / 3600 as libc::c_int,
            time % 3600 as libc::c_int / 60 as libc::c_int,
        ); // secs
    } else if time > 60 as libc::c_int {
        // mins
        crate::src::qcommon::q_shared::Com_sprintf(
            buf,
            bufsize,
            b"%d min %d sec\x00" as *const u8 as *const libc::c_char,
            time / 60 as libc::c_int,
            time % 60 as libc::c_int,
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            buf,
            bufsize,
            b"%d sec\x00" as *const u8 as *const libc::c_char,
            time,
        );
    };
}

unsafe extern "C" fn UI_DisplayDownloadInfo(mut downloadName: *const libc::c_char) {
    static mut dlText: [libc::c_char; 13] =
        [68, 111, 119, 110, 108, 111, 97, 100, 105, 110, 103, 58, 0];
    static mut etaText: [libc::c_char; 21] = [
        69, 115, 116, 105, 109, 97, 116, 101, 100, 32, 116, 105, 109, 101, 32, 108, 101, 102, 116,
        58, 0,
    ];
    static mut xferText: [libc::c_char; 15] = [
        84, 114, 97, 110, 115, 102, 101, 114, 32, 114, 97, 116, 101, 58, 0,
    ];
    let mut downloadSize: libc::c_int = 0;
    let mut downloadCount: libc::c_int = 0;
    let mut downloadTime: libc::c_int = 0;
    let mut dlSizeBuf: [libc::c_char; 64] = [0; 64];
    let mut totalSizeBuf: [libc::c_char; 64] = [0; 64];
    let mut xferRateBuf: [libc::c_char; 64] = [0; 64];
    let mut dlTimeBuf: [libc::c_char; 64] = [0; 64];
    let mut xferRate: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut leftWidth: libc::c_int = 0;
    let mut style: libc::c_int = 0 as libc::c_int | 0x10 as libc::c_int | 0x800 as libc::c_int;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    downloadSize = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cl_downloadSize\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    downloadCount = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    downloadTime = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cl_downloadTime\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    leftWidth = (crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth(dlText.as_mut_ptr())
        as libc::c_float
        * crate::src::q3_ui::ui_atoms::UI_ProportionalSizeScale(style))
        as libc::c_int;
    width = (crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth(etaText.as_mut_ptr())
        as libc::c_float
        * crate::src::q3_ui::ui_atoms::UI_ProportionalSizeScale(style)) as libc::c_int;
    if width > leftWidth {
        leftWidth = width
    }
    width = (crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth(xferText.as_mut_ptr())
        as libc::c_float
        * crate::src::q3_ui::ui_atoms::UI_ProportionalSizeScale(style)) as libc::c_int;
    if width > leftWidth {
        leftWidth = width
    }
    leftWidth += 16 as libc::c_int;
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        8 as libc::c_int,
        128 as libc::c_int,
        dlText.as_mut_ptr(),
        style,
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        8 as libc::c_int,
        160 as libc::c_int,
        etaText.as_mut_ptr(),
        style,
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        8 as libc::c_int,
        224 as libc::c_int,
        xferText.as_mut_ptr(),
        style,
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
    );
    if downloadSize > 0 as libc::c_int {
        s = crate::src::qcommon::q_shared::va(
            b"%s (%d%%)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            downloadName,
            (downloadCount as libc::c_float * 100.0f32 / downloadSize as libc::c_float)
                as libc::c_int,
        )
    } else {
        s = downloadName
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        leftWidth,
        128 as libc::c_int,
        s,
        style,
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
    );
    UI_ReadableSize(
        dlSizeBuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        downloadCount,
    );
    UI_ReadableSize(
        totalSizeBuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        downloadSize,
    );
    if downloadCount < 4096 as libc::c_int || downloadTime == 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            leftWidth,
            160 as libc::c_int,
            b"estimating\x00" as *const u8 as *const libc::c_char,
            style,
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            leftWidth,
            192 as libc::c_int,
            crate::src::qcommon::q_shared::va(
                b"(%s of %s copied)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dlSizeBuf.as_mut_ptr(),
                totalSizeBuf.as_mut_ptr(),
            ),
            style,
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
    } else {
        if (crate::src::q3_ui::ui_atoms::uis.realtime - downloadTime) / 1000 as libc::c_int != 0 {
            xferRate = downloadCount
                / ((crate::src::q3_ui::ui_atoms::uis.realtime - downloadTime) / 1000 as libc::c_int)
        //xferRate = (int)( ((float)downloadCount) / elapsedTime);
        } else {
            xferRate = 0 as libc::c_int
        }
        UI_ReadableSize(
            xferRateBuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            xferRate,
        );
        // Extrapolate estimated completion time
        if downloadSize != 0 && xferRate != 0 {
            let mut n: libc::c_int = downloadSize / xferRate; // estimated time for entire d/l in secs
                                                              // We do it in K (/1024) because we'd overflow around 4MB
            n = (n - downloadCount / 1024 as libc::c_int * n
                / (downloadSize / 1024 as libc::c_int))
                * 1000 as libc::c_int;
            UI_PrintTime(
                dlTimeBuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                n,
            );
            //(n - (((downloadCount/1024) * n) / (downloadSize/1024))) * 1000);
            crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
                leftWidth,
                160 as libc::c_int,
                dlTimeBuf.as_mut_ptr(),
                style,
                crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
            );
            crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
                leftWidth,
                192 as libc::c_int,
                crate::src::qcommon::q_shared::va(
                    b"(%s of %s copied)\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    dlSizeBuf.as_mut_ptr(),
                    totalSizeBuf.as_mut_ptr(),
                ),
                style,
                crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
            );
        } else {
            crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
                leftWidth,
                160 as libc::c_int,
                b"estimating\x00" as *const u8 as *const libc::c_char,
                style,
                crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
            );
            if downloadSize != 0 {
                crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
                    leftWidth,
                    192 as libc::c_int,
                    crate::src::qcommon::q_shared::va(
                        b"(%s of %s copied)\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        dlSizeBuf.as_mut_ptr(),
                        totalSizeBuf.as_mut_ptr(),
                    ),
                    style,
                    crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
                );
            } else {
                crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
                    leftWidth,
                    192 as libc::c_int,
                    crate::src::qcommon::q_shared::va(
                        b"(%s copied)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        dlSizeBuf.as_mut_ptr(),
                    ),
                    style,
                    crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
                );
            }
        }
        if xferRate != 0 {
            crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
                leftWidth,
                224 as libc::c_int,
                crate::src::qcommon::q_shared::va(
                    b"%s/Sec\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    xferRateBuf.as_mut_ptr(),
                ),
                style,
                crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
            );
        }
    };
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
//
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
//
// ui_connect.c
//
/*
========================
UI_DrawConnectScreen

This will also be overlaid on the cgame info screen during loading
to prevent it from blinking away too rapidly on local or lan games.
========================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawConnectScreen(
    mut overlay: crate::src::qcommon::q_shared::qboolean,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cstate: crate::ui_public_h::uiClientState_t = crate::ui_public_h::uiClientState_t {
        connState: crate::src::qcommon::q_shared::CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut info: [libc::c_char; 1024] = [0; 1024];
    crate::src::q3_ui::ui_qmenu::Menu_Cache();
    if overlay as u64 == 0 {
        // draw the dialog background
        crate::src::q3_ui::ui_atoms::UI_SetColor(
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            640 as libc::c_int as libc::c_float,
            480 as libc::c_int as libc::c_float,
            crate::src::q3_ui::ui_atoms::uis.menuBackShader,
        );
    }
    // see what information we should display
    crate::src::ui::ui_syscalls::trap_GetClientState(
        &mut cstate as *mut _ as *mut crate::ui_public_h::uiClientState_t,
    );
    info[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if crate::src::ui::ui_syscalls::trap_GetConfigString(
        0 as libc::c_int,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    ) != 0
    {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            320 as libc::c_int,
            16 as libc::c_int,
            crate::src::qcommon::q_shared::va(
                b"Loading %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    info.as_mut_ptr(),
                    b"mapname\x00" as *const u8 as *const libc::c_char,
                ),
            ),
            0x20 as libc::c_int | 0x1 as libc::c_int | 0x800 as libc::c_int,
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320 as libc::c_int,
        64 as libc::c_int,
        crate::src::qcommon::q_shared::va(
            b"Connecting to %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cstate.servername.as_mut_ptr(),
        ),
        0x1 as libc::c_int | 0x10 as libc::c_int | 0x800 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::menu_text_color.as_mut_ptr(),
    );
    //UI_DrawProportionalString( 320, 96, "Press Esc to abort", UI_CENTER|UI_SMALLFONT|UI_DROPSHADOW, menu_text_color );
    // display global MOTD at bottom
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 as libc::c_int / 2 as libc::c_int,
        480 as libc::c_int - 32 as libc::c_int,
        crate::src::qcommon::q_shared::Info_ValueForKey(
            cstate.updateInfoString.as_mut_ptr(),
            b"motd\x00" as *const u8 as *const libc::c_char,
        ),
        0x1 as libc::c_int | 0x10 as libc::c_int | 0x800 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::menu_text_color.as_mut_ptr(),
    );
    // print any server info (server full, bad version, etc)
    if (cstate.connState as libc::c_uint)
        < crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
    {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString_AutoWrapped(
            320 as libc::c_int,
            192 as libc::c_int,
            630 as libc::c_int,
            20 as libc::c_int,
            cstate.messageString.as_mut_ptr(),
            0x1 as libc::c_int | 0x10 as libc::c_int | 0x800 as libc::c_int,
            crate::src::q3_ui::ui_qmenu::menu_text_color.as_mut_ptr(),
        );
    }
    if lastConnState as libc::c_uint > cstate.connState as libc::c_uint {
        lastLoadingText[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    lastConnState = cstate.connState;
    match cstate.connState as libc::c_uint {
        3 => {
            s = crate::src::qcommon::q_shared::va(
                b"Awaiting challenge...%i\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cstate.connectPacketCount,
            )
        }
        4 => {
            s = crate::src::qcommon::q_shared::va(
                b"Awaiting connection...%i\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cstate.connectPacketCount,
            )
        }
        5 => {
            let mut downloadName: [libc::c_char; 1024] = [0; 1024];
            crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
                b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
                downloadName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            if *downloadName.as_mut_ptr() != 0 {
                UI_DisplayDownloadInfo(downloadName.as_mut_ptr());
                return;
            }
            s = b"Awaiting gamestate...\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char
        }
        6 => return,
        7 => return,
        _ => return,
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320 as libc::c_int,
        128 as libc::c_int,
        s,
        0x1 as libc::c_int | 0x10 as libc::c_int | 0x800 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
    );
    // password required / connection rejected information goes here
}
/*
===================
UI_KeyConnect
===================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_KeyConnect(mut key: libc::c_int) {
    if key == crate::keycodes_h::K_ESCAPE as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
            b"disconnect\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    };
}
