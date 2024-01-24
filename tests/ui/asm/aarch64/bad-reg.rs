// only-aarch64
// compile-flags: -A incomplete-features -C target-feature=+neon,+sve

#![feature(repr_simd, stdarch_aarch64_sve, stdsimd, asm_const, unsized_locals, unsized_fn_params)]

use std::arch::aarch64::float64x2_t;
use std::arch::aarch64::sve::svfloat64_t;
use std::arch::aarch64::sve::svdup_n_f64;
use std::arch::asm;

fn main() {
    let mut foo = 0;
    let mut bar = 0;

    let f64x2: float64x2_t = unsafe { std::mem::transmute(0i128) };
    let mut f64xN: svfloat64_t = svdup_n_f64(0.0);

    unsafe {
        // Bad register/register class

        asm!("{}", in(foo) foo);
        //~^ ERROR invalid register class `foo`: unknown register class
        asm!("", in("foo") foo);
        //~^ ERROR invalid register `foo`: unknown register
        asm!("{:z}", in(reg) foo);
        //~^ ERROR invalid asm template modifier for this register class
        asm!("{:r}", in(vreg) foo);
        //~^ ERROR invalid asm template modifier for this register class
        asm!("{:r}", in(vreg_low16) foo);
        //~^ ERROR invalid asm template modifier for this register class
        asm!("{:v}", in(zreg) foo);
        //~^ ERROR invalid asm template modifier for this register class
        asm!("{:a}", const 0);
        //~^ ERROR asm template modifiers are not allowed for `const` arguments
        asm!("{:a}", sym main);
        //~^ ERROR asm template modifiers are not allowed for `sym` arguments
        asm!("", in("x29") foo);
        //~^ ERROR invalid register `x29`: the frame pointer cannot be used as an operand
        asm!("", in("sp") foo);
        //~^ ERROR invalid register `sp`: the stack pointer cannot be used as an operand
        asm!("", in("xzr") foo);
        //~^ ERROR invalid register `xzr`: the zero register cannot be used as an operand
        asm!("", in("x19") foo);
        //~^ ERROR invalid register `x19`: x19 is used internally by LLVM and cannot be used as an operand for inline asm

        asm!("{}", in(zreg) f64x2);
        //~^ ERROR type `float64x2_t` cannot be used with this register class
        asm!("{}", in(vreg) f64xN);
        //~^ ERROR type `svfloat64_t` cannot be used with this register class

        asm!("", in("ffr") foo);
        //~^ ERROR register class `ffr_reg` can only be used as a clobber, not as an input or output
        //~| ERROR type `i32` cannot be used with this register class
        asm!("", out("ffr") _);
        asm!("{}", in(ffr_reg) foo);
        //~^ ERROR register class `ffr_reg` can only be used as a clobber, not as an input or output
        //~| ERROR type `i32` cannot be used with this register class
        asm!("{}", out(ffr_reg) _);
        //~^ ERROR register class `ffr_reg` can only be used as a clobber, not as an input or output

        // Explicit register conflicts
        // (except in/lateout which don't conflict)

        asm!("", in("x0") foo, in("w0") bar);
        //~^ ERROR register `x0` conflicts with register `x0`
        asm!("", in("x0") foo, out("x0") bar);
        //~^ ERROR register `x0` conflicts with register `x0`
        asm!("", in("w0") foo, lateout("w0") bar);
        asm!("", in("v0") foo, in("q0") bar);
        //~^ ERROR register `v0` conflicts with register `v0`
        asm!("", in("v0") foo, out("q0") bar);
        //~^ ERROR register `v0` conflicts with register `v0`
        asm!("", in("v0") foo, lateout("q0") bar);
        asm!("", in("v0") foo, in("z0") bar);
        //~^ ERROR register `z0` conflicts with register `v0`
        asm!("", in("v0") foo, out("z0") bar);
        //~^ ERROR register `z0` conflicts with register `v0`
        asm!("", in("v0") foo, lateout("z0") bar);
    }
}
