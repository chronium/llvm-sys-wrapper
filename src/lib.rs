extern crate libc;
extern crate llvm_sys;

mod builder;
mod context;
mod cstring_manager;
mod engine;
mod function;
mod module;
mod phi;
mod struct_type;

pub use self::builder::Builder;
pub use self::context::Context;
pub use self::engine::{Engine, FuncallResult};
pub use self::function::Function;
pub use self::llvm_sys::core::*;
pub use self::llvm_sys::prelude::*;
pub use self::llvm_sys::*;
pub use self::module::Module;
pub use self::phi::Phi;
pub use self::struct_type::Struct;
use llvm_sys::target_machine::LLVMCodeGenOptLevel;

pub enum CPU {
    Native,
    X86_64,
    I686,
}

use std::ffi::CString;
impl From<CPU> for CString {
    fn from(cpu: CPU) -> CString {
        match cpu {
            CPU::Native => CString::new("native").expect(""),
            CPU::X86_64 => CString::new("x86-64").expect(""),
            CPU::I686 => CString::new("i686").expect(""),
        }
    }
}

pub enum CodegenLevel {
    O0,
    O1,
    O2,
    O3,
}

impl From<CodegenLevel> for LLVMCodeGenOptLevel {
    fn from(level: CodegenLevel) -> LLVMCodeGenOptLevel {
        match level {
            CodegenLevel::O0 => LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
            CodegenLevel::O1 => LLVMCodeGenOptLevel::LLVMCodeGenLevelLess,
            CodegenLevel::O2 => LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
            CodegenLevel::O3 => LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
        }
    }
}

#[allow(non_snake_case)]
pub mod LLVM {
    use llvm_sys::core::*;
    use llvm_sys::prelude::*;
    use llvm_sys::target;
    use llvm_sys::target_machine::*;
    use module::Module;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::os::raw::c_uint;
    use std::ptr;
    use CodegenLevel;
    use CPU;

    pub fn initialize() {
        unsafe {
            if target::LLVM_InitializeNativeTarget() != 0 {
                panic!("Could not initialise target");
            }
            if target::LLVM_InitializeNativeAsmPrinter() != 0 {
                panic!("Could not initialise ASM Printer");
            }
        }
    }

    pub fn emit(
        module: &Module,
        opt_level: CodegenLevel,
        out: String,
        cpu: CPU,
    ) -> Result<(), String> {
        let features = CString::new("").expect("");
        let cpu: CString = cpu.into();
        let out = CString::new(out).expect("");
        unsafe {
            let triple = LLVMGetDefaultTargetTriple();
            let target = LLVMGetFirstTarget();
            let reloc_mode = LLVMRelocMode::LLVMRelocDefault;
            let code_model = LLVMCodeModel::LLVMCodeModelDefault;
            let target_machine = LLVMCreateTargetMachine(
                target,
                triple,
                cpu.as_ptr(),
                features.as_ptr(),
                opt_level.into(),
                reloc_mode,
                code_model,
            );
            let file_type = LLVMCodeGenFileType::LLVMObjectFile;

            let mut err_str = ptr::null_mut();
            let res = LLVMTargetMachineEmitToFile(
                target_machine,
                module.as_ref(),
                out.as_ptr() as *mut i8,
                file_type,
                &mut err_str,
            );

            if res == 1 {
                let err = CStr::from_ptr(err_str);
                Err(err.to_string_lossy().into_owned())
            } else {
                Ok(())
            }
        }
    }

    pub mod Type {
        use super::*;

