#![allow(unused_parens)]
pub mod controls;
pub mod movement;
pub mod mouse;
pub mod audio;
pub mod scene_loader;
pub mod node_ext;

#[macro_export]
macro_rules! gd_unimplemented { 
    () => ({
        let line = std::line!();
        let file = std::file!();
        gdnative::godot_print!("{}:{} Rust: unimplemented!", file, line);
        std::unimplemented!()
    });
}

#[macro_export]
macro_rules! gd_err {
    ($($arg:tt)*) => ({
        let line = std::line!();
        let file = std::file!();
        let val: String = format!($($arg)*);
        gdnative::godot_error!("{}:{} {}", file, line, val);
    });
}


#[macro_export]
macro_rules! gd_panic {
    ($($arg:tt)*) => ({
        $crate::gd_err!($($arg)*);
        panic!($($arg)*);
    });
}

#[macro_export]
macro_rules! some_or_bail {
    ($opt:expr, $($arg:tt)*) => ({
        match $opt {
            Some(val) => val,
            None => {
                let line = std::line!();
                let file = std::file!();
                let val: String = format!($($arg)*);
                gdnative::godot_error!("{}:{} {}", file, line, val);
                return
            }
        }
    });
}
