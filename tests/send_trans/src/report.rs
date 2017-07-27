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

use config::Node;
use std::io::{Write, stdout};
use std::time::Duration;
use tabwriter::TabWriter;

#[derive(Debug)]
pub struct GeneralReport {
    pub ready_tm: Duration,
    pub cost_tm: Duration,
    pub node: Vec<Node>,
    pub captain_report: Vec<CaptainReport>,
}

impl GeneralReport {
    pub fn new(sz: usize) -> Self {
        GeneralReport {
            ready_tm: Duration::new(0, 0),
            cost_tm: Duration::new(0, 0),
            node: Vec::<Node>::with_capacity(sz),
            captain_report: Vec::<CaptainReport>::with_capacity(sz),
        }
    }
    pub fn print(&self) {
        let mut out = TabWriter::new(stdout());
        writeln!(out,
                 "Node\tAmount\tThread\tSuccess\tFailure\tMissing\tSuccCostAvg (ms)")
                .unwrap();
        for crpt in self.captain_report.iter() {
            let rpt = crpt.analyse();
            let ref node = self.node[crpt.captain_id];
            write!(out, "{}:{}\t", node.host, node.port).unwrap();
            write!(out,
                   "{}\t{}\t{}\t{}\t{}\t",
                   rpt.success_cnt + rpt.failure_cnt + rpt.missing_cnt,
                   crpt.soldier_report.len(),
                   rpt.success_cnt,
                   rpt.failure_cnt,
                   rpt.missing_cnt)
                    .unwrap();
            let success_tm = rpt.get_success_tm();
            let success_tm = success_tm.as_secs() as f64 * 1e3 +
                             success_tm.subsec_nanos() as f64 * 1e-6;
            writeln!(out, "{:.6}", success_tm).unwrap();
        }
        writeln!(out, "").unwrap();
        out.flush().unwrap();
    }
}

#[derive(Debug)]
pub struct CaptainReport {
    pub captain_id: usize,
    pub ready_tm: Duration,
    pub cost_tm: Duration,
    pub soldier_report: Vec<SoldierReport>,
}

impl CaptainReport {
    pub fn new(id: usize, sz: usize) -> Self {
        CaptainReport {
            captain_id: id,
            ready_tm: Duration::new(0, 0),
            cost_tm: Duration::new(0, 0),
            soldier_report: Vec::<SoldierReport>::with_capacity(sz),
        }
    }
    pub fn analyse(&self) -> SimpleReport {
        let mut rpt = SimpleReport::new();
        for srpt in self.soldier_report.iter() {
            rpt.add(srpt.success_tm_sum,
                    (srpt.success_cnt, srpt.failure_cnt, srpt.missing_cnt));
        }
        rpt
    }
}

#[derive(Debug)]
pub struct SoldierReport {
    pub soldier_id: usize,
    pub ready_tm: Duration,
    pub cost_tm: Duration,
    pub success_tm_sum: Duration,
    pub success_cnt: usize,
    pub failure_cnt: usize,
    pub missing_cnt: usize,
}

impl SoldierReport {
    pub fn new(id: usize, rt: Duration, ct: Duration, sr: SimpleReport) -> Self {
        SoldierReport {
            soldier_id: id,
            ready_tm: rt,
            cost_tm: ct,
            success_tm_sum: sr.success_tm_sum,
            success_cnt: sr.success_cnt,
            failure_cnt: sr.failure_cnt,
            missing_cnt: sr.missing_cnt,
        }
    }
}

//pub type SimpleReport = (Duration, usize, usize, usize);

#[derive(Debug)]
pub struct SimpleReport {
    pub success_tm_sum: Duration,
    pub success_cnt: usize,
    pub failure_cnt: usize,
    pub missing_cnt: usize,
}

impl SimpleReport {
    pub fn new() -> Self {
        SimpleReport {
            success_tm_sum: Duration::new(0, 0),
            success_cnt: 0,
            failure_cnt: 0,
            missing_cnt: 0,
        }
    }
    pub fn add(&mut self, st: Duration, c: (usize, usize, usize)) {
        self.success_tm_sum += st;
        self.success_cnt += c.0;
        self.failure_cnt += c.1;
        self.missing_cnt += c.2;
    }
    pub fn get_success_tm(&self) -> Duration {
        let cnt = self.success_cnt as u32;
        if cnt == 0 {
            self.success_tm_sum
        } else {
            self.success_tm_sum / cnt
        }
    }
}
