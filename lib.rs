use libc::{ c_int, c_char, c_uchar };
use std::{ os::raw::c_long, ffi::CString };

include!("./bindings.rs");

#[derive(PartialEq, Debug)]
pub enum VimMode {
    NORMAL,
    VISUAL,
    INSERT,
    OP_PENDING,
    UNKNOWN
}

pub fn vim_get_mode() -> VimMode {
    let result = unsafe { vimGetMode() };
    match result {
        1 | 257 => VimMode::NORMAL,
        16 => VimMode::INSERT,
        2 => VimMode::VISUAL,
        4 => VimMode::OP_PENDING,
        _ => VimMode::UNKNOWN
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

pub fn vim_new_buffer() {
    let buffer = unsafe { vimBufferNew(1 as c_int); };
    dbg!(buffer);
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
        vim_set_window_size((50, 100));
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
        vim_buffer_open("./test/futurama-quotes.txt");

        vim_execute("e!");
        vim_key("<esc>");
        vim_key("<esc>");
        vim_input("g");
        vim_input("g");
        assert_eq!(vim_cursor_get_line(), 1);

        vim_input("G");
        assert_eq!(vim_cursor_get_line(), 44);
            //assert_eq!(vim_cursor_get_line() > 1, true);

        vim_input("g");
        vim_input("g");
        assert_eq!(vim_cursor_get_line(), 1);
        vim_input("qall!");
        assert_eq!("a", "b");
        //assert_eq!(vim_cursor_get_line(), 1);
        teardown();
    }

    fn navigation_and_file_creation_test() {
        let filename = "./test/file_test";
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        vim_init();
        assert_eq!(vim_get_mode(), VimMode::NORMAL);
        vim_execute(format!("e {}", filename).as_str());
        // vim_execute(format!("w {}", filename).as_str());
        //vim_new_buffer();
        vim_input("i");
        assert_eq!(vim_get_mode(), VimMode::INSERT);
        vim_input("blastoise");
        vim_key("<ESC>");
        vim_key("<ESC>");
        assert_eq!(vim_get_mode(), VimMode::NORMAL);
        vim_execute("w");
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "blastoise\n");
        vim_key("<ESC>");
        assert_eq!(vim_get_mode(), VimMode::NORMAL);
        vim_input("V");
        assert_eq!(vim_get_mode(), VimMode::VISUAL);
        vim_input("d");
        vim_execute("w");
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "");
        vim_input("i");
        assert_eq!(vim_get_mode(), VimMode::INSERT);
        vim_input("test");
        vim_execute("w");
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "test\n");
        fs::remove_file(filename).expect("File deleted");

        teardown();
    }
}
