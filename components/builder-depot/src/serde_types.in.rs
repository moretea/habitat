// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::net::SocketAddr;

pub mod http {
    #[derive(Clone, Serialize, Deserialize)]
    pub struct OriginCreateReq {
        pub name: String,
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    pub path: String,
    pub listen_addr: SocketAddr,
    pub datastore_addr: SocketAddr,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<SocketAddr>,
    /// URL to GitHub API
    pub github_url: String,
    /// Client identifier used for GitHub API requests
    pub github_client_id: String,
    /// Client secret used for GitHub API requests
    pub github_client_secret: String,
    /// allows you to upload packages and public keys without auth
    pub insecure: bool,
    /// Whether to log events for funnel metrics
    pub events_enabled: bool,
}
