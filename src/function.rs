#![allow(dead_code)]

extern crate llvm_sys;

use self::llvm_sys::core::*;
use self::llvm_sys::prelude::*;
use cstring_manager::CStringManager;

#[derive(Debug, Clone)]
pub struct Function {
    llvm_function: LLVMValueRef,
    llvm_module: LLVMModuleRef,
    function_type: LLVMTypeRef,
}

impl Function {
    pub fn new(module: LLVMModuleRef, name: &str, function_type: LLVMTypeRef) -> Function {
        let function_name_ptr = CStringManager::new_cstring_as_ptr(name);
        let function = unsafe { LLVMAddFunction(module, function_name_ptr, function_type) };
        Function {
            llvm_function: function,
            llvm_module: module,
            function_type: function_type,
        }
    }

    pub fn from_ptr(func_ptr: LLVMValueRef) -> Function {
        Function {
            llvm_function: func_ptr,
            llvm_module: 0 as LLVMModuleRef,
            function_type: 0 as LLVMTypeRef,
        }
    }

    pub fn append_basic_block(&self, name: &str) -> LLVMBasicBlockRef {
        let label_name_ptr = CStringManager::new_cstring_as_ptr(name);
        if self.llvm_module.is_null() {
            unsafe { LLVMAppendBasicBlock(self.llvm_function, label_name_ptr) }
        } else {
            let context = unsafe { LLVMGetModuleContext(self.llvm_module) };
            unsafe { LLVMAppendBasicBlockInContext(context, self.llvm_function, label_name_ptr) }
        }
    }

    pub fn as_ref(&self) -> LLVMValueRef {
        self.llvm_function
    }

    #[inline]
    pub fn get_param(&self, index: u32) -> LLVMValueRef {
        unsafe { LLVMGetParam(self.llvm_function, index) }
    }

    #[inline]
    pub fn params_count(&self) -> u32 {
        unsafe { LLVMCountParams(self.llvm_function) }
    }

    #[inline]
    pub fn get_function_type(&self) -> LLVMTypeRef {
        self.function_type
    }

    #[inline]
    pub fn get_return_type(&self) -> LLVMTypeRef {
        unsafe { LLVMGetReturnType(self.function_type) }
    }

    #[inline]
    pub fn get_param_types(&self) -> LLVMTypeRef {
        let mut types: LLVMTypeRef = 0 as LLVMTypeRef;
        let ptr: *mut LLVMTypeRef = &mut types;
        unsafe {
            LLVMGetParamTypes(self.function_type, ptr);
        }
        types
    }
}
