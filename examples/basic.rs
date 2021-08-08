mod raw_guard;

#[cfg(feature = "backend-std")]
mod main {
    use std::io::{Read as _, Write as _};
    use std::os::unix::io::AsRawFd as _;

    pub fn run(pty: &mut pty_process::StdPty) {
        let _raw = super::raw_guard::RawGuard::new();
        let mut buf = [0_u8; 4096];
        let pty_fd = pty.as_raw_fd();
        let stdin = std::io::stdin().as_raw_fd();

        loop {
            let mut set = nix::sys::select::FdSet::new();
            set.insert(pty_fd);
            set.insert(stdin);
            match nix::sys::select::select(
                None,
                Some(&mut set),
                None,
                None,
                None,
            ) {
                Ok(n) => {
                    if n > 0 {
                        if set.contains(pty_fd) {
                            match pty.read(&mut buf) {
                                Ok(bytes) => {
                                    let buf = &buf[..bytes];
                                    let stdout = std::io::stdout();
                                    let mut stdout = stdout.lock();
                                    stdout.write_all(buf).unwrap();
                                    stdout.flush().unwrap();
                                }
                                Err(e) => {
                                    // EIO means that the process closed the other
                                    // end of the pty
                                    if e.raw_os_error() != Some(libc::EIO) {
                                        eprintln!("pty read failed: {:?}", e);
                                    }
                                    break;
                                }
                            };
                        }
                        if set.contains(stdin) {
                            match std::io::stdin().read(&mut buf) {
                                Ok(bytes) => {
                                    let buf = &buf[..bytes];
                                    pty.write_all(buf).unwrap();
                                }
                                Err(e) => {
                                    eprintln!("stdin read failed: {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("select failed: {:?}", e);
                    break;
                }
            }
        }
    }
}

#[cfg(feature = "backend-std")]
fn main() {
    use pty_process::Command as _;
    use std::os::unix::process::ExitStatusExt as _;

    let (mut child, mut pty) = std::process::Command::new("sleep")
        .args(&["500"])
        .spawn_pty(Some(&pty_process::Size::new(24, 80)))
        .unwrap();

    main::run(&mut pty);

    let status = child.wait().unwrap();
    std::process::exit(
        status
            .code()
            .unwrap_or_else(|| status.signal().unwrap_or(0) + 128),
    );
}

#[cfg(not(feature = "backend-std"))]
fn main() {
    unimplemented!()
}
