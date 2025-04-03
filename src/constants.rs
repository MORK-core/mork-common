use crate::syscall::message_info::MessageInfo;

pub const ROOT_TASK_START: u64 = 0x1000;
pub const MAX_THREAD_PIRO: usize = 256;
pub const MAX_CNODE_SIZE: usize = 240;
pub const MAX_MESSAGE_LEN: usize = 512 - size_of::<MessageInfo>() / size_of::<usize>() - MAX_EXTRA_CAPS - 3;
pub const MAX_EXTRA_CAPS: usize = 3;
pub const NORMAL_PAGE_SIZE: usize = 4096;

pub enum CNodeSlot {
    CapNull = 0,
    CapInitThread,
    CapInitCNode,
    CapInitVSpace,
}

pub enum ObjectType {
    Invalid = 0,
    CNode,
    Thread,
    PageTable,
    Frame,
    NTypes,
}

impl ObjectType {
    pub fn from_usize(label: usize) -> Self {
        assert!(label > ObjectType::Invalid as usize && label < ObjectType::NTypes as usize);
        unsafe {
            core::mem::transmute::<u8, ObjectType>(label as u8)
        }
    }
}

