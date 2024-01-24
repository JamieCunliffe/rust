// assembly-output: emit-asm
// compile-flags: --target aarch64-unknown-linux-gnu -C target-feature=+sve
// needs-llvm-components: aarch64

#![feature(no_core, lang_items, rustc_attrs, repr_simd, repr_scalable, unsized_locals, unsized_fn_params)]
#![crate_type = "rlib"]
#![no_core]
#![allow(asm_sub_register, non_camel_case_types)]

#[rustc_builtin_macro]
macro_rules! asm {
    () => {};
}
#[rustc_builtin_macro]
macro_rules! concat {
    () => {};
}
#[rustc_builtin_macro]
macro_rules! stringify {
    () => {};
}

#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}

type ptr = *mut u8;

#[repr(simd)]
pub struct i8x8(i8, i8, i8, i8, i8, i8, i8, i8);
#[repr(simd)]
pub struct i16x4(i16, i16, i16, i16);
#[repr(simd)]
pub struct i32x2(i32, i32);
#[repr(simd)]
pub struct i64x1(i64);
#[repr(simd)]
pub struct f32x2(f32, f32);
#[repr(simd)]
pub struct f64x1(f64);
#[repr(simd)]
pub struct i8x16(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8);
#[repr(simd)]
pub struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);
#[repr(simd)]
pub struct i32x4(i32, i32, i32, i32);
#[repr(simd)]
pub struct i64x2(i64, i64);
#[repr(simd)]
pub struct f32x4(f32, f32, f32, f32);
#[repr(simd)]
pub struct f64x2(f64, f64);

impl Copy for i8 {}
impl Copy for i16 {}
impl Copy for i32 {}
impl Copy for f32 {}
impl Copy for i64 {}
impl Copy for f64 {}
impl Copy for ptr {}
impl Copy for i8x8 {}
impl Copy for i16x4 {}
impl Copy for i32x2 {}
impl Copy for i64x1 {}
impl Copy for f32x2 {}
impl Copy for f64x1 {}
impl Copy for i8x16 {}
impl Copy for i16x8 {}
impl Copy for i32x4 {}
impl Copy for i64x2 {}
impl Copy for f32x4 {}
impl Copy for f64x2 {}

macro_rules! impl_sve_type {
    ($(($v:vis, $elem_type:ty, $name:ident, $elt:literal))*) => ($(
        #[repr(simd, scalable($elt))]
        #[allow(non_camel_case_types)]
        $v struct $name {
            _ty: [$elem_type],
        }
    )*)
}

impl_sve_type! {
    (pub, bool, svbool_t, 16)
    (pub, i8, svint8_t, 16)
    (pub, u8, svuint8_t, 16)
    (pub, i16, svint16_t, 8)
    (pub, u16, svuint16_t, 8)
    (pub, f32, svfloat32_t, 4)
    (pub, i32, svint32_t, 4)
    (pub, u32, svuint32_t, 4)
    (pub, f64, svfloat64_t, 2)
    (pub, i64, svint64_t, 2)
    (pub, u64, svuint64_t, 2)
}

extern "C" {
    fn extern_func();
    static extern_static: u8;
}

// CHECK-LABEL: sym_fn:
// CHECK: //APP
// CHECK: bl extern_func
// CHECK: //NO_APP
#[no_mangle]
pub unsafe fn sym_fn() {
    asm!("bl {}", sym extern_func);
}

// CHECK-LABEL: sym_static:
// CHECK: //APP
// CHECK: adr x0, extern_static
// CHECK: //NO_APP
#[no_mangle]
pub unsafe fn sym_static() {
    asm!("adr x0, {}", sym extern_static);
}

// Regression test for #75761
// CHECK-LABEL: issue_75761:
// CHECK: str {{.*}}x30
// CHECK: //APP
// CHECK: //NO_APP
// CHECK: ldr {{.*}}x30
#[no_mangle]
pub unsafe fn issue_75761() {
    asm!("", out("v0") _, out("x30") _);
}

