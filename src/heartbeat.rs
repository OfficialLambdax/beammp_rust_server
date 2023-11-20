use serde::Serialize;
use tokio::sync::mpsc::Receiver;

#[derive(Serialize)]
struct HeartbeatInfo {
    uuid: String,
    players: usize,
    maxplayers: usize,
    port: u16,
    map: String,
    private: String, // Needs to be either "true" or "false"
    version: String,
    clientversion: String,
    name: String,
    modlist: String,
    modstotalsize: usize,
    modstotal: usize,
    playerslist: String,
    desc: String,
}

pub async fn backend_heartbeat(config: std::sync::Arc<crate::config::Config>, mut hb_rx: Receiver<crate::server::ServerStatus>) {
	// backend will not accept heartbeat on invalid keys, no point handling it.
	let mut key_is_invalid = false;
	if config.general.auth_key.is_none() {key_is_invalid = true} else {
		// valid keys format
		// xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
		// -8--------4----4----4----12--------
		let key_check = config.general.auth_key.clone().unwrap();
		let key_check: Vec<&str> = key_check.split("-").collect();
		if key_check.len() != 5 {key_is_invalid = true}
		else if key_check[0].len() != 8 {key_is_invalid = true}
		else if key_check[1].len() != 4 {key_is_invalid = true}
		else if key_check[2].len() != 4 {key_is_invalid = true}
		else if key_check[3].len() != 4 {key_is_invalid = true}
		else if key_check[4].len() != 12 {key_is_invalid = true}
	}
	if key_is_invalid {debug!("auth_key has invalid format. canceling heartbeat init");return}
	
    let mut info = HeartbeatInfo {
        uuid: config.general.auth_key.clone().unwrap_or(String::from("Unknown name!")),
        players: 0,
        maxplayers: config.general.max_players,
        port: config.general.port.unwrap_or(30814),
        map: config.general.map.clone(),
        private: if config.general.private { String::from("true") } else { String::from("false") },
        version: String::from("3.3.0"), // TODO: Don't hardcode this
        clientversion: String::from("2.0"), // TODO: What? I think for now I can fill in 2.0
        name: config.general.name.clone(),
        modlist: String::from("-"), // TODO: Implement this
        modstotalsize: 0, // TODO: Implement this
        modstotal: 0, // TODO: Implement this
        playerslist: String::new(),
        desc: config.general.description.clone(),
    };

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    loop {
        interval.tick().await;

        tokio::select! {
            _ = heartbeat_post(&info) => {}
            status = hb_rx.recv() => {
                if let Some(status) = status {
                    trace!("status update: {:?}", status);
                    info.players = status.player_count;
                    info.playerslist = status.player_list.clone();
                }
            }
        }
    }
}

async fn heartbeat_post(heartbeat_info: &HeartbeatInfo) {
    match reqwest::Client::builder()
        .local_address("0.0.0.0".parse::<std::net::IpAddr>().unwrap())
        .build().unwrap()
        .post("https://backend.beammp.com/heartbeat")
        .form(heartbeat_info)
        .send()
        .await
    {
        Ok(resp) => {
            debug!("heartbeat response:\n{:?}", resp.text().await);
        },
        Err(e) => error!("Heartbeat error occured: {e}"),
    }
}
