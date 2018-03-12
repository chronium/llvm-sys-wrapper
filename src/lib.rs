extern crate llvm_sys;

mod builder;
mod module;
mod function;
mod context;

#[allow(unused_imports)]
use builder::Builder;
#[allow(unused_imports)]
use module::Module;
#[allow(unused_imports)]
use function::Function;
#[allow(unused_imports)]
use context::Context;

#[allow(non_snake_case)]
pub mod LLVM {
    use llvm_sys::core::*;
    use llvm_sys::target;
    use llvm_sys::prelude::*;
    use std::os::raw::c_uint;

    pub fn initialize(){
        unsafe {
            if target::LLVM_InitializeNativeTarget() != 0 {
                panic!("Could not initialise target");
            }
            if target::LLVM_InitializeNativeAsmPrinter() != 0 {
                panic!("Could not initialise ASM Printer");
            }
        }
    }

    pub mod types {
        use super::*;

        pub fn PointerType(elem_type: LLVMTypeRef, address_space: c_uint) -> LLVMTypeRef {
            unsafe { LLVMPointerType(elem_type, address_space) }
        }
        pub fn Half() -> LLVMTypeRef {
            unsafe { LLVMHalfType() }
        }
        pub fn Int1() -> LLVMTypeRef {
            unsafe { LLVMInt1Type() }
        }
        pub fn Int8() -> LLVMTypeRef {
            unsafe { LLVMInt8Type() }
        }
        pub fn Void() -> LLVMTypeRef {
            unsafe { LLVMVoidType() }
        }
        pub fn Float() -> LLVMTypeRef {
            unsafe { LLVMFloatType() }
        }
        pub fn FP128() -> LLVMTypeRef {
            unsafe { LLVMFP128Type() }
        }
        pub fn Int16() -> LLVMTypeRef {
            unsafe { LLVMInt16Type() }
        }
        pub fn Int32() -> LLVMTypeRef {
            unsafe { LLVMInt32Type() }
        }
        pub fn Int64() -> LLVMTypeRef {
            unsafe { LLVMInt64Type() }
        }
        pub fn Label() -> LLVMTypeRef {
            unsafe { LLVMLabelType() }
        }
        pub fn Double() -> LLVMTypeRef {
            unsafe { LLVMDoubleType() }
        }
        pub fn Int128() -> LLVMTypeRef {
            unsafe { LLVMInt128Type() }
        }
        pub fn X86MMX() -> LLVMTypeRef {
            unsafe { LLVMX86MMXType() }
        }
        pub fn X86FP80() -> LLVMTypeRef {
            unsafe { LLVMX86FP80Type() }
        }
        pub fn PPCFP128() -> LLVMTypeRef {
            unsafe { LLVMPPCFP128Type() }
        }
    }
}

#[allow(unused_macros)]
macro_rules! function_type {
    ($result_type:expr) => (
        unsafe {
            let mut param_types = [];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 0)
        }
    );
    ($result_type:expr, ...) => (
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
    ($result_type:expr, $( $param_type:expr ),* , ...) => (
        unsafe {
            let mut param_types = [ $( $param_type ),* ];
            LLVMFunctionType($result_type, param_types.as_mut_ptr(), param_types.len() as u32, 1)
        }
    )
}

#[cfg(test)]
mod tests {
    use llvm_sys::core::*;
    use super::*;

/*
    #[test]
    fn it_works() {
        // 参考: [Go言語で利用するLLVM入門](https://postd.cc/an-introduction-to-llvm-in-go/)

        //LLVM::initialize();

        // setup our builder and module
        let builder = Builder::new();
        let module = Module::new(builder.as_ref(), "my_module");

        // create our function prologue
        let fun_type = function_type!(LLVM::types::Int32());
        let function = Function::new(builder.as_ref(), module.as_ref(), "main", fun_type);
        let entry_block = function.append_basic_block("entry");
        builder.position_at_end(entry_block);

        // int a = 32
        let typ = unsafe { LLVMInt32Type() };
        let a = builder.build_alloca(typ, "a");
        let const_a_value = unsafe { LLVMConstInt(LLVMInt32Type(), 32, 0) };
        builder.build_store(const_a_value, a);

        // int b = 16
        let b = builder.build_alloca(typ, "b");
        let const_b_value = unsafe { LLVMConstInt(LLVMInt32Type(), 16, 0) };
        builder.build_store(const_b_value, b);

        // return a + b
        let a_val = builder.build_load(a, "a_val");
        let b_val = builder.build_load(b, "b_val");
        let ab_val = builder.build_add(a_val, b_val, "ab_val");
        builder.build_ret(ab_val);

        // verify & dump
        match module.verify() {
            Ok(_) => {
                module.dump()
            },
            Err(msg) => println!("Error: {}", msg)
        }        
    }
*/

    #[test]
    fn test_puts() {
        // 参考: [llvm で Hello wolrd!! 〜llvm入門 その2〜](http://blog.64p.org/entry/2012/07/18/172418)

        LLVM::initialize();

        // create context
        let context = Context::global_context();

         // setup our builder and module
        let builder = Builder::new();
        let module = Module::with_context(builder.as_ref(), "top", context);

        // create main function and entry point
        let fun_type = function_type!(LLVM::types::Void());
        let function = Function::new(builder.as_ref(), module.as_ref(), "main", fun_type);
        let entry_block = function.append_basic_block("entry");
        builder.position_at_end(entry_block);

        let helloworld = builder.build_global_string_ptr("Hello, world!\n", "hello_world_str");

        let puts_type = function_type!(LLVM::types::Int32(), LLVM::types::PointerType(LLVM::types::Int8(), 0) );
        let puts_func = Function::new(builder.as_ref(), module.as_ref(), "puts", puts_type);

        let mut args = [helloworld];
        let _call = builder.build_call(puts_func.as_ref(), args.as_mut_ptr(), args.len() as u32, "call_puts");

        let _ret = builder.build_ret_void();

        // verify & dump
        match module.verify() {
            Ok(_) => {
                module.dump()
            },
            Err(msg) => println!("Error: {}", msg)
        }
    }
}
