//! Fail-closed check for the x86-64 microarchitecture level the system was built for.
//!
//! User space built for x86-64-v3 or v4 dies with #UD on the first AVX2 or AVX-512
//! instruction on an older CPU, usually far into boot and with no useful message. Setting
//! `[arch.x86_64] level` in config.toml makes the kernel check the CPU against that level
//! before it does anything else, and stop with a list of what is missing.

use crate::arch::cpuid::cpuid;

/// The level from config.toml, if any (see build.rs).
const LEVEL: Option<u8> = if cfg!(x86_64_level = "v4") {
    Some(4)
} else if cfg!(x86_64_level = "v3") {
    Some(3)
} else if cfg!(x86_64_level = "v2") {
    Some(2)
} else {
    None
};

/// Features the psABI requires at each level, beyond the baseline.
fn missing_features(level: u8, missing: &mut impl FnMut(&'static str)) {
    let cpuid = cpuid();
    let basic = cpuid.get_feature_info();
    let ext = cpuid.get_extended_feature_info();
    let ext_proc = cpuid.get_extended_processor_and_feature_identifiers();

    let mut need = |name: &'static str, present: bool| {
        if !present {
            missing(name);
        }
    };

    if level >= 2 {
        let b = basic.as_ref();
        need("cmpxchg16b", b.is_some_and(|f| f.has_cmpxchg16b()));
        need("popcnt", b.is_some_and(|f| f.has_popcnt()));
        need("sse3", b.is_some_and(|f| f.has_sse3()));
        need("ssse3", b.is_some_and(|f| f.has_ssse3()));
        need("sse4.1", b.is_some_and(|f| f.has_sse41()));
        need("sse4.2", b.is_some_and(|f| f.has_sse42()));
        need("lahf_lm", ext_proc.as_ref().is_some_and(|f| f.has_lahf_sahf()));
    }
    if level >= 3 {
        let b = basic.as_ref();
        let e = ext.as_ref();
        need("avx", b.is_some_and(|f| f.has_avx()));
        need("avx2", e.is_some_and(|f| f.has_avx2()));
        need("bmi1", e.is_some_and(|f| f.has_bmi1()));
        need("bmi2", e.is_some_and(|f| f.has_bmi2()));
        need("f16c", b.is_some_and(|f| f.has_f16c()));
        need("fma", b.is_some_and(|f| f.has_fma()));
        need("lzcnt", ext_proc.as_ref().is_some_and(|f| f.has_lzcnt()));
        need("movbe", b.is_some_and(|f| f.has_movbe()));
        need("xsave", b.is_some_and(|f| f.has_xsave()));
    }
    if level >= 4 {
        let e = ext.as_ref();
        need("avx512f", e.is_some_and(|f| f.has_avx512f()));
        need("avx512bw", e.is_some_and(|f| f.has_avx512bw()));
        need("avx512cd", e.is_some_and(|f| f.has_avx512cd()));
        need("avx512dq", e.is_some_and(|f| f.has_avx512dq()));
        need("avx512vl", e.is_some_and(|f| f.has_avx512vl()));
    }
}

/// Halt with a readable message if this CPU is below the configured level.
///
/// Called on the BSP right after the serial console comes up, before anything that could
/// depend on the missing features.
pub fn check() {
    let Some(level) = LEVEL else {
        return;
    };

    let mut count = 0usize;
    missing_features(level, &mut |name| {
        if count == 0 {
            error!("This CPU does not meet x86-64-v{} required by this build.", level);
        }
        error!("  missing CPU feature: {}", name);
        count = count.saturating_add(1);
    });

    if count == 0 {
        info!("CPU meets x86-64-v{}", level);
        return;
    }
    error!(
        "Rebuild with a lower cpu.x86_64_level in ferric.toml, or boot on a newer CPU. Halting."
    );
    loop {
        // SAFETY: interrupts are still disabled this early, so this halts the BSP for good.
        unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)) };
    }
}
