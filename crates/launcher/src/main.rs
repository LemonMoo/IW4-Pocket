use std::path::PathBuf;

use asset_transport::{ensure_artifacts_dir, games_root_from_env};

#[global_allocator]
static PROCESS_ALLOCATOR: diag::ProcessCountingAllocator = diag::ProcessCountingAllocator;

const LICENSES: [(&str, &str); 5] = [
    ("LICENSE", include_str!("../../../LICENSE")),
    ("NOTICE", include_str!("../../../NOTICE")),
    (
        "OFL-Oxanium.txt",
        include_str!("../../ui/assets/OFL-Oxanium.txt"),
    ),
    (
        "OFL-FiraMono.txt",
        include_str!("../../console/assets/OFL-FiraMono.txt"),
    ),
    (
        "THIRD-PARTY-LICENSES.txt",
        include_str!("../../../THIRD-PARTY-LICENSES.txt"),
    ),
];

/// iOS sandbox: everything lives in the app's Documents folder, which the Files app
/// can reach (UIFileSharingEnabled). Game data goes in Documents/Games.
#[cfg(target_os = "ios")]
fn prepare_ios_sandbox() {
    let Some(home) = std::env::var_os("HOME").map(PathBuf::from) else {
        return;
    };
    let docs = home.join("Documents");
    let games = docs.join("Games");
    let artifacts = docs.join("iw4l-artifacts");
    let config = home.join("Library").join("Application Support");
    for dir in [&games, &artifacts, &config] {
        let _ = std::fs::create_dir_all(dir);
    }
    // SAFETY: called first thing in main, before any thread is spawned.
    unsafe {
        if std::env::var_os("IW4L_GAMES").is_none() {
            std::env::set_var("IW4L_GAMES", &games);
        }
        if std::env::var_os("IW4L_ARTIFACTS_DIR").is_none() {
            std::env::set_var("IW4L_ARTIFACTS_DIR", &artifacts);
        }
        std::env::set_var("XDG_CONFIG_HOME", &config);
    }
}

/// iOS has no console and jetsam kills leave no panic. Sample memory 4x/s and log on
/// every 40 MB change (or every 2 s): the last line before a silent exit shows the
/// footprint and how many bytes iOS still allowed this process (os_proc_available_memory).
#[cfg(target_os = "ios")]
unsafe extern "C" {
    fn os_proc_available_memory() -> usize;
}

/// task_vm_info up to `phys_footprint`, the number iOS actually enforces (it includes
/// GPU-backed and compressed memory that `resident_size` misses).
#[cfg(target_os = "ios")]
#[repr(C)]
#[derive(Default)]
struct TaskVmInfo {
    virtual_size: u64,
    region_count: i32,
    page_size: i32,
    resident_size: u64,
    resident_size_peak: u64,
    device: u64,
    device_peak: u64,
    internal: u64,
    internal_peak: u64,
    external: u64,
    external_peak: u64,
    reusable: u64,
    reusable_peak: u64,
    purgeable_volatile_pmap: u64,
    purgeable_volatile_resident: u64,
    purgeable_volatile_virtual: u64,
    compressed: u64,
    compressed_peak: u64,
    compressed_lifetime: u64,
    phys_footprint: u64,
}

#[cfg(target_os = "ios")]
fn footprint_mb() -> u64 {
    const TASK_VM_INFO: libc::c_int = 22;
    let mut info = TaskVmInfo::default();
    let mut count = (size_of::<TaskVmInfo>() / size_of::<u32>()) as libc::mach_msg_type_number_t;
    // SAFETY: the struct mirrors the kernel's task_vm_info prefix; count bounds the write.
    #[allow(deprecated)]
    let kr = unsafe {
        libc::task_info(
            libc::mach_task_self(),
            TASK_VM_INFO as _,
            (&raw mut info).cast(),
            &raw mut count,
        )
    };
    if kr == 0 { info.phys_footprint / (1024 * 1024) } else { 0 }
}

