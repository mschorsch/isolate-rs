//
// Inspired by
// https://l3net.wordpress.com/2014/02/01/debian-virtualization-back-to-the-basics-part-3/
//

use nix::sched::*;
use nix::libc::SIGCHLD;
use nix::unistd::{execvp, chdir, sethostname};
use nix::mount::*;
use std::ffi::CString;
use nix::sys::wait::waitpid;

use errors::*;
use errors::Result; // ide hint

use ::Config;

const STACK_SIZE: usize = 1024 * 1024; // 1MB

fn child_worker(config: &Config) -> Result<()> {
    let command = &config.command;
    let initial_dir = &config.initial_dir;
    let readonly_dirs: Vec<&str> = config.readonly_dirs.iter().map(|s| s.as_ref()).collect();
    let tmpfs_dirs: Vec<&str> = config.tmpfs_dirs.iter().map(|s| s.as_ref()).collect();

    // building the chroot filsystem
    mount_all(initial_dir, &readonly_dirs, &tmpfs_dirs)?;

    // run command
    run_command(command)
}

fn mount_all(initial_dir: &str, readonly_dirs: &[&str], tmpfs_dirs: &[&str]) -> Result<()> {
    // mount --make-rslave /
    mount(None::<&str>, "/", None::<&str>, MS_SLAVE | MS_REC, None::<&str>)?;

    // mount readonly
    for dir in readonly_dirs {
        mount_readonly(dir)?;
    }

    // mount temporary
    for dir in tmpfs_dirs {
        mount_tmpfs(dir)?;
    }

    // re-mount /proc in order to have (for example) ps utility working
    remount_proc()?;

    // hostname
    sethostname("container")?;

    chdir(initial_dir).chain_err(|| format!("Can't change dir to '{}'", initial_dir))
}

/// Mount readonly
fn mount_readonly(dir: &str) -> Result<()> {
    // mount --bind /bin /bin
    mount(Some(dir), dir, None::<&str>, MS_BIND | MS_REC, None::<&str>)?;

    // mount --bind -o remount,ro /bin
    mount(None::<&str>, dir, None::<&str>,
          MS_BIND | MS_REMOUNT | MS_RDONLY | MS_REC,
          None::<&str>)?;

    Ok(())
}

/// The contents of this directory will be lost once the bash session is ended.
fn mount_tmpfs(dir: &str) -> Result<()> {
    mount(None::<&str>, dir, Some("tmpfs"), MsFlags::empty(), None::<&str>)
        .chain_err(|| format!("Can't mount tmpfs '{}'", dir))
}

/// Remount /proc
#[allow(unused_must_use)]
fn remount_proc() -> Result<()> {
    umount("/proc");
    mount(Some("proc"), "/proc", Some("proc"), MS_MGC_VAL, None::<&str>)
        .chain_err(|| "Can't remount /proc")
}

fn run_command(cmd: &[CString]) -> Result<()> {
    execvp(&cmd[0],  &cmd[1..])
        .chain_err(|| format!("Unable to run command '{:?}'", &cmd[0]))
        .map(|_| ())
}

pub fn do_clone(config: &Config) -> Result<()> {
    // init stack clone stack
    // TODO allocate on heap or stack???
    let mut stack: Box<[u8]> = vec![0u8; STACK_SIZE].into_boxed_slice(); // heap allocated
    //let mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE]; // stack allocated

    // init callback
    let callback = || {
        if let Err(ref e) = child_worker(config) {
            eprintln!("error: {}", e);

            for e in e.iter().skip(1) {
                eprintln!("caused by: {}", e);
            }

            if let Some(backtrace) = e.backtrace() {
                eprintln!("backtrace: {:?}", backtrace);
            }
        }

        -1 // should never happen
    };

    // clone environment
    let child_pid = clone(Box::new(callback),
                          &mut stack,
                          CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWUTS | CLONE_NEWNET,
                          Some(SIGCHLD)).chain_err(|| format!("Could not exec clone(...)"))?;

    // wait for the child to finish
    waitpid(child_pid, None)?;

    Ok(())
}
