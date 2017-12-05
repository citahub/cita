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

use std::fs::{read_dir, DirBuilder, File, OpenOptions};
use std::io;
use std::io::{Read, Seek, Write};
use std::mem::transmute;
//use std::path::{PathBuf, Path};
use std::path::Path;
use std::str;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Wal {
    fs: File,
    dir: String,
}

#[allow(dead_code)]
impl Wal {
    pub fn new(dir: &str) -> Result<Wal, io::Error> {
        let fss = read_dir(&dir);
        if fss.is_err() {
            DirBuilder::new().recursive(true).create(dir).unwrap();
        }

        let fpath = dir.to_string() + "/authorities_old";
        let big_path = Path::new(&*fpath).to_path_buf();
        let fs = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(big_path)?;
        Ok(Wal {
            fs: fs,
            dir: dir.to_string(),
        })
    }

    pub fn save(&mut self, mtype: u8, msg: &Vec<u8>) -> io::Result<usize> {
        let mlen = msg.len() as u32;
        if mlen == 0 {
            return Ok(0);
        }
        self.fs.set_len(0)?;
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
            let hd: [u8; 4] = [
                vec_buf[index],
                vec_buf[index + 1],
                vec_buf[index + 2],
                vec_buf[index + 3],
            ];
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
