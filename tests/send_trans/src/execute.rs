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

use config::{AppConfig, Node};
use report::{CaptainReport, GeneralReport, SimpleReport, SoldierReport};
use std::boxed::Box;
use std::fmt;
use std::marker::{Send, Sync};
use std::ops::Fn;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::{Duration, Instant};

pub struct Mission<T> {
    pub data: T,
    pub doing: Box<Fn(&Node, &T) -> SimpleReport>,
}

unsafe impl<T> Send for Mission<T> {}
unsafe impl<T> Sync for Mission<T> {}

impl<T> Mission<T> {
    pub fn start(&self, node: &Node) -> SimpleReport {
        let ref data = self.data;
        (self.doing)(node, data)
    }
}

impl<T> fmt::Debug for Mission<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A mission.") // TODO
    }
}

#[derive(Debug)]
struct Job<T>
    where T: Send + Sync
{
    mission: Mission<T>,
    countdown: isize,
    start: bool,
    node: Vec<Node>,
    captain_num: usize, // how many teams
    soldier_num: usize, // the size of team
}

impl<T> Job<T>
    where T: Send + Sync
{
    fn new(m: Mission<T>, c: &AppConfig) -> Self {
        let cnum = c.node.len();
        let snum = c.thread;
        Job {
            mission: m,
            countdown: (cnum * snum) as isize,
            start: false,
            node: c.node.clone(),
            captain_num: cnum,
            soldier_num: snum,
        }
    }

    fn run(&self, id: usize) -> SimpleReport {
        self.mission.start(&self.node[id].clone())
    }
}

pub fn run_for_config<T: 'static>(config: AppConfig, mission: Mission<T>) -> GeneralReport
    where T: Send + Sync
{
    debug!("Running for: {}", config);
    let job = Arc::new(RwLock::new(Job::new(mission, &config)));
    let report = general_do_job(job.clone());
    report
}

fn general_do_job<T: 'static>(job: Arc<RwLock<Job<T>>>) -> GeneralReport
    where T: Send + Sync
{
    debug!("General has accepted the mission ...");
    let now = Instant::now();
    let mut captain_team = vec![];
    let captain_num = {
        job.as_ref().read().unwrap().captain_num
    };
    let report = Arc::new(Mutex::new(GeneralReport::new(captain_num)));
    {
        for n in job.as_ref().read().unwrap().node.iter() {
            report.as_ref().lock().unwrap().node.push(n.clone())
        }
    }
    for captain_id in 0..captain_num {
        let job = job.clone();
        let report = report.clone();
        let captain = thread::spawn(move || { captain_do_job(job, report, captain_id); });
        captain_team.push(captain);
    }
    let wait_secs = Duration::from_secs(1);
    loop {
        trace!("General is waiting for soldiers to get ready.");
        thread::sleep(wait_secs);
        let job = job.as_ref().read().unwrap();
        if job.countdown < 1 {
            break;
        }
    }
    {
        debug!("Soldiers are ready to do job.");
        report.as_ref().lock().unwrap().ready_tm = now.elapsed();
        let mut jobrw = job.as_ref().write().unwrap();
        jobrw.start = true;
    }
    for captain in captain_team {
        captain.join().unwrap();
    }
    let mut report_original = Arc::try_unwrap(report).ok().unwrap().into_inner().unwrap();
    report_original.cost_tm = now.elapsed();
    debug!("General has finished the mission.");
    report_original
}

fn captain_do_job<T: 'static>(job: Arc<RwLock<Job<T>>>,
                              report: Arc<Mutex<GeneralReport>>,
                              captain_id: usize)
    where T: Send + Sync
{
    debug!("Captain#{} has accepted the mission ...", captain_id);
    let now = Instant::now();
    let soldier_num = {
        job.as_ref().read().unwrap().soldier_num
    };
    let (tx, rx): (Sender<SoldierReport>, Receiver<SoldierReport>) = channel();
    for soldier_id in 0..soldier_num {
        let job = job.clone();
        let sender = tx.clone();

        thread::spawn(move || { soldier_do_job(job, sender, captain_id, soldier_id); });
    }
    let mut subreport = CaptainReport::new(captain_id, soldier_num);
    subreport.ready_tm = now.elapsed();
    for _ in 0..soldier_num {
        subreport.soldier_report.push(rx.recv().unwrap());
    }
    subreport.cost_tm = now.elapsed();
    {
        report
            .as_ref()
            .lock()
            .unwrap()
            .captain_report
            .push(subreport);
    }
    debug!("Captain#{} has finished the mission.", captain_id);
}

fn soldier_do_job<T>(job: Arc<RwLock<Job<T>>>,
                     sender: Sender<SoldierReport>,
                     captain_id: usize,
                     soldier_id: usize)
    where T: Send + Sync
{
    debug!("Soldier#{}-{} has accepted the mission ...",
           captain_id,
           soldier_id);
    let now = Instant::now();
    {
        let mut jobrw = job.as_ref().write().unwrap();
        jobrw.countdown -= 1;
    }
    let wait_millis = Duration::from_millis(10);
    loop {
        trace!("Soldier#{}-{} is waiting for the order from the General.",
               captain_id,
               soldier_id);
        thread::sleep(wait_millis);
        let job = job.as_ref().read().unwrap();
        if job.start {
            break;
        }
    }
    let ready_tm = now.elapsed();
    let result = {
        job.as_ref().read().unwrap().run(captain_id)
    };
    let cost_tm = now.elapsed();
    {
        sender
            .send(SoldierReport::new(soldier_id, ready_tm, cost_tm, result))
            .unwrap();
    }
    debug!("Soldier#{}-{} has finished the mission.",
           captain_id,
           soldier_id);
}
