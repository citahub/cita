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

extern crate crossbeam_channel;

use cita_db::journaldb::Algorithm;
use cita_db::DatabaseConfig;
use core::db::NUM_COLUMNS;
use core::libexecutor::command;
use core::snapshot as CoreSnapshot;
use core::snapshot::io::{PackedReader, PackedWriter};
use crossbeam_channel::{Receiver, Sender};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Resp as Ack, SnapshotResp};
use libproto::TryInto;
use std::sync::Arc;

pub fn handle_snapshot_height(req_height: u64, current_height: u64) -> u64 {
    match req_height {
        height if height == 0 || height > current_height => {
            warn!(
                "snapshot.end_height({}) reset to current_height({})",
                req_height, current_height,
            );
            current_height
        }
        height => height,
    }
}

pub fn handle_snapshot_filename(filename: String) -> String {
    filename + "_executor.rlp"
}

pub fn spawn_take_snapshot(
    filename: String,
    highest_height: u64,
    command_req_sender: &Sender<command::Command>,
    command_resp_receiver: &Receiver<command::CommandResp>,
    mq_resp_sender: &Sender<(String, Vec<u8>)>,
) -> ::std::thread::JoinHandle<()> {
    // Prepare
    let executor = command::clone_executor_reader(&command_req_sender, &command_resp_receiver);
    let writer = PackedWriter {
        file: ::std::fs::File::create(filename).unwrap(),
        state_hashes: Vec::new(),
        block_hashes: Vec::new(),
        cur_len: 0,
    };
    let progress = ::std::sync::Arc::new(Default::default());

    // Spawn -> Take -> Response
    let mq_resp_sender = mq_resp_sender.clone();
    ::std::thread::spawn(move || {
        let result = CoreSnapshot::take_snapshot(&executor, highest_height, writer, &*progress)
            .map_err(|err| err.to_string());
        response(&mq_resp_sender, Ack::SnapshotAck, result);
    })
}

pub fn spawn_restore_snapshot(
    filename: &str,
    command_req_sender: &Sender<command::Command>,
    command_resp_receiver: &Receiver<command::CommandResp>,
    mq_resp_sender: &Sender<(String, Vec<u8>)>,
) -> ::std::thread::JoinHandle<()> {
    let executor = command::clone_executor_reader(command_req_sender, command_resp_receiver);
    let db_config = DatabaseConfig::with_columns(NUM_COLUMNS);
    let data_path = cita_directories::DataPath::root_node_path() + "/snapshot_executor";
    let snapshot_params = CoreSnapshot::service::ServiceParams {
        db_config,
        pruning: Algorithm::Archive,
        snapshot_root: data_path.into(),
        executor: Arc::new(executor),
    };
    let snapshot_service = Arc::new(
        CoreSnapshot::service::Service::new(snapshot_params).expect("new snapshot service"),
    );

    // Spawn -> Restore -> Response
    let mq_resp_sender = mq_resp_sender.clone();
    let filename = filename.to_owned();
    ::std::thread::spawn(move || {
        match PackedReader::new(::std::path::Path::new(&filename)) {
            Ok(Some(reader)) => {
                let result = CoreSnapshot::restore_using(&snapshot_service, &reader, true);
                response(&mq_resp_sender, Ack::RestoreAck, result);
            }
            Ok(None) => {
                response(
                    &mq_resp_sender,
                    Ack::RestoreAck,
                    Err(format!(
                        "failed to open {} cause: invalid format file",
                        filename
                    )),
                );
            }
            Err(err) => {
                response(
                    &mq_resp_sender,
                    Ack::RestoreAck,
                    Err(format!("failed to open {} cause: {:?}", filename, err)),
                );
            }
        };
    })
}

