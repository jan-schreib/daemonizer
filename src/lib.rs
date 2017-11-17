extern crate libc;

use std::ffi::CString;

fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}

fn get_guids(name: &str, group: &str) -> (u32, u32) {
    let p = unsafe { libc::getpwnam(CString::new(name).unwrap().as_ptr()) };
    let g = unsafe { libc::getgrnam(CString::new(group).unwrap().as_ptr()) };
    unsafe {
        (
            {
                *g
            }.gr_gid,
            {
                *p
            }.pw_uid,
        )
    }
}

fn set_guid(name: &str, group: &str) -> bool {
    let guids = get_guids(name, group);
    unsafe { !(libc::setgid(guids.0) != 0 || libc::setuid(guids.1) != 0) }
}

/// Daemonizes and privdrops the current process to the
/// user and group supplied as arguments.
///
/// ```
/// use daemonizer::*;
///
/// fn main() {
///     match daemonize("_daemon", "_daemon") {
///         Ok(v) => (),
///         Err(e) => (),
///     }
/// }
/// ```
pub fn daemonize(user: &str, group: &str) -> Result<(), String> {
    if !is_root() {
        Err(
            "Starting this application requires root privileges".to_string(),
        )
    } else if !set_guid(user, group) {
        Err(format!("Unable to set user to {} or {}", user, group))
    } else {
        unsafe {
            if libc::daemon(0, 0) == 0 {
                Ok(())
            } else {
                Err("Error on daemon call".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_guid_to_root() {
        assert_eq!(false, set_guid("root", "wheel"));
    }

    #[test]
    fn test_not_root_daemonize() {
        match daemonize("root", "wheel") {
            Ok(_) => (),
            Err(e) => assert_eq!("Starting this application requires root privileges", e),
        }
    }
}
