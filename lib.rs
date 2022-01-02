use libc::{ c_int, c_char };
use std::ffi::CString;

#[link(name = "libvim")]
#[allow(non_snake_case)]
extern {
    fn vimInit(argc: c_int, argv: *const *const c_char);
}

pub fn vim_init() {
    // create a vector of zero terminated strings
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    unsafe { vimInit(c_args.len() as c_int, c_args.as_ptr()); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vim_init_test() {
        vim_init();
//vimInit();
//
//vimCommand("e ./aBigFile.txt");
//
//vimInput("G");
//
//print_endline ("Cursor is at line: " ++ string_of_int(Cursor.getLine()));
//
//vimInput("I");
//vimInput("a");
        assert_eq!(2 + 2, 4);
    }
}
