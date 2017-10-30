use config::{MonitorConfig, ProcessConfig};
use std::collections::HashMap;
use std::default::Default;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::remove_file;
use std::io::{BufReader, Read};
use std::io::Write;
use std::process::Child;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;


#[derive(Debug, Default)]
pub struct Processes {
    pub processcfg: ProcessConfig,
    pub processhandle: Option<Child>,
    pub children: HashMap<String, Arc<Mutex<Processes>>>,
}

impl Processes {
    pub fn new(monitorconfig: MonitorConfig) -> Self {
        let parentcfg = ProcessConfig {
            name: monitorconfig.name.clone(),
            command: monitorconfig.command.clone(),
            args: monitorconfig.args.clone(),
            pidfile: monitorconfig.pidfile.clone(),
            logfile: monitorconfig.logfile.clone(),
            errfile: monitorconfig.errfile.clone(),
            ..Default::default()
        };

        let processcfg = monitorconfig.process.unwrap();
        let mut children_processes = HashMap::new();

        for cfg in &processcfg {
            let child_name = cfg.name.clone().unwrap();
            let child_cfg = cfg.clone();

            let child_inner_process = HashMap::new();
            let child_handle = None;
            let child_process = Processes {
                processcfg: child_cfg,
                processhandle: child_handle,
                children: child_inner_process,
            };
            children_processes.insert(child_name, Arc::new(Mutex::new(child_process)));
        }

        Processes {
            processcfg: parentcfg,
            processhandle: None,
            children: children_processes,
        }
    }

    //find child process
    pub fn find_process(&mut self) -> Option<u32> {

        if self.processcfg.pidfile == None {
            let name = self.processcfg.name.clone().unwrap();
            warn!("{} pidfile path is null", name);
            return None;
        }

        let pidfile_clone = self.processcfg.pidfile.clone();
        let pidfile = pidfile_clone.unwrap();
        let is_exist = check_process(pidfile);
        is_exist
    }

    //start  parent process
    pub fn start(&mut self) {

        let command = self.processcfg.command.clone().unwrap();
        let arg_null: Vec<String> = Vec::new();
        let args = self.processcfg.args.clone().unwrap_or(arg_null);
        let log_path = self.processcfg.logfile.clone().unwrap();
        let error_path = self.processcfg.errfile.clone().unwrap();
        let child = Command::new(command)
            .args(args)
            .stdout(creat_file(log_path))
            .stderr(creat_file(error_path))
            .spawn()
            .expect("failed to execute child");

        self.processcfg.pid = Some(child.id());

        //record pid
        let pid = child.id();
        let pidfile = self.processcfg.pidfile.clone().unwrap();
        write_pid(pidfile, pid);

        //record process handle
        self.processhandle = Some(child);

        //record process status
        let name = self.processcfg.name.clone().unwrap();
        info!("{} started", name);

    }

    //run all child processes
    pub fn start_all(self) {
        for (_, child_process) in self.children {
            run_process(child_process);
        }
    }

    //stop process
    pub fn stop(&mut self) {
        let name = self.processcfg.name.clone().unwrap();
        let pidfile = self.processcfg.pidfile.clone().unwrap();
        match self.find_process() {
            None => {
                warn!("{} not started", name);
                return;
            }
            Some(pid) => {
                let pid_str = pid.to_string();
                let args = vec!["-9", &pid_str];
                Command::new("kill").args(args).spawn().expect("kill command failed to start");
                info!("{} stopped", name);
                //todo free resouce
                delete_pidfile(pidfile);
            }
        }
    }
    //stop all  processes
    pub fn stop_all(mut self) {

        //stop parent process
        self.stop();

        //stop all child process
        for (_, child_process) in self.children {
            let mut process = child_process.lock().unwrap();
            process.stop();
        }

    }
}

//run child process
pub fn run_process(child_process: Arc<Mutex<Processes>>) {

    thread::spawn(move || {
        loop {
            {
                //wait until process exit,then restart process
                let process_wait = child_process.clone();
                let mut process = process_wait.lock().unwrap();

                let name = process.processcfg.name.clone().unwrap();
                let pidfile = process.processcfg.pidfile.clone().unwrap();

                //check process exsit
                match process.find_process() {
                    Some(pid) => {
                        warn!("{} already started,pid is {}", name, pid);
                        return;
                    }
                    None => {}
                }

                // start child process
                process.start();

                let process_handle = &mut process.processhandle;

                match process_handle {
                    &mut Some(ref mut child) => {
                        match child.wait() {
                            Ok(_status) => {
                                warn!("{} exit status is {:?}", name, _status);
                                delete_pidfile(pidfile);
                            }
                            Err(e) => {
                                warn!("{} processhandle error {}", name, e);
                                delete_pidfile(pidfile);
                                return;
                            }
                        }
                    }
                    &mut None => {
                        //almost never happen
                        delete_pidfile(pidfile);
                        warn!("{} processHandle is None", name);
                        return;
                    }
                }
            }
            match change_status(&child_process) {
                //reach max respwawn times
                false => return,
                _ => {}
            }

        }
    });

}

//change child status..
pub fn change_status(child_process: &Arc<Mutex<Processes>>) -> bool {

    let process_temp = child_process.clone();
    let mut process = process_temp.lock().unwrap();

    let process_name = process.processcfg.name.clone().unwrap();

    //repawns++
    process.processcfg.respawns = process.processcfg.respawns.unwrap_or(0).checked_add(1);

    //reach max respawn times,default:3 times
    if process.processcfg.respawns.unwrap() > process.processcfg.respawn.unwrap_or(3) {
        warn!("{} reach max respawn limit", process_name);
        return false;
    }
    true
}

//write pid to the path file
pub fn write_pid(path: String, pid: u32) {
    let mut pid_file: File = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("pid file path error");
    pid_file.write_fmt(format_args!("{}", pid)).expect("write pid failed");
}
// read pid from the path file
pub fn read_pid(path: String) -> u32 {
    match File::open(path) {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).expect("read pid file failed");
            let pid = contents.parse::<u32>().expect("parse pid error from pid file");
            return pid;
        }
        Err(_) => {
            return 0;
        }
    }
}

//delete pid file
pub fn delete_pidfile(path: String) {
    remove_file(path).expect("delete pid failed");
}


//create log file
fn creat_file(path: String) -> File {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .expect("log path error!");

    file
}


//whether process exsit
fn check_process(pidfile: String) -> Option<u32> {
    //read pid from pidfile
    let pid: u32 = read_pid(pidfile);
    if pid == 0 {
        return None;
    }

    match File::open(format!("/proc/{}/cmdline", pid)) {
        Ok(_) => {
            return Some(pid);
        }
        Err(_) => {
            return None;
        }
    }
}
