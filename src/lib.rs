extern crate libc;
use std::ffi::{CStr, CString};
use std::env;
use libc::*;

pub type LuaState = *mut libc::c_void;

extern {
    fn glua_get_string(state: LuaState, stack_pos: c_int) -> *const c_char;
    
    fn glua_push_global(state: LuaState);
    fn glua_push_string(state: LuaState, string: *const c_char);
    fn glua_push_cfunc(state: LuaState, func: extern fn(LuaState) -> libc::c_int);
    fn glua_set_table(state: LuaState, stack_pos: c_int);
}

pub struct GLuaWrapper {
    state: LuaState
}

impl GLuaWrapper {
    pub fn get_string(&self, stack_pos: i32) -> String {
        unsafe {
            CStr::from_ptr(glua_get_string(self.state, stack_pos)).to_string_lossy().into_owned()
        }
    }
    pub fn push_string(&self, string: &str) {
        unsafe {
            glua_push_string(self.state, CString::new(string).unwrap().as_ptr());
        }
    }
    pub fn set_global(&self, key: &str, func: extern fn(LuaState) -> c_int) {
        unsafe {
            glua_push_global(self.state);
            self.push_string(key);
            glua_push_cfunc(self.state, func);
            glua_set_table(self.state, -3);
        }
    }
}

pub unsafe fn wrap_state(state: LuaState) -> GLuaWrapper {
    GLuaWrapper { state: state }
}

static mut gwrapper: Option<GLuaWrapper> = None;

#[no_mangle]
pub extern fn gmod13_open(state: LuaState) -> c_int {
    unsafe {
        gwrapper = Some(wrap_state(state));
    }
    
    let wrapper = unsafe { gwrapper.as_ref().unwrap() };
    
    extern fn get_env(state: LuaState) -> c_int {
        let wrapper = unsafe { gwrapper.as_ref().unwrap() };
        let key = wrapper.get_string(1);
        
        match env::var(&key) {
            Ok(val) => {
                wrapper.push_string(&val);
                1
            },
            Err(e) => {
                0
            }
        }
    }
    wrapper.set_global("GetEnvVariable", get_env);
    
    0
}

#[no_mangle]
pub extern fn gmod13_close(state: *mut libc::c_void) -> c_int {
    0
}