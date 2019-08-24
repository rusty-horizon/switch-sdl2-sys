extern crate cfg_if;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "bindgen")] {
        extern crate bindgen;
        use bindgen::callbacks::{ EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks };
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        #[derive(Debug)]
        struct CustomCallbacks;

        impl ParseCallbacks for CustomCallbacks {
            fn will_parse_macro(&self, _name: &str) -> MacroParsingBehavior {
                MacroParsingBehavior::Default
            }

            fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> {
                if _name.starts_with("POLL") && _value < i16::max_value() as i64 && _value > i16::min_value() as i64 {
                    Some(IntKind::I16)
                }
                else if _name.starts_with("DT_") && _value > 0 && _value < u8::max_value() as i64 {
                    Some(IntKind::U8)
                }
                else if _name.starts_with("S_IF") && _value > 0 && _value < u32::max_value() as i64 {
                    Some(IntKind::U32)
                }
                else if _value < i32::max_value() as i64 && _value > i32::min_value() as i64 {
                    Some(IntKind::I32)
                }
                else {
                    None
                }
            }

            fn enum_variant_behavior(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue,) -> Option<EnumVariantCustomBehavior> {
                None
            }

            fn enum_variant_name(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue,) -> Option<String> {
                None
            }
        }

        pub fn regen_bindings(input: &str, output: &str, whitelist: Option<Vec<String>>) -> Result<bindgen::Bindings, std::io::Error> {
            // we don't care if deletion succeeds, as long as the file is gone
            let _ = std::fs::remove_file(output);
            assert!(!std::path::Path::new(output).exists());

            let iswin = cfg!(windows);
            let ilibnx = if iswin {
                "-Ic:\\devkitpro\\libnx\\include"
            } else {
                "-I/opt/devkitpro/libnx/include"
            };
            let iportlibs = if iswin {
                "-Ic:\\devkitpro\\portlibs\\switch\\include"
            } else {
                "-I/opt/devkitpro/portlibs/switch/include"
            };
            let igcc1 = if iswin {
                "-Ic:\\devkitpro\\devkitA64\\aarch64-none-elf\\include"
            } else {
                "-I/opt/devkitpro/devkitA64/aarch64-none-elf/include"
            };
            let igcc2 = if iswin {
                "-Ic:\\devkitpro\\devkitA64\\lib\\gcc\\aarch64-none-elf\\8.3.0\\include"
            } else {
                "-I/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/8.3.0/include/"
            };
            let mut builder = bindgen::Builder::default().clang_arg("-mcrc").trust_clang_mangling(false).use_core().rust_target(bindgen::RustTarget::Nightly).ctypes_prefix("ctypes").generate_inline_functions(true).parse_callbacks(Box::new(CustomCallbacks{})).header(input).clang_arg(ilibnx).clang_arg(iportlibs).clang_arg(igcc1).clang_arg(igcc2).clang_arg("-nostdinc").clang_arg("-U__linux__").blacklist_type("u8").blacklist_type("u16").blacklist_type("u32").blacklist_type("u64");
            if let Some(whitelist) = whitelist {
                for func in whitelist {
                    builder = builder.whitelist_function(func);
                }
            }

            builder.generate().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not create file!")).and_then(|bnd| {
                let mut file = OpenOptions::new().write(true).create(true).open(output)?;
                file.write_all(br#"mod ctypes {
                    pub type c_void = core::ffi::c_void;
                    pub type c_char = u8;
                    pub type c_int = i32;
                    pub type c_long = i64;
                    pub type c_longlong = i64;
                    pub type c_schar = i8;
                    pub type c_short = i16;
                    pub type c_uchar = u8;
                    pub type c_uint = u32;
                    pub type c_ulong = u64;
                    pub type c_ulonglong = u64;
                    pub type c_ushort = u16;
                    pub type size_t = u64;
                    pub type ssize_t = i64;
                    pub type c_float = f32;
                    pub type c_double = f64;
                }"#)?;
                bnd.write(Box::new(file)).map(|_| bnd)
            })
        }

        pub fn bindgen() {
            let gen_path = "bindgen/sdl2.rs";
            let header_wrapper = "bindgen/sdl2.h";

            regen_bindings(header_wrapper, gen_path, None).expect("Error generating sdl2 bindings!");
        }
    } else {
        pub fn bindgen() {
            if !std::path::Path::new("bindgen/sdl2.rs").exists() {
                panic!("Bindgen disabled but sdl2 bindings missing!");
            }
        }
    }
}

cfg_if! {
    if #[cfg(all(feature = "ttf", feature = "bindgen"))] {
        pub fn ttf_bindgen() {
            regen_bindings("bindgen/sdl2-ttf.h", "bindgen/sdl2-ttf.rs", None).expect("Error generating sdl2-ttf bindings!");
        }
    } else if #[cfg(feature = "ttf")] {
        pub fn ttf_bindgen() {
            if !std::path::Path::new("bindgen/sdl2-ttf.rs").exists() {
                panic!("Bindgen disabled but sdl2-ttf output missing!");
            }
        }
    } else {
        pub fn ttf_bindgen() {}
    }
}

cfg_if! {
    if #[cfg(all(feature = "image", feature = "bindgen"))] {
        pub fn image_bindgen() {
            regen_bindings("bindgen/sdl2-image.h", "bindgen/sdl2-image.rs", None).expect("Error generating sdl2-image bindings!");
        }
    } else if #[cfg(feature = "image")] {
        pub fn image_bindgen() {
            if !std::path::Path::new("bindgen/sdl2-image.rs").exists() {
                panic!("Bindgen disabled but sdl2-image output missing!");
            }
        }
    } else {
        pub fn image_bindgen() {}
    }
}

pub fn main() {
    bindgen();
    ttf_bindgen();
    image_bindgen();
}