macro_rules! check {
    ($func:ident $ty:ident $class:ident $mov:literal $modifier:literal $($reg_suffix:literal)?) => {
        #[no_mangle]
        #[target_feature(enable = "sve")]
        pub unsafe fn $func(x: $ty) -> $ty {
            // Hack to avoid function merging
            extern "Rust" {
                fn dont_merge(s: &str);
            }
            dont_merge(stringify!($func));

            let y;
            asm!(
                concat!($mov, " {:", $modifier, "}" $(, $reg_suffix)?,
                        ", {:", $modifier, "}" $(, $reg_suffix)?),
                out($class) y,
                in($class) x
            );
            y
        }
    };
}

macro_rules! check_reg {
    ($func:ident $ty:ident $reg:tt $mov:literal $($reg_suffix:literal)?) => {
        #[no_mangle]
        #[target_feature(enable = "sve")]
        pub unsafe fn $func(x: $ty) -> $ty {
            // Hack to avoid function merging
            extern "Rust" {
                fn dont_merge(s: &str);
            }
            dont_merge(stringify!($func));

            let y;
            asm!(
                concat!($mov, " ", $reg $(, $reg_suffix)?, ", ", $reg $(, $reg_suffix)?),
                lateout($reg) y,
                in($reg) x);
            y
        }
    };
}

// CHECK-LABEL: reg_i8:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check!(reg_i8 i8 reg "mov" "");

// CHECK-LABEL: reg_i16:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check!(reg_i16 i16 reg "mov" "");

// CHECK-LABEL: reg_i32:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check!(reg_i32 i32 reg "mov" "");

// CHECK-LABEL: reg_f32:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check!(reg_f32 f32 reg "mov" "");

// CHECK-LABEL: reg_i64:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check!(reg_i64 i64 reg "mov" "");

// CHECK-LABEL: reg_f64:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check!(reg_f64 f64 reg "mov" "");

// CHECK-LABEL: reg_ptr:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check!(reg_ptr ptr reg "mov" "");

// CHECK-LABEL: vreg_i8:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i8 i8 vreg "fmov" "s");

// CHECK-LABEL: vreg_i16:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i16 i16 vreg "fmov" "s");

// CHECK-LABEL: vreg_i32:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i32 i32 vreg "fmov" "s");

// CHECK-LABEL: vreg_f32:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_f32 f32 vreg "fmov" "s");

// CHECK-LABEL: vreg_i64:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i64 i64 vreg "fmov" "s");

// CHECK-LABEL: vreg_f64:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_f64 f64 vreg "fmov" "s");

// CHECK-LABEL: vreg_ptr:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_ptr ptr vreg "fmov" "s");

// CHECK-LABEL: vreg_i8x8:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i8x8 i8x8 vreg "fmov" "s");

// CHECK-LABEL: vreg_i16x4:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i16x4 i16x4 vreg "fmov" "s");

// CHECK-LABEL: vreg_i32x2:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i32x2 i32x2 vreg "fmov" "s");

// CHECK-LABEL: vreg_i64x1:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i64x1 i64x1 vreg "fmov" "s");

// CHECK-LABEL: vreg_f32x2:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_f32x2 f32x2 vreg "fmov" "s");

// CHECK-LABEL: vreg_f64x1:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_f64x1 f64x1 vreg "fmov" "s");

// CHECK-LABEL: vreg_i8x16:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i8x16 i8x16 vreg "fmov" "s");

// CHECK-LABEL: vreg_i16x8:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i16x8 i16x8 vreg "fmov" "s");

// CHECK-LABEL: vreg_i32x4:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i32x4 i32x4 vreg "fmov" "s");

// CHECK-LABEL: vreg_i64x2:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_i64x2 i64x2 vreg "fmov" "s");

// CHECK-LABEL: vreg_f32x4:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_f32x4 f32x4 vreg "fmov" "s");

// CHECK-LABEL: vreg_f64x2:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_f64x2 f64x2 vreg "fmov" "s");

// CHECK-LABEL: vreg_v_i8x8:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_i8x8 i8x8 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_i16x4:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_i16x4 i16x4 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_i32x2:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_i32x2 i32x2 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_i64x1:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_i64x1 i64x1 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_f32x2:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_f32x2 f32x2 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_f64x1:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_f64x1 f64x1 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_i8x16:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_i8x16 i8x16 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_i16x8:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_i16x8 i16x8 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_i32x4:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_i32x4 i32x4 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_i64x2:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_i64x2 i64x2 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_f32x4:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_f32x4 f32x4 vreg "mov" "v" ".16b");

