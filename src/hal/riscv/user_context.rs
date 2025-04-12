use core::ops::{Index, IndexMut};
use crate::hal::UserContextTrait;
use crate::syscall::ipc_buffer::IPCBuffer;

pub const MAX_GENERAL_REGISTER_NUM: usize = 31;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UserContext {
    registers: [usize; 32],
}

impl Index<UserRegister> for UserContext {
    type Output = usize;

    fn index(&self, index: UserRegister) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<UserRegister> for UserContext {
    fn index_mut(&mut self, index: UserRegister) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl Index<usize> for UserContext {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.registers[index]
    }
}

impl IndexMut<usize> for UserContext {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.registers[index]
    }
}

impl UserContextTrait for UserContext {
    fn new() -> Self {
        Self { registers: [0; 32] }
    }

    fn from_ipc_buffer(buffer: &IPCBuffer) -> &Self {
        unsafe {
            & *(&buffer.msg as *const _ as usize as *const Self)
        }
    }

    fn from_ipc_buffer_mut(buffer: &mut IPCBuffer) -> &mut Self {
        unsafe {
            &mut *(&mut buffer.msg as *mut _ as usize as *mut Self)
        }
    }

    fn set_next_ip(&mut self, value: usize) {
        self[UserRegister::NextIP] = value;
    }

    fn get_next_ip(&self) -> usize {
        self[UserRegister::NextIP]
    }

    fn set_sp(&mut self, value: usize) {
        self[UserRegister::sp] = value;
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum UserRegister {
    ra = 0, sp = 1, gp = 2, tp = 3,

    t0 = 4, t1 = 5, t2 = 6, s0 = 7, s1 = 8,

    /* x10-x17 > a0-a7 */
    a0 = 9, a1 = 10, a2 = 11, a3 = 12,
    a4 = 13, a5 = 14, a6 = 15, a7 = 16, s2 = 17,
    s3 = 18, s4 = 19, s5 = 20, s6 = 21, s7 = 22,
    s8 = 23, s9 = 24, s10 = 25, s11 = 26,

    t3 = 27, t4 = 28, t5 = 29, t6 = 30,
    NextIP = 31,
}