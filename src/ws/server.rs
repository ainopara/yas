use std::net::{TcpListener, TcpStream};
use anyhow::anyhow;
use clap::ArgMatches;
use log::{info, warn};
use tungstenite::{accept, Message, WebSocket};
use crate::artifact::internal_artifact::InternalArtifact;
use crate::lock::LockAction;
use crate::ws::packet::{ConfigNotifyData, LockRspData, Packet, ScanRspData};
use crate::common::cli::get_cli;

pub fn run_ws(
    matches: ArgMatches,
    do_scan: fn(_matches: ArgMatches) -> anyhow::Result<Vec<InternalArtifact>>,
    do_lock: fn(_matches: ArgMatches, _actions: Vec<LockAction>) -> anyhow::Result<()>
) -> anyhow::Result<()> {
    let verbose = matches.get_flag("verbose");
    let cfg_ntf = ConfigNotifyData::packet(&matches)?;

    let addr = matches.get_one::<String>("listen").unwrap();
    let server = TcpListener::bind(addr)?;

    info!("websocket server started: ws://{}", addr);

    let recv_packet = |ws: &mut WebSocket<TcpStream>| -> anyhow::Result<Option<Packet>> {
        match ws.read_message() {
            Ok(Message::Text(json)) => Ok(Some(serde_json::from_str::<Packet>(&json)?)),
            Ok(m) => {
                warn!("ignored message: {:?}", m);
                Ok(None)
            }
            Err(e) => {
                warn!("connection lost: {}", e);
                Err(anyhow!(e))
            }
        }
    };

    let handle_packet = |pkt: &Packet| -> anyhow::Result<Option<Packet>> {
        match pkt {
            Packet::ScanReq(p) => {
                if verbose {
                    info!("recieved: {:?}", pkt);
                } else {
                    info!("recieved: {}", pkt.name());
                }
                let matches = get_cli()
                    .no_binary_name(true)
                    .try_get_matches_from(p.argv.iter())?;
                Ok(Some(ScanRspData::packet(do_scan(matches))?))
            }
            Packet::LockReq(p) => {
                if verbose {
                    info!("recieved: {:?}", pkt);
                } else {
                    info!("recieved: {}", pkt.name());
                }
                let matches = get_cli()
                    .no_binary_name(true)
                    .try_get_matches_from(p.argv.iter())?;
                let actions = match &p.lock_json {
                    Some(json_str) => LockAction::from_lock_json(&json_str)?,
                    None => match &p.indices {
                        Some(indices) => LockAction::from_v1(&indices),
                        None => Vec::new(),
                    },
                };
                Ok(Some(LockRspData::packet(do_lock(matches, actions))?))
            }
            p => {
                warn!("unexpected packet: {}", p.name());
                Err(anyhow!("unexpected packet"))
            }
        }
    };

    let send_packet = |ws: &mut WebSocket<TcpStream>, pkt: &Packet| -> anyhow::Result<()> {
        match ws.write_message(Message::Text(pkt.to_json()?)) {
            Ok(_) => {
                if verbose {
                    info!("sent: {:?}", pkt);
                } else {
                    info!("sent: {}", pkt.name());
                }
                Ok(())
            }
            Err(e) => {
                warn!("connection closed: {}", e);
                Err(anyhow!(e))
            }
        }
    };

    for stream in server.incoming() {
        let stream = stream?;
        info!("connection established: {}", stream.peer_addr()?);
        let mut ws = accept(stream)?;
        send_packet(&mut ws, &cfg_ntf)?;
        loop {
            let pkt = match recv_packet(&mut ws) {
                Ok(Some(p)) => p,
                Ok(None) => continue,
                Err(_) => break,
            };
            let rsp = match handle_packet(&pkt) {
                Ok(Some(p)) => p,
                Ok(None) => continue,
                Err(_) => continue,
            };
            if let Err(_) = send_packet(&mut ws, &rsp) {
                break;
            }
        }
    }
    Ok(())
}
