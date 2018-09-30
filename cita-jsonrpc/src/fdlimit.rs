// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(any(target_os = "linux"))]
pub fn set_fd_limit() {
    use libc;
    use std::io;

    unsafe {
        let mut rlim = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlim) != 0 {
            let err = io::Error::last_os_error();
            panic!("set_fd_limit: error calling getrlimit: {}", err);
        }
        rlim.rlim_cur = rlim.rlim_max;
        if libc::setrlimit(libc::RLIMIT_NOFILE, &rlim) != 0 {
            let err = io::Error::last_os_error();
            panic!("raise_fd_limit: error calling setrlimit: {}", err);
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn set_fd_limit() {
    use libc;
    use std::cmp;
    use std::io;
    use std::mem::size_of_val;
    use std::ptr::null_mut;

    unsafe {
        static KERN_MAXFILESPERPROC: libc::c_int = 29;
        static CTL_KERN: libc::c_int = 1;
        let mut mib: [libc::c_int; 2] = [CTL_KERN, KERN_MAXFILESPERPROC];
        let mut maxfiles: libc::c_int = 0;
        let mut size: libc::size_t = size_of_val(&maxfiles) as libc::size_t;

        if libc::sysctl(
            &mut mib[0],
            2,
            &mut maxfiles as *mut _ as *mut _,
            &mut size,
            null_mut(),
            0,
        ) != 0
        {
            let err = io::Error::last_os_error();
            panic!("set_fd_limit: error calling sysctl: {}", err);
        }

        let mut rlim = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlim) != 0 {
            let err = io::Error::last_os_error();
            panic!("set_fd_limit: error calling getrlimit: {}", err);
        }

        rlim.rlim_cur = cmp::min(maxfiles as libc::rlim_t, rlim.rlim_max);

        if libc::setrlimit(libc::RLIMIT_NOFILE, &rlim) != 0 {
            let err = io::Error::last_os_error();
            panic!("set_fd_limit: error calling setrlimit: {}", err);
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "linux")))]
pub fn set_fd_limit() {}
