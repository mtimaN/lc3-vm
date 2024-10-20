use crossterm::event::poll;
use std::io;
use std::io::Error;
use std::os::unix::io::AsRawFd;
use std::time::Duration;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

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
    let timeout = Duration::from_secs(0);
    return poll(timeout).expect("Poll failed");
}
