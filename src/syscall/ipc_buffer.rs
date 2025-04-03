use crate::constants::{MAX_EXTRA_CAPS, MAX_MESSAGE_LEN};
use crate::syscall::message_info::MessageInfo;

#[repr(C, align(4096))]
pub struct IPCBuffer {
    pub tag: MessageInfo,
    pub msg: [usize; MAX_MESSAGE_LEN],
    pub user_data: usize,
    pub caps_or_badges: [usize; MAX_EXTRA_CAPS],
    pub receive_cnode: usize,
    pub receive_idx: usize,
}