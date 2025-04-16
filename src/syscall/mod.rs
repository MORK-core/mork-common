pub mod message_info;
pub mod ipc_buffer;

#[derive(Debug)]
pub enum Syscall {
    Syscall = -1,
    SysReplyRecv = -2,
    SysSend = -3,
    SysNBSend = -4,
    SysRecv = -5,
    SysReply = -6,
    SysYield = -7,
    SysNBRecv = -8,
    SysDebugPutChar = -9,
    SysDebugShutdown = -10,
}

impl Syscall {
    pub fn from(value: isize) -> Option<Self> {
        match value {
            -1 => Some(Syscall::Syscall),
            -2 => Some(Syscall::SysReplyRecv),
            -3 => Some(Syscall::SysSend),
            -4 => Some(Syscall::SysNBSend),
            -5 => Some(Syscall::SysRecv),
            -6 => Some(Syscall::SysReply),
            -7 => Some(Syscall::SysYield),
            -8 => Some(Syscall::SysNBRecv),
            -9 => Some(Syscall::SysDebugPutChar),
            -10 => Some(Syscall::SysDebugShutdown),
            _ => None,
        }
    }

    pub fn to(&self) -> isize {
        match self {
            Syscall::Syscall => -1,
            Syscall::SysReplyRecv => -2,
            Syscall::SysSend => -3,
            Syscall::SysNBSend => -4,
            Syscall::SysRecv => -5,
            Syscall::SysReply => -6,
            Syscall::SysYield => -7,
            Syscall::SysNBRecv => -8,
            Syscall::SysDebugPutChar => -9,
            Syscall::SysDebugShutdown => -10,
        }
    }
}