use core::sync::atomic::{AtomicU64, Ordering};

// --- 1. RAFT CRASH FAULT TOLERANCE (CFT) ENGINE ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RaftRole {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub term: u64,
    pub index: u64,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct VoteRequest {
    pub term: u64,
    pub candidate_id: u64,
    pub last_log_index: u64,
    pub last_log_term: u64,
}

#[derive(Debug, Clone)]
pub struct VoteResponse {
    pub term: u64,
    pub vote_granted: bool,
}

pub struct RaftNode {
    pub node_id: u64,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub role: RaftRole,
    pub log: Vec<LogEntry>,
    pub commit_index: u64,
    pub last_applied: u64,
    pub votes_received: usize,
    pub total_nodes: usize,
}

impl RaftNode {
    pub fn new(node_id: u64, total_nodes: usize) -> Self {
        Self {
            node_id,
            current_term: 0,
            voted_for: None,
            role: RaftRole::Follower,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            votes_received: 0,
            total_nodes,
        }
    }

    /// Pokreće izbor za Lidera (Prelazak u Candidate stanje)
    pub fn start_election(&mut self) -> VoteRequest {
        self.role = RaftRole::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.node_id);
        self.votes_received = 1; // Glasa za samog sebe

        let last_log_index = self.log.len() as u64;
        let last_log_term = self.log.last().map_or(0, |entry| entry.term);

        VoteRequest {
            term: self.current_term,
            candidate_id: self.node_id,
            last_log_index,
            last_log_term,
        }
    }

    /// Obrađuje zahtev za glas od drugog čvora
    pub fn handle_vote_request(&mut self, req: &VoteRequest) -> VoteResponse {
        if req.term > self.current_term {
            self.current_term = req.term;
            self.role = RaftRole::Follower;
            self.voted_for = None;
        }

        let my_last_index = self.log.len() as u64;
        let my_last_term = self.log.last().map_or(0, |entry| entry.term);

        let log_ok = req.last_log_term > my_last_term 
            || (req.last_log_term == my_last_term && req.last_log_index >= my_last_index);

        let can_vote = (self.voted_for.is_none() || self.voted_for == Some(req.candidate_id)) && log_ok;

        if req.term == self.current_term && can_vote {
            self.voted_for = Some(req.candidate_id);
            VoteResponse { term: self.current_term, vote_granted: true }
        } else {
            VoteResponse { term: self.current_term, vote_granted: false }
        }
    }

    /// Obrađuje odgovor na glas i proverava kvorum
    pub fn handle_vote_response(&mut self, resp: VoteResponse) -> bool {
        if self.role == RaftRole::Candidate && resp.term == self.current_term && resp.vote_granted {
            self.votes_received += 1;
            let quorum = (self.total_nodes / 2) + 1;
            if self.votes_received >= quorum {
                self.role = RaftRole::Leader;
                return true; // Postao Lider!
            }
        }
        false
    }

    /// Dodaje komandu u dnevnik (Samo ako je Lider)
    pub fn append_command(&mut self, cmd: String) -> Result<u64, &'static str> {
        if self.role != RaftRole::Leader {
            return Err("Not a leader: Samo Raft Lider može upisivati u dnevnik!");
        }

        let entry = LogEntry {
            term: self.current_term,
            index: (self.log.len() + 1) as u64,
            command: cmd,
        };
        let idx = entry.index;
        self.log.push(entry);
        Ok(idx)
    }
}

// --- 2. PRACTICAL BYZANTINE FAULT TOLERANCE (PBFT) ENGINE ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PbftPhase {
    Idle,
    PrePrepare,
    Prepare,
    Commit,
    Committed,
}

#[derive(Debug, Clone)]
pub struct PbftMessage {
    pub view: u64,
    pub sequence: u64,
    pub digest: u64,
    pub sender_id: u64,
    pub phase: PbftPhase,
    pub is_byzantine_malicious: bool,
}

pub struct PbftNode {
    pub node_id: u64,
    pub view: u64,
    pub sequence: u64,
    pub phase: PbftPhase,
    pub prepare_votes: usize,
    pub commit_votes: usize,
    pub total_nodes: usize, // N = 3f + 1
    pub max_faulty_nodes: usize, // f
    pub is_byzantine: bool,
}

impl PbftNode {
    pub fn new(node_id: u64, total_nodes: usize, is_byzantine: bool) -> Self {
        let f = (total_nodes.saturating_sub(1)) / 3;
        Self {
            node_id,
            view: 0,
            sequence: 0,
            phase: PbftPhase::Idle,
            prepare_votes: 0,
            commit_votes: 0,
            total_nodes,
            max_faulty_nodes: f,
            is_byzantine,
        }
    }

    /// Faza 1: Primary čvor emituje Pre-Prepare poruku
    pub fn primary_pre_prepare(&mut self, seq: u64, data: &str) -> PbftMessage {
        self.sequence = seq;
        self.phase = PbftPhase::PrePrepare;
        
        let mut digest = 0u64;
        for b in data.bytes() {
            digest = digest.wrapping_add(b as u64);
        }

        PbftMessage {
            view: self.view,
            sequence: seq,
            digest: if self.is_byzantine { digest ^ 0xDEADBEEF } else { digest },
            sender_id: self.node_id,
            phase: PbftPhase::PrePrepare,
            is_byzantine_malicious: self.is_byzantine,
        }
    }

    /// Faza 2: Obrada Prepare poruka i kvorum 2f + 1
    pub fn handle_prepare(&mut self, msg: &PbftMessage) -> Result<Option<PbftMessage>, &'static str> {
        if msg.is_byzantine_malicious {
            return Err("PBFT WARN: Odbijena nevažeća/zlonamerna poruka od strane Byzantine čvora!");
        }

        self.prepare_votes += 1;
        let quorum = 2 * self.max_faulty_nodes + 1;

        if self.prepare_votes >= quorum && self.phase != PbftPhase::Prepare {
            self.phase = PbftPhase::Prepare;
            Ok(Some(PbftMessage {
                view: self.view,
                sequence: self.sequence,
                digest: msg.digest,
                sender_id: self.node_id,
                phase: PbftPhase::Prepare,
                is_byzantine_malicious: false,
            }))
        } else {
            Ok(None)
        }
    }

    /// Faza 3: Obrada Commit poruka i finalizacija
    pub fn handle_commit(&mut self, msg: &PbftMessage) -> Result<bool, &'static str> {
        if msg.phase != PbftPhase::Prepare && msg.phase != PbftPhase::Commit {
            return Err("PBFT FAULT: Pogrešan redosled PBFT faza!");
        }

        self.commit_votes += 1;
        let quorum = 2 * self.max_faulty_nodes + 1;

        if self.commit_votes >= quorum {
            self.phase = PbftPhase::Committed;
            Ok(true) // Stanje je uspešno transponovano na BFT distribuiranu mrežu
        } else {
            Ok(false)
        }
    }
}