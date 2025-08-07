use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::net::Ipv4Addr;
use crate::ofp_controller::openflow0x01::OF0x01Controller;
use rust_ofp::openflow0x01::{Action, PacketIn, PacketOut, PseudoPort, SwitchFeatures};
use rust_ofp::openflow0x01::message::parse_payload;
use rust_ofp::packet::Nw;

pub struct RustController {
    known_hosts: HashMap<u32, u32>,
}

impl RustController {
    fn routing_packet_in(&mut self, sw: u64, pkt: PacketIn, stream: &mut TcpStream) {
        let pk = parse_payload(&pkt.input_payload);
        let _dst_mac = pk.dl_dst;
        let _src_mac = pk.dl_src;
        let dst_ip: u32;
        let src_ip: u32;
        let dst_ip_addr: Ipv4Addr;
        let src_ip_addr: Ipv4Addr;
        
        let nw_header = pk.nw;
        match nw_header {
            Nw::Ip(ip_header) => {
                src_ip = ip_header.src;
                dst_ip = ip_header.dst;
                src_ip_addr = Ipv4Addr::from(src_ip);
                dst_ip_addr = Ipv4Addr::from(dst_ip);
            }
            _ => {
                println!("Non IP Packet");
                let pkt_out = PacketOut {
                    output_payload: pkt.input_payload,
                    port_id: None,
                    apply_actions: vec![Action::Output(PseudoPort::AllPorts)],
                };
                Self::send_packet_out(sw, 0, pkt_out, stream);
                return
            } 
        }
        println!("From: {:?}", src_ip_addr);
        println!("To {:?}", dst_ip_addr);

        let dst_policy = self.known_hosts.get(&src_ip);
        match dst_policy {
            Some(d) if *d == dst_ip => {
                let pkt_out = PacketOut {
                    output_payload: pkt.input_payload,
                    port_id: None,
                    apply_actions: vec![Action::Output(PseudoPort::AllPorts)],
                };
                Self::send_packet_out(sw, 0, pkt_out, stream);
                println!("Packet Out");
            }
            _ => ()
        }
    }
}

impl OF0x01Controller for RustController {
    fn new(policy: HashMap<u32, u32>) -> RustController {
        RustController {known_hosts: policy}
    }

    fn switch_connected(&mut self, _: u64, _: SwitchFeatures, stream: &mut TcpStream, remote: &mut TcpStream) {
        remote.write_all(stream.peer_addr().unwrap().ip().to_string().as_bytes()).unwrap();
        println!("write to remote");
    }

    fn switch_disconnected(&mut self, _: u64) {}

    fn packet_in(&mut self, sw: u64, _: u32, pkt: PacketIn, stream: &mut TcpStream) {
        self.routing_packet_in(sw, pkt, stream);
    }
}