// CHECK-LABEL: vreg_v_f64x2:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_v_f64x2 f64x2 vreg "mov" "v" ".16b");

// CHECK-LABEL: zreg_i16:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(zreg_i16 i16 zreg "fmov" "s");

// CHECK-LABEL: zreg_i32:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(zreg_i32 i32 zreg "fmov" "s");

// CHECK-LABEL: zreg_f32:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(zreg_f32 f32 zreg "fmov" "s");

// CHECK-LABEL: zreg_i64:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(zreg_i64 i64 zreg "fmov" "s");

// CHECK-LABEL: zreg_f64:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(zreg_f64 f64 zreg "fmov" "s");

// CHECK-LABEL: zreg_ptr:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(zreg_ptr ptr zreg "fmov" "s");

// CHECK-LABEL: zreg_svint8_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_svint8_t svint8_t zreg "mov" "" ".d");

// CHECK-LABEL: zreg_svint16_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_svint16_t svint16_t zreg "mov" "" ".d");

// CHECK-LABEL: zreg_svint32_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_svint32_t svint32_t zreg "mov" "" ".d");

// CHECK-LABEL: zreg_svint64_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_svint64_t svint64_t zreg "mov" "" ".d");

// CHECK-LABEL: zreg_svfloat32_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_svfloat32_t svfloat32_t zreg "mov" "" ".d");

// CHECK-LABEL: zreg_svfloat64_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_svfloat64_t svfloat64_t zreg "mov" "" ".d");

// CHECK-LABEL: vreg_low16_i8:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i8 i8 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i16:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i16 i16 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_f32:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_f32 f32 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i64:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i64 i64 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_f64:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_f64 f64 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_ptr:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_ptr ptr vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i8x8:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i8x8 i8x8 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i16x4:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i16x4 i16x4 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i32x2:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i32x2 i32x2 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i64x1:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i64x1 i64x1 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_f32x2:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_f32x2 f32x2 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_f64x1:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_f64x1 f64x1 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i8x16:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i8x16 i8x16 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i16x8:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i16x8 i16x8 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i32x4:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i32x4 i32x4 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_i64x2:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_i64x2 i64x2 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_f32x4:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_f32x4 f32x4 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_f64x2:
// CHECK: //APP
// CHECK: fmov s{{[0-9]+}}, s{{[0-9]+}}
// CHECK: //NO_APP
check!(vreg_low16_f64x2 f64x2 vreg_low16 "fmov" "s");

// CHECK-LABEL: vreg_low16_v_i8x8:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_i8x8 i8x8 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_i16x4:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_i16x4 i16x4 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_i32x2:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_i32x2 i32x2 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_i64x1:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_i64x1 i64x1 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_f32x2:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_f32x2 f32x2 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_f64x1:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_f64x1 f64x1 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_i8x16:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_i8x16 i8x16 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_i16x8:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_i16x8 i16x8 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_i32x4:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_i32x4 i32x4 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_i64x2:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_i64x2 i64x2 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_f32x4:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_f32x4 f32x4 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: vreg_low16_v_f64x2:
// CHECK: //APP
// CHECK: mov v{{[0-9]+}}.16b, v{{[0-9]+}}.16b
// CHECK: //NO_APP
check!(vreg_low16_v_f64x2 f64x2 vreg_low16 "mov" "v" ".16b");

// CHECK-LABEL: zreg_low16_svint8_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_low16_svint8_t svint8_t zreg_low16 "mov" "" ".d");

// CHECK-LABEL: zreg_low16_svint16_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_low16_svint16_t svint16_t zreg_low16 "mov" "" ".d");

// CHECK-LABEL: zreg_low16_svint32_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_low16_svint32_t svint32_t zreg_low16 "mov" "" ".d");

// CHECK-LABEL: zreg_low16_svint64_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_low16_svint64_t svint64_t zreg_low16 "mov" "" ".d");

// CHECK-LABEL: zreg_low16_svfloat32_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_low16_svfloat32_t svfloat32_t zreg_low16 "mov" "" ".d");

