#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use cstring_manager::CStringManager;
use engine::Engine;
use function;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Debug)]
pub struct Module {
    llvm_module: LLVMModuleRef,
}

impl Module {
    pub fn new(name: &str) -> Module {
        let mod_name_ptr = CStringManager::new_cstring_as_ptr(name);
        let module = unsafe { LLVMModuleCreateWithName(mod_name_ptr) };
        Module {
            llvm_module: module,
        }
    }

    pub fn new_in_context(name: &str, context: LLVMContextRef) -> Module {
        let mod_name_ptr = CStringManager::new_cstring_as_ptr(name);
        let module = unsafe { LLVMModuleCreateWithNameInContext(mod_name_ptr, context) };
        Module {
            llvm_module: module,
        }
    }

    pub fn as_ref(&self) -> LLVMModuleRef {
        self.llvm_module
    }

    pub fn add_function(&self, name: &str, function_type: LLVMTypeRef) -> function::Function {
        function::Function::new(self.llvm_module, name, function_type)
    }

    #[inline]
    pub fn named_function(&self, name: &str) -> function::Function {
        let func_name_ptr = CStringManager::new_cstring_as_ptr(name);
        let named_function = unsafe { LLVMGetNamedFunction(self.llvm_module, func_name_ptr) };
        function::Function::from_ptr(named_function)
    }

    pub fn get_or_add_function(
        &self,
        name: &str,
        function_type: LLVMTypeRef,
    ) -> function::Function {
        let func_name_ptr = CStringManager::new_cstring_as_ptr(name);
        let named_function = unsafe { LLVMGetNamedFunction(self.llvm_module, func_name_ptr) };
        if named_function.is_null() {
            function::Function::new(self.llvm_module, name, function_type)
        } else {
            function::Function::from_ptr(named_function)
        }
    }

    #[inline]
    pub fn add_global(&self, ty: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let glob_name_ptr = CStringManager::new_cstring_as_ptr(name);
        unsafe { LLVMAddGlobal(self.as_ref(), ty, glob_name_ptr) }
    }

    #[inline]
    pub fn set_initializer(&self, ptr: LLVMValueRef, val: LLVMValueRef) {
        unsafe { LLVMSetInitializer(ptr, val) }
    }

    pub fn verify(&self) -> Result<(), String> {
        let mut error: *mut c_char = 0 as *mut c_char;
        let ok = unsafe {
            let buf: *mut *mut c_char = &mut error;
            LLVMVerifyModule(
                self.llvm_module,
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
                buf,
            )
        };
        if ok == 1 {
            // error
            let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
            Err(err_msg)
        } else {
            // success
            Ok(())
        }
    }

    #[inline]
    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.llvm_module) }
    }

    pub fn print_module_to_string(&self) -> String {
        let ptr = unsafe { LLVMPrintModuleToString(self.llvm_module) };
        let string = unsafe { CString::from_raw(ptr).into_string().unwrap() };
        unsafe {
            LLVMDisposeMessage(ptr);
        }
        string
    }

    pub fn print_module_to_file(&self, filename: &str) -> Result<(), String> {
        let fname_ptr = CStringManager::new_cstring_as_ptr(filename);
        let mut error: *mut c_char = 0 as *mut c_char;
        let ok = unsafe {
            let buf: *mut *mut c_char = &mut error;
            LLVMPrintModuleToFile(self.llvm_module, fname_ptr, buf)
        };
        if ok == 1 {
            // error
            let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
            unsafe { LLVMDisposeMessage(error) }
            Err(err_msg)
        } else {
            // success
            Ok(())
        }
    }

    #[inline]
    pub fn create_interpreter(&self) -> Result<Engine, String> {
        Engine::create_interpreter(self.as_ref())
    }

    #[inline]
    pub fn create_jit_engine(&self) -> Result<Engine, String> {
        Engine::create_jit_engine(self.as_ref())
    }
}

impl Drop for Module {
    #[inline]
    fn drop(&mut self) {
        unsafe { LLVMDisposeModule(self.llvm_module) }
    }
}
