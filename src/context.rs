use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

use super::*;

use value::IntoConstValue;

// LLVM Wrappers

pub struct Context {
    pub ptr: LLVMContextRef
}
impl_llvm_ref!(Context, LLVMContextRef);

impl Context {
    pub fn new() -> Self {
        let context = unsafe {
            llvm::LLVMContextCreate()
        };
        Context { ptr: context }
    }

    pub fn create_builder(&self) -> Builder {
        let builder = unsafe {
            llvm::LLVMCreateBuilderInContext(self.ptr)
        };
        Builder { ptr: builder }
    }

    pub fn module_create_with_name(&self, name: &str) -> Module {
        let c_name = CString::new(name).unwrap();
        let module = unsafe {
            llvm::LLVMModuleCreateWithNameInContext(c_name.as_ptr(), self.ptr)
        };
        Module { ptr: module }
    }

    pub fn void_type(&self) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMVoidTypeInContext(self.ptr)
        }
    }

    pub fn append_basic_block(&self, func: &mut Function, name: &str) -> LLVMBasicBlockRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMAppendBasicBlockInContext(self.ptr, func.ptr, c_name.as_ptr())
        }
    }

    /// Creates a constant in this context
    /// The value must implement the trait `IntoValue`
    pub fn cons<T: IntoConstValue>(&self, val: T) -> LLVMValueRef {
        val.gen_const(self)
    }

    pub fn cons_str(&self, s: &str) -> LLVMValueRef {
        let c_str = CString::new(s).unwrap();
        unsafe {
            llvm::LLVMConstStringInContext(self.ptr,
                                           c_str.as_ptr(),
                                           c_str.as_bytes().len() as u32,
                                           false as i32)
        }
    }

    pub fn cons_ty<T: ContextType>(&self) -> LLVMTypeRef {
        T::get_type_in_context(self)
    }

    pub fn struct_type(&self,
                       mut element_types: Vec<LLVMTypeRef>,
                       packed: bool) -> LLVMTypeRef {
        unsafe {
            llvm::LLVMStructTypeInContext(self.ptr,
                                          element_types.as_mut_ptr(),
                                          element_types.len() as u32,
                                          packed as i32)
        }
    }
    pub fn struct_type_named(&self, name: &str) -> LLVMTypeRef {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm::LLVMStructCreateNamed(self.ptr, c_name.as_ptr())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            llvm::LLVMContextDispose(self.ptr);
        }
    }
}
