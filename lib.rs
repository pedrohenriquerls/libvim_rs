use libc::{ c_int, c_char, c_uchar };
use std::ffi::CString;

#[link(name="vim", kind="static")]
#[allow(non_snake_case)]
extern {
    fn vimInit(argc: c_int, argv: *const *const c_char);
    fn vimExecute(cmd: *const c_uchar);
    fn vimExecuteLines(lines: *const *const c_uchar, lineCount: c_int);
    fn vimInput(input: *const c_uchar);
    fn vimKey(key: *const c_uchar);
    fn vimGetMode() -> c_int;
}

#[derive(PartialEq, Debug)]
pub enum VimMode {
    NORMAL,
    VISUAL,
    INSERT,
    UNKNOWN
}

pub fn vim_get_mode() -> VimMode {
    let result = unsafe { vimGetMode() };
    match result {
        1 => VimMode::NORMAL,
        16 => VimMode::INSERT,
        2 => VimMode::VISUAL,
        _ => VimMode::UNKNOWN
    }
}

pub fn vim_init() {
    // create a vector of zero terminated strings
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    unsafe { vimInit(c_args.len() as c_int, c_args.as_ptr()); }
}

pub fn vim_execute_lines(cmds: Vec<String>) {
    let args = cmds.into_iter().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    let c_args = args.iter().map(|arg| arg.as_bytes().as_ptr()).collect::<Vec<*const c_uchar>>();
    unsafe { vimExecuteLines(c_args.as_ptr(), c_args.len() as c_int); }
}

pub fn vim_execute(cmd: String) {
    unsafe { vimExecute(CString::new(cmd).unwrap().as_bytes().as_ptr()); }
}

pub fn vim_input(cmd: String) {
    unsafe { vimInput(CString::new(cmd).unwrap().as_bytes().as_ptr()); }
}

pub fn vim_key(cmd: String) {
    unsafe { vimKey(CString::new(cmd).unwrap().as_bytes().as_ptr()); }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use super::*;

    #[test]
    fn vim_init_test() {
        let filename = "./test/file_test";
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        vim_init();
        assert_eq!(vim_get_mode(), VimMode::NORMAL);
        vim_execute(format!("e {}", filename));
        vim_input(String::from("i"));
        assert_eq!(vim_get_mode(), VimMode::INSERT);
        vim_input(String::from("blastoise"));
        vim_execute(String::from("w"));
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "blastoise\n");
        vim_key(String::from( "<ESC>" ));
        assert_eq!(vim_get_mode(), VimMode::NORMAL);
        vim_input(String::from("V"));
        assert_eq!(vim_get_mode(), VimMode::VISUAL);
        vim_input(String::from("d"));
        vim_execute(String::from("w"));
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "");
        vim_input(String::from("i"));
        assert_eq!(vim_get_mode(), VimMode::INSERT);
        vim_input(String::from("test"));
        vim_execute(String::from("w"));
        assert_eq!(fs::read_to_string(filename).expect("Read file"), "test\n");
        fs::remove_file(filename).expect("File deleted");
    }
}
