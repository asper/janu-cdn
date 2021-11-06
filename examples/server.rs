//
// Copyright (c) 2017, 2021 TAWHEED Technology  
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   TAWHEED janu team, <janu@ tawedge.co>
//

use async_std::fs;
use async_std::sync::Arc;
use janu::{Properties, Janu};
use janu_cdn::server::Server;
use janu_cdn::types::ServerConfig;

async fn read_file(path: String) -> String {
    fs::read_to_string(path).await.unwrap()
}

#[async_std::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();

    let zsession = Arc::new(
        Janu::new(Properties::from(String::from("mode=peer;listener=tcp/0.0.0.0:8998")).into())
            .await
            .unwrap(),
    );

    let config = args[1].clone();
    let config = read_file(config).await;
    let config = serde_yaml::from_str::<ServerConfig>(&config).unwrap();

    let server = Server::new(zsession, config);

    let _h = server.serve();

    println!("Ready!");

    let () = std::future::pending().await;
}
