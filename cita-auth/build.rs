// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate util;

use std::env;

use util::build_info::gen_build_info;

const VERSION: &str = "1.0.0";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    gen_build_info(out_dir.as_ref(), "build_info.rs", VERSION.to_owned());
}
