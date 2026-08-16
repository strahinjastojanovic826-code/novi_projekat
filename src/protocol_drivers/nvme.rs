#[derive(Debug, Clone, PartialEq)]
pub enum NvmeOpcode {
    Identify = 0x06,
    Read = 0x02,
    Write = 0x01,
}

#[derive(Debug, Clone)]
pub struct NvmeCommand {
    pub command_id: u16,
    pub opcode: NvmeOpcode,
    pub nsid: u32,       // Namespace ID (npr. 1)
    pub lba: u64,        // Logical Block Address
    pub block_count: u16,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct NvmeCompletion {
    pub command_id: u16,
    pub status_code: u16, // 0 = Success
    pub sq_head: u16,
}

pub struct NvmeController {
    pub model: String,
    pub firmware: String,
    pub sq_tail: u16,
    pub cq_head: u16,
    pub submission_queue: Vec<NvmeCommand>,
    pub completion_queue: Vec<NvmeCompletion>,
}

impl NvmeController {
    pub fn new() -> Self {
        Self {
            model: "Quantum NVMe PCIe Gen4 x4 SSD 1TB".into(),
            firmware: "QFW-1092-PRO".into(),
            sq_tail: 0,
            cq_head: 0,
            submission_queue: Vec::new(),
            completion_queue: Vec::new(),
        }
    }

    pub fn submit_command(&mut self, opcode: NvmeOpcode, lba: u64, payload: String) -> NvmeCompletion {
        let cmd_id = self.sq_tail;
        self.sq_tail += 1;

        let cmd = NvmeCommand {
            command_id: cmd_id,
            opcode,
            nsid: 1,
            lba,
            block_count: 1,
            payload,
        };

        self.submission_queue.push(cmd);

        let completion = NvmeCompletion {
            command_id: cmd_id,
            status_code: 0, // Success
            sq_head: self.sq_tail,
        };

        self.completion_queue.push(completion.clone());
        self.cq_head += 1;

        completion
    }
}