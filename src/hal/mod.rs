use crate::syscall::ipc_buffer::IPCBuffer;

mod riscv;

pub const MAX_GENERAL_REGISTER_NUM: usize = riscv::user_context::MAX_GENERAL_REGISTER_NUM;
pub type UserContext = riscv::user_context::UserContext;

pub trait UserContextTrait {
    fn new() -> Self;
    fn from_ipc_buffer(buffer: &IPCBuffer) -> &Self;
    fn from_ipc_buffer_mut(buffer: &mut IPCBuffer) -> &mut Self;
    fn set_next_ip(&mut self, value: usize);
    fn set_arg0(&mut self, value: usize);
    fn set_arg1(&mut self, value: usize);
    fn get_next_ip(&self) -> usize;
    fn set_sp(&mut self, value: usize);
    fn set_tls(&mut self, value: usize);
}
