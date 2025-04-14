use crate::syscall::message_info::MessageInfo;

pub const MAX_THREAD_PIRO: usize = 256;
pub const MAX_CNODE_SIZE: usize = 240;
pub const MAX_MESSAGE_LEN: usize = 512 - size_of::<MessageInfo>() / size_of::<usize>() - MAX_EXTRA_CAPS - 3;
pub const MAX_EXTRA_CAPS: usize = 3;
pub const PAGE_SIZE_NORMAL: usize = 4096;
pub const PAGE_SIZE_2M: usize = 1 << 21;
pub enum CNodeSlot {
    CapNull = 0,
    CapInitThread,
    CapInitVSpace,
}

pub enum ObjectType {
    Invalid = 0,
    CNode,
    Thread,
    PageTable,
    Frame4K,
    Frame2M,
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

