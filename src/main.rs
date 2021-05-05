use std::{
    fs,
    env,
    process,
    thread,
    time::Duration,
};
extern crate yaml_rust;
extern crate paho_mqtt as mqtt;
use yaml_rust::{YamlLoader, /* YamlEmitter */};
use chrono::{DateTime, Local};

const DFLT_CONFIG_PATH: &str = "config.yaml";
const DFLT_CLIENT_ID: &str = "kraken_edge_heartbeater";
const DFLT_TOPIC: &str = "krakeniot";
const QOS:i32 = 1;

fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}

/*
fn debug_yaml(doc: &yaml_rust::Yaml) {
    let mut out_config = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_config);
        emitter.dump(doc).unwrap();
    }
    println!("{}", out_config);
}
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting `Kraken Edge HeartBeater`...");
    // Load config
    let config_path = env::args().nth(1).unwrap_or(DFLT_CONFIG_PATH.to_string());
    let docs = load_yaml(&config_path);
    let config = &docs[0];
    // Display the details of config 
    // debug_yaml(&config);
    println!("Loaded configs from {:?}.", &config_path);
    println!("MQTT Host: {:?}", config["mqtt"]["host"].as_str().unwrap());
    println!("MQTT Topic: {:?}", config["mqtt"]["topic"].as_str().unwrap());
    println!("Client ID: {:?}", config["client"]["id"].as_str().unwrap());
    println!("Publish Interval: {:?}", config["client"]["interval"].as_i64().unwrap());
    // set configs;
    let host = config["mqtt"]["host"].as_str().unwrap();
    let topic = config["mqtt"]["topic"].as_str().unwrap_or(DFLT_TOPIC);
    let client_id = config["client"]["id"].as_str().unwrap_or(DFLT_CLIENT_ID);
    let interval: u64 = config["client"]["interval"].as_i64().unwrap() as u64;
    // Define the set of options for to create the client.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(client_id.to_string())
        .finalize();
    // Create a client
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error createing the mqtt client: {:?}", err);
        process::exit(1);
    });
    // Define the set of options for the connection
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();
    // Connect and wait for it to complate or fail
    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }
    println!("Connect to MQTT broker successfully.");
    println!("Start to publish heartbeats...");
    // Publish heartbeat to the topic of Kraken IoT Collector
    loop {
        let dt: DateTime<Local> = Local::now();
        let timestamp: i64 = dt.timestamp();
        let content = client_id.to_string() + ":" + &timestamp.to_string();
        let msg = mqtt::Message::new(DFLT_TOPIC, content.clone(), QOS);
        println!("Publishing messages on the {:?} topic", topic);
        let tok = cli.publish(msg);
        if let Err(e) = tok {
            println!("Error sending message: {:?}", e);
            break; 
        }
        thread::sleep(Duration::from_millis(interval));
    }
    let tok = cli.disconnect(None);
    println!("Disconnect from the broker");
    tok.unwrap();
    Ok(())
}