pub fn change_database(
    mq_resp_sender: &Sender<(String, Vec<u8>)>,
    origin_db: String,
    snap_db: String,
) {
    info!(
        "change database {} with snapshot database {}",
        origin_db, snap_db
    );
    let backup_db = origin_db.clone() + ".backup";
    let _ = ::std::fs::remove_dir_all(backup_db.clone());
    let result = Ok(())
        .and_then(|_| ::std::fs::rename(origin_db.clone(), backup_db.clone()))
        .and_then(|_| ::std::fs::rename(snap_db, origin_db))
        .and_then(|_| ::std::fs::remove_dir_all(backup_db))
        .map_err(|err| err.to_string());
    response(mq_resp_sender, Ack::ClearAck, result);
}

pub fn response(mq_resp_sender: &Sender<(String, Vec<u8>)>, ack: Ack, result: Result<(), String>) {
    let action = match ack {
        Ack::SnapshotAck => "take",
        Ack::BeginAck => "begin",
        Ack::RestoreAck => "restore",
        Ack::ClearAck => "clear",
        Ack::EndAck => "end",
    };

    let mut response = SnapshotResp::new();
    response.set_resp(ack);
    match result {
        Ok(()) => {
            info!("successful to {} snapshot", action);
            response.set_flag(true)
        }
        Err(reason) => {
            // TODO: impl Display for snapshot.cmd
            error!("failed to {} snapshot cause: {}", action, reason);
            response.set_flag(false);
        }
    }

    let message: libproto::Message = response.into();
    mq_resp_sender.send((
        routing_key!(Executor >> SnapshotResp).into(),
        message.try_into().unwrap(),
    ));
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use self::tempdir::TempDir;
    use super::*;
    use libproto::snapshot::Resp as Ack;
    use libproto::Message;
    use libproto::TryFrom;

    fn wrap(origin_db: &str, temp_dir: ::std::path::PathBuf) -> String {
        match origin_db {
            "-" => temp_dir.to_str().unwrap().to_string(),
            _ => origin_db.to_string(),
        }
    }

    #[test]
    fn test_change_database() {
        let (mq_resp_sender, mq_resp_receiver) = crossbeam_channel::unbounded();

        let cases = vec![
            (
                "/not/found/hello/i/am/evil/db",
                "/not/found/you/are/evil/db",
                false,
            ),
            ("-", "/not/found/you/are/evil/db", false),
            ("/not/found/hello/i/am/evil/db", "-", false),
            ("-", "-", true),
        ];
        for (origin_id, snap_id, expected_flag) in cases.into_iter() {
            // Prepare directories as parameters
            let origin_db_dir = TempDir::new("snapshot-test").unwrap();
            let snap_db_dir = TempDir::new("snapshot-test").unwrap();
            let origin_db_path = origin_db_dir.path().to_owned();
            let snap_db_path = snap_db_dir.path().to_owned();
            let origin_db = wrap(origin_id, origin_db_path);
            let snap_db = wrap(snap_id, snap_db_path);

            // Change database
            change_database(&mq_resp_sender, origin_db.clone(), snap_db.clone());

            // Check sent messages
            let (key, msg_vec) = mq_resp_receiver.recv().expect("receive ClearAck");
            assert_eq!(routing_key!(Executor >> SnapshotResp).to_string(), key);

            let mut message: Message = Message::try_from(msg_vec).unwrap();
            let snapshot_resp = message.take_snapshot_resp().unwrap();
            assert_eq!(Ack::ClearAck, snapshot_resp.resp);
            assert_eq!(
                expected_flag, snapshot_resp.flag,
                "change_database with ({}, {}) should be {}",
                origin_id, snap_id, expected_flag,
            );

            // Check database existence
            if expected_flag {
                assert_eq!(
                    true,
                    ::std::path::Path::new(&origin_db).exists(),
                    "the directory of origin database is exist",
                );
                assert_eq!(
                    false,
                    ::std::path::Path::new(&snap_db).exists(),
                    "snapshot database has been moved off",
                );
            }
        }
    }

    #[test]
    fn test_handle_snapshot_height() {
        assert_eq!(8, handle_snapshot_height(0, 8));
        assert_eq!(7, handle_snapshot_height(7, 8));
        assert_eq!(8, handle_snapshot_height(8, 8));
        assert_eq!(8, handle_snapshot_height(9, 8));
    }
}
