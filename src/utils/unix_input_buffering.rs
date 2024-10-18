use nix::sys::select;
use std::fs::File;
use std::io::Error;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::time::Duration;
use termios::{tcgetattr, tcsetattr, Termios, ECHO, ICANON, TCSANOW};

pub fn disable_input_buffering() -> Result<Termios, Error> {
    let stdin_fd = io::stdin().as_raw_fd();

    // Save the original settings globally
    let mut original_tio = Termios::from_fd(stdin_fd)?;

    // Modify the terminal to disable canonical mode and echo
    original_tio.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin_fd, TCSANOW, &original_tio)?;

    Ok(original_tio)
}

pub fn restore_input_buffering(original_tio: Termios) -> Result<(), Error> {
    let stdin_fd = io::stdin().as_raw_fd();
    tcsetattr(stdin_fd, TCSANOW, &original_tio)?;

    Ok(())
}

pub fn check_key() -> bool {
    let stdin_fd: RawFd = io::stdin().as_raw_fd();
    let mut readfds = FdSet::new();
    readfds.insert(stdin_fd);

    let timeout = Duration::from_secs(0);

    let result = select(None, &mut readfds, None, None, Some(timeout));
    result.map_or(false, |count| count > 0)
}