// CHECK-LABEL: zreg_low16_svfloat64_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_low16_svfloat64_t svfloat64_t zreg_low16 "mov" "" ".d");

// CHECK-LABEL: zreg_low8_svint8_t:
// CHECK: //APP
// CHECK: mov z{{[0-9]+}}.d, z{{[0-9]+}}.d
// CHECK: //NO_APP
check!(zreg_low8_svint8_t svint8_t zreg_low8 "mov" "" ".d");

// CHECK-LABEL: zreg_low8_svint16_t:
// CHECK: //APP
// CHECK: mov z{{[0-7]}}.d, z{{[0-7]}}.d
// CHECK: //NO_APP
check!(zreg_low8_svint16_t svint16_t zreg_low8 "mov" "" ".d");

// CHECK-LABEL: zreg_low8_svint32_t:
// CHECK: //APP
// CHECK: mov z{{[0-7]}}.d, z{{[0-7]}}.d
// CHECK: //NO_APP
check!(zreg_low8_svint32_t svint32_t zreg_low8 "mov" "" ".d");

// CHECK-LABEL: zreg_low8_svint64_t:
// CHECK: //APP
// CHECK: mov z{{[0-7]}}.d, z{{[0-7]}}.d
// CHECK: //NO_APP
check!(zreg_low8_svint64_t svint64_t zreg_low8 "mov" "" ".d");

// CHECK-LABEL: zreg_low8_svfloat32_t:
// CHECK: //APP
// CHECK: mov z{{[0-7]}}.d, z{{[0-7]}}.d
// CHECK: //NO_APP
check!(zreg_low8_svfloat32_t svfloat32_t zreg_low8 "mov" "" ".d");

// CHECK-LABEL: zreg_low8_svfloat64_t:
// CHECK: //APP
// CHECK: mov z{{[0-7]}}.d, z{{[0-7]}}.d
// CHECK: //NO_APP
check!(zreg_low8_svfloat64_t svfloat64_t zreg_low8 "mov" "" ".d");

// CHECK-LABEL: preg_svbool_t:
// CHECK: //APP
// CHECK: mov p{{[0-9]+}}.b, p{{[0-9]+}}.b
// CHECK: //NO_APP
check!(preg_svbool_t svbool_t preg "mov" "" ".b");

// CHECK-LABEL: x0_i8:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check_reg!(x0_i8 i8 "x0" "mov");

// CHECK-LABEL: x0_i16:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check_reg!(x0_i16 i16 "x0" "mov");

// CHECK-LABEL: x0_i32:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check_reg!(x0_i32 i32 "x0" "mov");

// CHECK-LABEL: x0_f32:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check_reg!(x0_f32 f32 "x0" "mov");

// CHECK-LABEL: x0_i64:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check_reg!(x0_i64 i64 "x0" "mov");

// CHECK-LABEL: x0_f64:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check_reg!(x0_f64 f64 "x0" "mov");

// CHECK-LABEL: x0_ptr:
// CHECK: //APP
// CHECK: mov x{{[0-9]+}}, x{{[0-9]+}}
// CHECK: //NO_APP
check_reg!(x0_ptr ptr "x0" "mov");

// CHECK-LABEL: v0_i8:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i8 i8 "s0" "fmov");

// CHECK-LABEL: v0_i16:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i16 i16 "s0" "fmov");

// CHECK-LABEL: v0_i32:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i32 i32 "s0" "fmov");

// CHECK-LABEL: v0_f32:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_f32 f32 "s0" "fmov");

// CHECK-LABEL: v0_i64:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i64 i64 "s0" "fmov");

// CHECK-LABEL: v0_f64:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_f64 f64 "s0" "fmov");

// CHECK-LABEL: v0_ptr:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_ptr ptr "s0" "fmov");

// CHECK-LABEL: v0_i8x8:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i8x8 i8x8 "s0" "fmov");

// CHECK-LABEL: v0_i16x4:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i16x4 i16x4 "s0" "fmov");

// CHECK-LABEL: v0_i32x2:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i32x2 i32x2 "s0" "fmov");

// CHECK-LABEL: v0_i64x1:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i64x1 i64x1 "s0" "fmov");

