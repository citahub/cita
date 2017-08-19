// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

use std::ffi::OsString;
use std::fs::{File, read_dir, OpenOptions, DirBuilder};
use std::io;
use std::io::{Write, Read, Seek};
use std::mem::transmute;
use std::path::{PathBuf, Path};
use std::str;

pub struct Wal {
    fs: File,
    dir: String,
}

impl Wal {
    pub fn new(dir: &str) -> Result<Wal, io::Error> {
        let mut tmp = OsString::new();
        let mut big_path = PathBuf::new();

        let mut fss = read_dir(&dir);
        if fss.is_err() {
            DirBuilder::new().recursive(true).create(dir).unwrap();
            fss = read_dir(&dir);
        }
        let mut fnum = 0;

        for dir_entry in fss? {
            let entry = dir_entry?;
            let epath = entry.path();
            if let Some(fname) = epath.clone().file_stem() {
                if tmp.len() < fname.len() || (tmp.len() == fname.len() && tmp.as_os_str() < fname) {
                    tmp = fname.to_os_string();
                    big_path = epath;
                    fnum += 1;
                }
            }
        }

        if fnum == 0 {
            tmp = OsString::from("1");
            let fpath = dir.to_string() + "/1.log";
            big_path = Path::new(&*fpath).to_path_buf();
        }

        let fs = OpenOptions::new().read(true).create(true).write(true).open(big_path)?;
        let hstr = tmp.into_string().unwrap();
        let hi = hstr.parse::<u32>();
        match hi {
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::Other, "not number file name"));
            }
            Ok(_) => {}
        }

        Ok(Wal { fs: fs, dir: dir.to_string() })
    }

    pub fn set_height(&mut self, height: usize) -> Result<(), io::Error> {
        let mut name = height.to_string();
        name = name + ".log";

        let pathname = self.dir.clone() + "/";
        let filename = pathname.clone() + &*name;
        self.fs = OpenOptions::new().create(true).read(true).write(true).open(filename)?;

        if height > 2 {
            let mut delname = (height - 2).to_string();
            delname = delname + ".log";
            let delfilename = pathname + &*delname;
            let _ = ::std::fs::remove_file(delfilename);
        }
        Ok(())
    }

    pub fn save(&mut self, mtype: u8, msg: &Vec<u8>) -> io::Result<usize> {
        let mlen = msg.len() as u32;
        if mlen == 0 {
            return Ok(0);
        }
        let len_bytes: [u8; 4] = unsafe { transmute(mlen.to_le()) };
        let type_bytes: [u8; 1] = unsafe { transmute(mtype.to_le()) };
        self.fs.seek(io::SeekFrom::End(0))?;
        self.fs.write(&len_bytes[..])?;
        self.fs.write(&type_bytes[..])?;
        let hlen = self.fs.write(msg.as_slice())?;
        self.fs.flush()?;
        Ok(hlen)
    }

    pub fn load(&mut self) -> Vec<(u8, Vec<u8>)> {
        let mut vec_buf: Vec<u8> = Vec::new();
        let mut vec_out: Vec<(u8, Vec<u8>)> = Vec::new();

        self.fs.seek(io::SeekFrom::Start(0)).unwrap();
        let res_fsize = self.fs.read_to_end(&mut vec_buf);
        if res_fsize.is_err() {
            return vec_out;
        }
        let fsize = res_fsize.unwrap();
        if fsize <= 5 {
            return vec_out;
        }
        let mut index = 0;
        loop {

            if index + 5 > fsize {
                break;
            }
            let hd: [u8; 4] = [vec_buf[index], vec_buf[index + 1], vec_buf[index + 2], vec_buf[index + 3]];
            let tmp: u32 = unsafe { transmute::<[u8; 4], u32>(hd) };
            let bodylen = tmp as usize;
            let mtype = vec_buf[index + 4];
            index += 5;
            if index + bodylen > fsize {
                break;
            }
            vec_out.push((mtype, vec_buf[index..index + bodylen].to_vec()));
            index += bodylen;
        }
        vec_out
    }
}
