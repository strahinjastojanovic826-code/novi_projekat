use std::sync::atomic::AtomicUsize;

pub static ZERO_DIV_TRAPS_CAUGHT: AtomicUsize = AtomicUsize::new(0);

// =============================================================================
// 1. SIGNAL HANDLER ASSEMBLY HACK (POSIX / x86_64)
// =============================================================================

#[cfg(unix)]
pub unsafe extern "C" fn sigfpe_asm_hack_handler(
    _sig: libc::c_int,
    _info: *mut libc::siginfo_t,
    uctx: *mut libc::c_void,
) {
    let context = uctx as *mut libc::ucontext_t;

    // Registrujemo da je stao hardverski trap
    ZERO_DIV_TRAPS_CAUGHT.fetch_add(1, Ordering::Relaxed);

    // Pristupamo registrima procesora u trenutku prekida (x86_64 gregs)
    let mcontext = &mut (*context).uc_mcontext;

    // Setujemo RAX (rezultat deljenja) na 0 ili posebnu vrednost (npr. 0 ili MAX)
    mcontext.gregs[libc::REG_RAX as usize] = 0;

    // RDX čuva ostatak pri deljenju - postavljamo ga na 0
    mcontext.gregs[libc::REG_RDX as usize] = 0;

    // KLJUČNI HACK: Moramo pomeriti RIP (Instruction Pointer) preko IDIV instrukcije!
    // x86_64 'idiv rbx' ili 'idiv rcx' obično zauzima 3 bajta (0x48, 0xF7, 0xFB/F9)
    mcontext.gregs[libc::REG_RIP as usize] += 3;
}

pub fn setup_sigfpe_handler() {
    #[cfg(unix)]
    unsafe {
        let mut sa: libc::sigaction = std::mem::zeroed();
        sa.sa_sigaction = sigfpe_asm_hack_handler as usize;
        sa.sa_flags = libc::SA_SIGINFO | libc::SA_NODEFER;
        libc::sigemptyset(&mut sa.sa_mask);
        libc::sigaction(libc::SIGFPE, &sa, std::ptr::null_mut());
    }
}

// =============================================================================
// 2. ALTERNATIVE: PURE BITWISE BRANCHLESS DIVISION (NO IF, NO SIGNALS)
// =============================================================================

pub struct BranchlessMath;

impl BranchlessMath {
    /// Deljenje bez `if` grane koristeći bitwise maskiranje (Multiplexer u ALU)
    /// Ako je `b == 0`, pretvara `b` u `1` bez ijedne conditional branch instrukcije!
    #[inline(always)]
    pub fn safe_div_bitwise(a: u64, b: u64) -> u64 {
        // Pretvara b == 0 u masku (1 ako je 0, 0 ako nije 0)
        let is_zero = (b == 0) as u64; // Koristi SETE/SETZ instrukciju bez skoka
        
        // Ako je b == 0, safe_b postaje 1. Ako je b != 0, safe_b ostaje b.
        let safe_b = b + is_zero;
        
        // Maska za nuliranje rezultata ako je originalni b bio 0
        let zero_mask = 0u64.wrapping_sub(1 - is_zero);

        (a / safe_b) & zero_mask
    }
}