use libc::{ c_int, c_char, c_uchar, c_ulong };
use std::{ os::raw::c_long, ffi::CString, ffi::CStr };

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
include!("./bindings.rs");

pub type VimBuffer = file_buffer;
pub type CursorPosition = pos_T;

#[derive(PartialEq, Debug)]
pub enum VimMode {
    Normal,
    Visual,
    Insert,
    OpPending,
    Unknown
}

pub fn vim_get_mode() -> VimMode {
    let result = unsafe { vimGetMode() };
    match result {
        1 | 257 => VimMode::Normal,
        16 => VimMode::Insert,
        2 => VimMode::Visual,
        4 => VimMode::OpPending,
        _ => VimMode::Unknown
    }
}

pub fn vim_init() {
    // create a vector of zero terminated strings
    let mut args: Vec<*mut c_char> = std::env::args().into_iter().map(|arg| CString::new(arg).unwrap().into_raw() ).collect();
    unsafe { vimInit(args.len() as c_int, args.as_mut_ptr()); }
}

pub fn vim_execute_lines(cmds: &mut Vec<&str>) {
    let mut args: Vec<*mut c_uchar> = cmds.into_iter().map(|&mut arg|
        CString::new(arg).unwrap().into_raw() as *mut u8
    ).collect();
    unsafe { vimExecuteLines(args.as_mut_ptr(), args.len() as c_int); }
}

pub fn vim_execute(cmd: &str) {
    unsafe { vimExecute(CString::new(cmd).unwrap().into_raw() as *mut u8); }
}

pub fn vim_input(cmd: &str) {
    unsafe { vimInput(CString::new(cmd).unwrap().into_raw() as *mut u8); }
}

pub fn vim_key(cmd: &str) {
    unsafe { vimKey(CString::new(cmd).unwrap().into_raw() as *mut u8); }
}

pub fn vim_buffer_get_line(buffer: &mut file_buffer, line_number: c_long) -> Option<&str> {
    unsafe { 
        let result = vimBufferGetLine(buffer as *mut file_buffer, line_number);
        if result.is_null() {
            None
        } else {
            match CStr::from_ptr(result as *const c_char).to_str() {
                Ok(val) => Some(val),
                Err(error) => {
                    panic!("Failed to read buffer lines {}", error);
                }
            }
        }
    }
}

pub fn vim_buffer_line_count(buffer: &mut file_buffer) -> c_ulong {
    unsafe {
        match vimBufferGetLineCount(buffer as *mut file_buffer).try_into() {
            Ok(output) => output,
            Err(_) => 0
        }
    }
}

pub fn vim_buffer_get_id(buffer: &mut file_buffer) -> c_int {
    unsafe { vimBufferGetId(buffer as *mut file_buffer) }
}

pub fn vim_load_buffer(file_path: &str) -> Option<&mut file_buffer> {
    let file_path_c_string = CString::new(file_path).unwrap().into_raw() as *mut u8;
    unsafe { 
        let result = vimBufferLoad(file_path_c_string, 1, 0);
        if result.is_null() {
            None
        } else {
            Some(&mut *result)
        }
    }
}

pub fn vim_new_buffer<'a>() -> Option<&'a mut file_buffer> {
    unsafe {
        let result = vimBufferNew(1);
        if result.is_null() {
            None
        } else {
            Some(&mut *result)
        }
    }
}

pub fn vim_buffer_open(file_path: &str) -> Option<&mut file_buffer> {
    let file_path_c_string = CString::new(file_path).unwrap().into_raw() as *mut u8;
    unsafe { 
        let result = vimBufferOpen(file_path_c_string, 1, 0);
        if result.is_null() {
            None
        } else {
            Some(&mut *result)
        }
    }
}

pub fn vim_cursor_get_line() -> c_long {
    unsafe { vimCursorGetLine() }
}

pub fn vim_cursor_get_position() -> pos_T {
    unsafe { vimCursorGetPosition() }
}

pub fn vim_set_window_size(size: (c_int, c_int)) {
    let (width, height) = size;
    unsafe {
        vimWindowSetHeight(height);
        vimWindowSetWidth(width);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use super::*;

    fn setup() {
        vim_init();
        vim_set_window_size((1024, 768));
    }

    fn teardown() {
        vim_execute("qall!");
    }

    #[test]
    fn suite_test() {
        setup();
        navigation_and_file_creation_test();
        navigation_G_gg_test();
    }

    fn navigation_G_gg_test() {
        let buffer = vim_buffer_open("./test/futurama-quotes.txt").expect("Read file as buffer");
        let quote = "    Leela: Oh, I'm sorry. Now I'll axe you again. Where is the mi-cro-wave?";
        assert_eq!(vim_buffer_get_line(buffer, 14).expect("Read the line 14"), quote);
        assert_eq!(vim_buffer_line_count(buffer), 44);
        assert_eq!(vim_buffer_get_id(buffer), 3);

        vim_execute("e!");
        vim_key("<esc>");
        vim_key("<esc>");
        vim_input("g");
        vim_input("g");
        assert_eq!(vim_cursor_get_line(), 1);

        vim_input("G");
        assert_eq!(vim_cursor_get_line(), 44);
        vim_input("$");
        let cursor = vim_cursor_get_position();
        assert_eq!(cursor.lnum, 44);
        assert_eq!(cursor.col, 29);

        vim_input("g");
        vim_input("g");
        assert_eq!(vim_cursor_get_line(), 1);
        teardown();
    }

    fn navigation_and_file_creation_test() {
        let filename = "./test/file_test";
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        vim_init();
        assert_eq!(vim_get_mode(), VimMode::Normal);
        vim_execute(format!("e {}", filename).as_str());
        // vim_execute(format!("w {}", filename).as_str());
        //vim_new_buffer();
        vim_input("i");
        assert_eq!(vim_get_mode(), VimMode::Insert);
        vim_input("blastoise");
        vim_key("<ESC>");
        vim_key("<ESC>");
        assert_eq!(vim_get_mode(), VimMode::Normal);
        vim_execute("w");
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "blastoise\n");
        vim_key("<ESC>");
        assert_eq!(vim_get_mode(), VimMode::Normal);
        vim_input("V");
        assert_eq!(vim_get_mode(), VimMode::Visual);
        vim_input("d");
        vim_execute("w");
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "");
        vim_input("i");
        assert_eq!(vim_get_mode(), VimMode::Insert);
        vim_input("test");
        vim_execute("w");
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "test\n");
        fs::remove_file(filename).expect("File deleted");

        teardown();
    }
}
