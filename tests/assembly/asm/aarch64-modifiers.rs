// assembly-output: emit-asm
// compile-flags: -O
// compile-flags: --target aarch64-unknown-linux-gnu -C target-feature=+sve
// needs-llvm-components: aarch64

#![feature(no_core, lang_items, rustc_attrs, repr_simd, repr_scalable, unsized_locals, unsized_fn_params)]
#![crate_type = "rlib"]
#![no_core]
#![allow(asm_sub_register)]

#[rustc_builtin_macro]
macro_rules! asm {
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

impl Copy for i32 {}

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

macro_rules! check {
    ($func:ident $reg:ident $code:literal) => {
        // -O and extern "C" guarantee that the selected register is always r0/s0/d0/q0
        #[no_mangle]
        pub unsafe extern "C" fn $func() -> i32 {
            // Hack to avoid function merging
            extern "Rust" {
                fn dont_merge(s: &str);
            }
            dont_merge(stringify!($func));

            let y;
            asm!($code, out($reg) y);
            y
        }
    };
}

macro_rules! check_sve {
    ($func:ident $reg:ident $code:literal) => {
        // -O and extern "C" guarantee that the selected register is always r0/s0/d0/q0
        #[no_mangle]
        #[target_feature(enable = "sve")]
        pub unsafe extern "C" fn $func() -> svint32_t {
            // Hack to avoid function merging
            extern "Rust" {
                fn dont_merge(s: &str);
            }
            dont_merge(stringify!($func));

            let y;
            asm!($code, out($reg) y);
            y
        }
    };
}

// CHECK-LABEL: reg:
// CHECK: //APP
// CHECK: mov x0, x0
// CHECK: //NO_APP
check!(reg reg "mov {0}, {0}");

// CHECK-LABEL: reg_w:
// CHECK: //APP
// CHECK: mov w0, w0
// CHECK: //NO_APP
check!(reg_w reg "mov {0:w}, {0:w}");

// CHECK-LABEL: reg_x:
// CHECK: //APP
// CHECK: mov x0, x0
// CHECK: //NO_APP
check!(reg_x reg "mov {0:x}, {0:x}");

// CHECK-LABEL: vreg:
// CHECK: //APP
// CHECK: add v0.4s, v0.4s, v0.4s
// CHECK: //NO_APP
check!(vreg vreg "add {0}.4s, {0}.4s, {0}.4s");

// CHECK-LABEL: vreg_b:
// CHECK: //APP
// CHECK: ldr b0, [x0]
// CHECK: //NO_APP
check!(vreg_b vreg "ldr {:b}, [x0]");

// CHECK-LABEL: vreg_h:
// CHECK: //APP
// CHECK: ldr h0, [x0]
// CHECK: //NO_APP
check!(vreg_h vreg "ldr {:h}, [x0]");

// CHECK-LABEL: vreg_s:
// CHECK: //APP
// CHECK: ldr s0, [x0]
// CHECK: //NO_APP
check!(vreg_s vreg "ldr {:s}, [x0]");

// CHECK-LABEL: vreg_d:
// CHECK: //APP
// CHECK: ldr d0, [x0]
// CHECK: //NO_APP
check!(vreg_d vreg "ldr {:d}, [x0]");

// CHECK-LABEL: vreg_q:
// CHECK: //APP
// CHECK: ldr q0, [x0]
// CHECK: //NO_APP
check!(vreg_q vreg "ldr {:q}, [x0]");

// CHECK-LABEL: vreg_v:
// CHECK: //APP
// CHECK: add v0.4s, v0.4s, v0.4s
// CHECK: //NO_APP
check!(vreg_v vreg "add {0:v}.4s, {0:v}.4s, {0:v}.4s");

// CHECK-LABEL: vreg_low16:
// CHECK: //APP
// CHECK: add v0.4s, v0.4s, v0.4s
// CHECK: //NO_APP
check!(vreg_low16 vreg_low16 "add {0}.4s, {0}.4s, {0}.4s");

// CHECK-LABEL: vreg_low16_b:
// CHECK: //APP
// CHECK: ldr b0, [x0]
// CHECK: //NO_APP
check!(vreg_low16_b vreg_low16 "ldr {:b}, [x0]");

// CHECK-LABEL: vreg_low16_h:
// CHECK: //APP
// CHECK: ldr h0, [x0]
// CHECK: //NO_APP
check!(vreg_low16_h vreg_low16 "ldr {:h}, [x0]");

// CHECK-LABEL: vreg_low16_s:
// CHECK: //APP
// CHECK: ldr s0, [x0]
// CHECK: //NO_APP
check!(vreg_low16_s vreg_low16 "ldr {:s}, [x0]");

// CHECK-LABEL: vreg_low16_d:
// CHECK: //APP
// CHECK: ldr d0, [x0]
// CHECK: //NO_APP
check!(vreg_low16_d vreg_low16 "ldr {:d}, [x0]");

// CHECK-LABEL: vreg_low16_q:
// CHECK: //APP
// CHECK: ldr q0, [x0]
// CHECK: //NO_APP
check!(vreg_low16_q vreg_low16 "ldr {:q}, [x0]");

// CHECK-LABEL: vreg_low16_v:
// CHECK: //APP
// CHECK: add v0.4s, v0.4s, v0.4s
// CHECK: //NO_APP
check!(vreg_low16_v vreg_low16 "add {0:v}.4s, {0:v}.4s, {0:v}.4s");

// CHECK-LABEL: zreg:
// CHECK: //APP
// CHECK: add z0.s, z0.s, z0.s
// CHECK: //NO_APP
check_sve!(zreg zreg "add {0}.s, {0}.s, {0}.s");

// CHECK-LABEL: zreg_z:
// CHECK: //APP
// CHECK: add z0.s, z0.s, z0.s
// CHECK: //NO_APP
check_sve!(zreg_z zreg "add {0:z}.s, {0:z}.s, {0:z}.s");

// CHECK-LABEL: zreg_b:
// CHECK: //APP
// CHECK: ldr b0, [x0]
// CHECK: //NO_APP
check!(zreg_b zreg "ldr {:b}, [x0]");

// CHECK-LABEL: zreg_h:
// CHECK: //APP
// CHECK: ldr h0, [x0]
// CHECK: //NO_APP
check!(zreg_h zreg "ldr {:h}, [x0]");

// CHECK-LABEL: zreg_s:
// CHECK: //APP
// CHECK: ldr s0, [x0]
// CHECK: //NO_APP
check!(zreg_s zreg "ldr {:s}, [x0]");

// CHECK-LABEL: zreg_d:
// CHECK: //APP
// CHECK: ldr d0, [x0]
// CHECK: //NO_APP
check!(zreg_d zreg "ldr {:d}, [x0]");

// CHECK-LABEL: zreg_q:
// CHECK: //APP
// CHECK: ldr q0, [x0]
// CHECK: //NO_APP
check!(zreg_q zreg "ldr {:q}, [x0]");

// CHECK-LABEL: zreg_low16:
// CHECK: //APP
// CHECK: add z0.s, z0.s, z0.s
// CHECK: //NO_APP
check_sve!(zreg_low16 zreg_low16 "add {0}.s, {0}.s, {0}.s");

// CHECK-LABEL: zreg_low16_z:
// CHECK: //APP
// CHECK: add z0.s, z0.s, z0.s
// CHECK: //NO_APP
check_sve!(zreg_low16_z zreg "add {0:z}.s, {0:z}.s, {0:z}.s");

// CHECK-LABEL: zreg_low8:
// CHECK: //APP
// CHECK: add z0.s, z0.s, z0.s
// CHECK: //NO_APP
check_sve!(zreg_low8 zreg_low8 "add {0}.s, {0}.s, {0}.s");

// CHECK-LABEL: zreg_low8_z:
// CHECK: //APP
// CHECK: add z0.s, z0.s, z0.s
// CHECK: //NO_APP
check_sve!(zreg_low8_z zreg_low8 "add {0:z}.s, {0:z}.s, {0:z}.s");