// CHECK-LABEL: v0_f32x2:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_f32x2 f32x2 "s0" "fmov");

// CHECK-LABEL: v0_f64x1:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_f64x1 f64x1 "s0" "fmov");

// CHECK-LABEL: v0_i8x16:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i8x16 i8x16 "s0" "fmov");

// CHECK-LABEL: v0_i16x8:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i16x8 i16x8 "s0" "fmov");

// CHECK-LABEL: v0_i32x4:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i32x4 i32x4 "s0" "fmov");

// CHECK-LABEL: v0_i64x2:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_i64x2 i64x2 "s0" "fmov");

// CHECK-LABEL: v0_f32x4:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_f32x4 f32x4 "s0" "fmov");

// CHECK-LABEL: v0_f64x2:
// CHECK: //APP
// CHECK: fmov s0, s0
// CHECK: //NO_APP
check_reg!(v0_f64x2 f64x2 "s0" "fmov");

// CHECK-LABEL: v0_v_i8x16:
// CHECK: //APP
// CHECK: mov v0.16b, v0.16b
// CHECK: //NO_APP
check_reg!(v0_v_i8x16 i8x16 "v0" "mov" ".16b");

// CHECK-LABEL: v0_v_i16x8:
// CHECK: //APP
// CHECK: mov v0.16b, v0.16b
// CHECK: //NO_APP
check_reg!(v0_v_i16x8 i16x8 "v0" "mov" ".16b");

// CHECK-LABEL: v0_v_i32x4:
// CHECK: //APP
// CHECK: mov v0.16b, v0.16b
// CHECK: //NO_APP
check_reg!(v0_v_i32x4 i32x4 "v0" "mov" ".16b");

// CHECK-LABEL: v0_v_i64x2:
// CHECK: //APP
// CHECK: mov v0.16b, v0.16b
// CHECK: //NO_APP
check_reg!(v0_v_i64x2 i64x2 "v0" "mov" ".16b");

// CHECK-LABEL: v0_v_f32x4:
// CHECK: //APP
// CHECK: mov v0.16b, v0.16b
// CHECK: //NO_APP
check_reg!(v0_v_f32x4 f32x4 "v0" "mov" ".16b");

// CHECK-LABEL: v0_v_f64x2:
// CHECK: //APP
// CHECK: mov v0.16b, v0.16b
// CHECK: //NO_APP
check_reg!(v0_v_f64x2 f64x2 "v0" "mov" ".16b");

// CHECK-LABEL: z0_svint8_t:
// CHECK: //APP
// CHECK: mov z0.d, z0.d
// CHECK: //NO_APP
check_reg!(z0_svint8_t svint8_t "z0" "mov" ".d");

// CHECK-LABEL: z0_svint16_t:
// CHECK: //APP
// CHECK: mov z0.d, z0.d
// CHECK: //NO_APP
check_reg!(z0_svint16_t svint16_t "z0" "mov" ".d");

// CHECK-LABEL: z0_svint32_t:
// CHECK: //APP
// CHECK: mov z0.d, z0.d
// CHECK: //NO_APP
check_reg!(z0_svint32_t svint32_t "z0" "mov" ".d");

// CHECK-LABEL: z0_svint64_t:
// CHECK: //APP
// CHECK: mov z0.d, z0.d
// CHECK: //NO_APP
check_reg!(z0_svint64_t svint64_t "z0" "mov" ".d");

// CHECK-LABEL: z0_svfloat32_t:
// CHECK: //APP
// CHECK: mov z0.d, z0.d
// CHECK: //NO_APP
check_reg!(z0_svfloat32_t svfloat32_t "z0" "mov" ".d");

// CHECK-LABEL: z0_svfloat64_t:
// CHECK: //APP
// CHECK: mov z0.d, z0.d
// CHECK: //NO_APP
check_reg!(z0_svfloat64_t svfloat64_t "z0" "mov" ".d");

// CHECK-LABEL: p0_svbool_t:
// CHECK: //APP
// CHECK: mov p0.b, p0.b
// CHECK: //NO_APP
check_reg!(p0_svbool_t svbool_t "p0" "mov" ".b");