/// iOS has no console and jetsam kills leave no panic. Sample 4x/s and log on every 40 MB
/// change (or every 2 s): footprint (what iOS enforces, GPU included), what iOS still
/// allows, and the RGBA texture bytes sent to the GPU. The last line before a silent exit
/// shows how much memory was left.
#[cfg(target_os = "ios")]
fn spawn_memory_logger() {
    let _ = std::thread::Builder::new().name("mem-log".into()).spawn(|| {
        let mut last_mb = 0u64;
        let mut last_log = std::time::Instant::now();
        loop {
            let footprint = footprint_mb();
            let changed = footprint.abs_diff(last_mb) >= 40;
            if changed || last_log.elapsed() >= std::time::Duration::from_secs(2) {
                // SAFETY: no arguments, returns a byte count (iOS 13+).
                let available_mb = unsafe { os_proc_available_memory() } / (1024 * 1024);
                let tex_mb =
                    diag::IOS_TEXTURE_BYTES.load(std::sync::atomic::Ordering::Relaxed) / (1024 * 1024);
                diag::boot_crumb(&format!(
                    "mem: footprint {footprint} MB, iOS still allows {available_mb} MB, textures sent {tex_mb} MB"
                ));
                last_mb = footprint;
                last_log = std::time::Instant::now();
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    });
}

/// A fatal signal (SIGSEGV, SIGABRT, ...) is not a panic. Log which one, then die as usual.
/// SIGKILL cannot be caught: if the log just stops, that is iOS killing the app for memory.
#[cfg(target_os = "ios")]
extern "C" fn fatal_signal(signal: libc::c_int) {
    diag::boot_crumb(&format!(
        "FATAL SIGNAL {signal} (11=SEGV 6=ABRT 10=BUS 4=ILL 5=TRAP)\n{}",
        std::backtrace::Backtrace::force_capture()
    ));
    // SAFETY: restore the default action and re-raise so iOS still gets its crash report.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        libc::raise(signal);
    }
}

#[cfg(target_os = "ios")]
fn install_signal_logging() {
    for signal in [libc::SIGSEGV, libc::SIGABRT, libc::SIGBUS, libc::SIGILL, libc::SIGTRAP] {
        // SAFETY: installing a plain C handler at startup.
        unsafe {
            libc::signal(signal, fatal_signal as extern "C" fn(libc::c_int) as libc::sighandler_t);
        }
    }
}

fn main() {
    #[cfg(target_os = "ios")]
    {
        prepare_ios_sandbox();
        let _ = std::fs::remove_file(
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default())
                .join("Documents")
                .join("iw4l-boot.log"),
        );
        diag::boot_crumb("1 main entered, sandbox ready");
        install_signal_logging();
        spawn_memory_logger();
        std::panic::set_hook(Box::new(|info| {
            let thread = std::thread::current();
            diag::boot_crumb(&format!(
                "PANIC on thread {:?}: {info}\n{}",
                thread.name(),
                std::backtrace::Backtrace::force_capture()
            ));
        }));
    }
    if std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == "licenses")
    {
        for (name, text) in LICENSES {
            println!("==> {name} <==\n\n{text}\n");
        }
        return;
    }
    #[cfg(target_os = "ios")]
    diag::boot_crumb("2 updater::startup");
    let mut args = match updater::startup().unwrap_or_else(|e| diag::exit_launch_error(&e)) {
        Some(args) => args,
        None => return,
    };
    if args.is_empty() {
        args.push("menu".into());
    }
    #[cfg(target_os = "ios")]
    diag::boot_crumb("3 after updater");
    bootstrap::bench::arm();
    prepare_process_root().unwrap_or_else(|e| {
        diag::exit_launch_error(&e);
    });
    let artifacts = ensure_artifacts_dir().unwrap_or_else(|e| diag::exit_launch_error(&e));
    announce_log(diag::init_log(&artifacts));
    let (mode, acceptance, cheats) = bootstrap::parse_cli(
        args.into_iter()
            .map(|arg| arg.to_string_lossy().into_owned()),
    )
    .unwrap_or_else(|e| diag::exit_launch_error(&e));
    let games = games_root_from_env().unwrap_or_else(|e| diag::exit_launch_error(&e));
    #[cfg(target_os = "ios")]
    diag::boot_crumb("4 calling bootstrap::launch");
    bootstrap::launch(games, artifacts, mode, acceptance, cheats);
}

fn prepare_process_root() -> Result<(), String> {
    #[cfg(windows)]
    {
        let exe =
            std::env::current_exe().map_err(|error| format!("cannot locate iw4l.exe: {error}"))?;
        let root = exe
            .parent()
            .ok_or_else(|| format!("iw4l.exe has no parent directory: {}", exe.display()))?;
        std::env::set_current_dir(root).map_err(|error| {
            format!(
                "cannot enter launcher directory {}: {error}",
                root.display()
            )
        })?;
    }
    Ok(())
}

fn announce_log(path: PathBuf) {
    diag::announce_log_stdout(&path, diag::latest_log_path().as_deref());
    diag::info!(Launch, "log: {}", path.display());
}
