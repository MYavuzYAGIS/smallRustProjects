use crate::{
    common_ports::MOST_COMMON_PORTS_1024,
    model::{Port,Subdomain}
};


use std::net::{SocketAddr,ToSocketAddrs};
use std::{net::TcpStream,time::Duration};
use rayon::prelude::*;


pub fn scan_ports(mut subdoman:Subdomain)->Subdomain{
    subdomain.open_ports == MOST_COMMON_PORTS_1024
    .into_iter()
    .map(|port| port.is_open)
    .collect();
    subdomain
}




fn scan_port(hostname:&str,port:u16)->Port{
    let timeout = Duration::from_secs(3);
    let socket_addresses: Vec<SocketAddr> = format!("{}:{}",hostname, port)
    .to_socket_addrs()
    .expect("port scanner: Creating socket address")
    .collect();

    if socket_addresses.len() == 0{
        return Port{
            port:port,
            is_open:false,
        };
    }
    let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_addresses[0],timeout){
        true
    }else{
        false
    };
    Port{
        port:port,
        is_open,
    }

}