        #[inline]
        pub fn PointerType(elem_type: LLVMTypeRef, address_space: c_uint) -> LLVMTypeRef {
            unsafe { LLVMPointerType(elem_type, address_space) }
        }
        #[inline]
        pub fn Pointer(elem_type: LLVMTypeRef, address_space: c_uint) -> LLVMTypeRef {
            unsafe { LLVMPointerType(elem_type, address_space) }
        }
        #[inline]
        pub fn Void() -> LLVMTypeRef {
            unsafe { LLVMVoidType() }
        }
        #[inline]
        pub fn Int(num_bits: c_uint) -> LLVMTypeRef {
            unsafe { LLVMIntType(num_bits) }
        }
        #[inline]
        pub fn Int1() -> LLVMTypeRef {
            unsafe { LLVMInt1Type() }
        }
        #[inline]
        pub fn Int8() -> LLVMTypeRef {
            unsafe { LLVMInt8Type() }
        }
        #[inline]
        pub fn Int16() -> LLVMTypeRef {
            unsafe { LLVMInt16Type() }
        }
        #[inline]
        pub fn Int32() -> LLVMTypeRef {
            unsafe { LLVMInt32Type() }
        }
        #[inline]
        pub fn Int64() -> LLVMTypeRef {
            unsafe { LLVMInt64Type() }
        }
        #[inline]
        pub fn Int128() -> LLVMTypeRef {
            unsafe { LLVMInt128Type() }
        }
        #[inline]
        pub fn Half() -> LLVMTypeRef {
            unsafe { LLVMHalfType() }
        }
        #[inline]
        pub fn Float() -> LLVMTypeRef {
            unsafe { LLVMFloatType() }
        }
        #[inline]
        pub fn Double() -> LLVMTypeRef {
            unsafe { LLVMDoubleType() }
        }
        #[inline]
        pub fn FP128() -> LLVMTypeRef {
            unsafe { LLVMFP128Type() }
        }
        #[inline]
        pub fn X86FP80() -> LLVMTypeRef {
            unsafe { LLVMX86FP80Type() }
        }
        #[inline]
        pub fn PPCFP128() -> LLVMTypeRef {
            unsafe { LLVMPPCFP128Type() }
        }
        #[inline]
        pub fn X86MMX() -> LLVMTypeRef {
            unsafe { LLVMX86MMXType() }
        }
        #[inline]
        pub fn Label() -> LLVMTypeRef {
            unsafe { LLVMLabelType() }
        }
        #[inline]
        pub fn CharPointer() -> LLVMTypeRef {
            Type::PointerType(Type::Int8(), 0)
        }
        #[inline]
        pub fn Int8Pointer() -> LLVMTypeRef {
            Type::PointerType(Type::Int8(), 0)
        }
    }

    pub mod Const {
        use super::*;

        #[inline]
        pub fn SInt(num_bits: c_uint, val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMIntType(num_bits), val, 1) }
        }
        #[inline]
        pub fn UInt(num_bits: c_uint, val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMIntType(num_bits), val, 0) }
        }
        #[inline]
        pub fn SInt1(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt1Type(), val, 1) }
        }
        #[inline]
        pub fn UInt1(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt1Type(), val, 0) }
        }
        #[inline]
        pub fn SInt8(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt8Type(), val, 1) }
        }
        #[inline]
        pub fn UInt8(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt8Type(), val, 0) }
        }
        #[inline]
        pub fn SInt16(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt16Type(), val, 1) }
        }
        #[inline]
        pub fn UInt16(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt16Type(), val, 0) }
        }
        #[inline]
        pub fn SInt32(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt32Type(), val, 1) }
        }
        #[inline]
        pub fn UInt32(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt32Type(), val, 0) }
        }
        #[inline]
        pub fn SInt64(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt64Type(), val, 1) }
        }
        #[inline]
        pub fn UInt64(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt64Type(), val, 0) }
        }
        #[inline]
        pub fn SInt128(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt128Type(), val, 1) }
        }
        #[inline]
        pub fn UInt128(val: u64) -> LLVMValueRef {
            unsafe { LLVMConstInt(LLVMInt128Type(), val, 0) }
        }

        #[inline]
        pub fn Half(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMHalfType(), val) }
        }
        #[inline]
        pub fn Float(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMFloatType(), val) }
        }
        #[inline]
        pub fn Double(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMDoubleType(), val) }
        }
        #[inline]
        pub fn FP128(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMFP128Type(), val) }
        }
        #[inline]
        pub fn X86FP80(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMX86FP80Type(), val) }
        }
        #[inline]
        pub fn PPCFP128(val: f64) -> LLVMValueRef {
            unsafe { LLVMConstReal(LLVMPPCFP128Type(), val) }
        }
    }
}

#[macro_export]
macro_rules! fn_type {
    ($result_type:expr) => (
        unsafe {
            let mut param_types = [];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 0)
        }
    );
    ($result_type:expr,,,) => (
        unsafe {
            let mut param_types = [];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 1)
        }
    );
    ($result_type:expr, $( $param_type:expr ),* ) => (
        unsafe {
            let mut param_types = [ $( $param_type ),* ];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 0)
        }
    );
    ($result_type:expr, $( $param_type:expr ),* ,,,) => (
        unsafe {
            let mut param_types = [ $( $param_type ),* ];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 1)
        }
    )
}
