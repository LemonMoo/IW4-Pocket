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

/// iOS has no console and jetsam kills leave no panic: log resident memory every
/// 2 s so the last line before a silent exit shows how much RAM was in use.
#[cfg(target_os = "ios")]
fn spawn_memory_logger() {
    let _ = std::thread::Builder::new().name("mem-log".into()).spawn(|| {
        loop {
            // SAFETY: plain mach call filling a zeroed POD struct of the advertised size.
            let resident_mb = unsafe {
                let mut info: libc::mach_task_basic_info = std::mem::zeroed();
                let mut count = (size_of::<libc::mach_task_basic_info>() / size_of::<u32>())
                    as libc::mach_msg_type_number_t;
                #[allow(deprecated)]
                let task = libc::mach_task_self();
                let kr = libc::task_info(
                    task,
                    libc::MACH_TASK_BASIC_INFO,
                    (&raw mut info).cast(),
                    &raw mut count,
                );
                if kr == 0 { info.resident_size / (1024 * 1024) } else { 0 }
            };
            diag::boot_crumb(&format!("mem: resident {resident_mb} MB"));
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });
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
