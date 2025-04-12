#[derive(Default, Clone, Copy, Debug)]
pub struct MessageInfo {
    pub words: [usize; 1],
}

impl MessageInfo {
    pub fn new(label: InvocationLabel, caps_unwrapped: usize, extra_caps: usize, length: usize) -> Self {
        let mut msg = MessageInfo { words: [0; 1]};
        msg.words[0] = 0
            | ((label as usize) & 0xfffffffffffff) << 12
            | (caps_unwrapped & 0x7) << 9
            | (extra_caps & 0x3) << 7
            | (length & 0x7f) << 0;
        msg
    }

    pub fn new_response(label: ResponseLabel) -> Self {
        let mut msg = MessageInfo { words: [0; 1]};
        msg.words[0] = 0 | ((label as usize) & 0xfffffffffffff) << 12;
        msg
    }

    pub fn from_word(word: usize) -> Self {
        let mut msg = MessageInfo { words: [0; 1]};
        msg.words[0] = word;
        msg
    }

    pub fn to_word(&self) -> usize {
        self.words[0]
    }

    pub fn get_label(&self) -> usize {
        Self::sign_extend((self.words[0] & 0xfffffffffffff000) >> 12, 0x0)
    }

    pub fn get_extra_caps(&self) -> usize {
        Self::sign_extend((self.words[0] & 0x180) >> 7, 0x0)
    }

    pub fn get_length(&self) -> usize {
        Self::sign_extend((self.words[0] & 0x7f) >> 0, 0x0)
    }

    fn sign_extend(ret: usize, sign: usize) -> usize {
        if ret & (1 << 63) != 0 {
            return ret | sign;
        }
        ret
    }
}

#[derive(Copy, Clone, Debug)]
pub enum InvocationLabel {
    InvalidInvocation = 0,
    AllocObject,
    TCBReadRegisters,
    TCBWriteRegisters,
    TCBCopyRegisters,
    TCBConfigure,
    TCBSetPriority,
    TCBSetIPCBuffer,
    TCBSetSpace,
    TCBSuspend,
    TCBResume,
    TCBSetTLSBase,
    CNodeRevoke,
    CNodeDelete,
    CNodeCopy,
    CNodeMint,
    CNodeMove,
    CNodeSaveCaller,
    IRQIssueIRQHandler,
    IRQAckIRQ,
    IRQSetIRQHandler,
    IRQClearIRQHandler,
    DomainSetSet,
    PageTableMap,
    PageTableUnmap,
    PageMap,
    PageUnmap,
    PageGetAddress,
    NInvocationLabels
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ResponseLabel {
    Success = InvocationLabel::NInvocationLabels as isize,
    NotEnoughSpace,
    ErrCapType,
    PageTableMiss,
    MappedAlready,
    OutOfRange,
    InvalidParam,
    NoIpcBuffer,
    UnSupported,
    UnknownFailure,
}

impl InvocationLabel {
    pub fn from_usize(label: usize) -> Self {
        assert!(label >= InvocationLabel::InvalidInvocation as usize && label < InvocationLabel::NInvocationLabels as usize);
        unsafe {
            core::mem::transmute::<u8, InvocationLabel>(label as u8)
        }
    }
}

impl ResponseLabel {
    pub fn from_usize(label: usize) -> Self {
        assert!(label >= ResponseLabel::Success as usize && label < ResponseLabel::UnknownFailure as usize);
        unsafe {
            core::mem::transmute::<u8, ResponseLabel>(label as u8)
        }
    }
}