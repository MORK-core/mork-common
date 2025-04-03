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

pub enum InvocationLabel {
    InvalidInvocation = 0,
    AllocObject = 1,
    TCBReadRegisters = 2,
    TCBWriteRegisters = 3,
    TCBCopyRegisters = 4,
    TCBConfigure = 5,
    TCBSetPriority = 6,
    TCBSetIPCBuffer = 9,
    TCBSetSpace = 10,
    TCBSuspend = 11,
    TCBResume = 12,
    TCBSetTLSBase = 15,
    CNodeRevoke = 16,
    CNodeDelete = 17,
    CNodeCopy = 19,
    CNodeMint = 20,
    CNodeMove = 21,
    CNodeSaveCaller = 24,
    IRQIssueIRQHandler = 25,
    IRQAckIRQ = 26,
    IRQSetIRQHandler = 27,
    IRQClearIRQHandler = 28,
    DomainSetSet = 29,
    PageTableMap = 30,
    PageTableUnmap = 31,
    PageMap = 32,
    PageUnmap = 33,
    PageGetAddress = 34,
    NInvocationLabels = 35,
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