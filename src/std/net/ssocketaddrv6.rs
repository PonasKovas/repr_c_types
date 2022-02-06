use super::SIpv6Addr;
use std::{
    fmt::Display,
    net::{SocketAddrV6, ToSocketAddrs},
    str::FromStr,
};

/// An IPv6 socket address.
///
/// See documentation of [`std::net::SocketAddrV6`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SSocketAddrV6 {
    ip: SIpv6Addr,
    flowinfo: u32,
    scope_id: u32,
    port: u16,
}

impl SSocketAddrV6 {
    pub fn from_socketaddrv6(x: &SocketAddrV6) -> Self {
        Self {
            ip: SIpv6Addr::from_ipv6addr(x.ip()),
            flowinfo: x.flowinfo(),
            scope_id: x.scope_id(),
            port: x.port(),
        }
    }
    pub fn as_socketaddrv6(&self) -> SocketAddrV6 {
        SocketAddrV6::new(
            self.ip.as_ipv6addr(),
            self.port,
            self.flowinfo,
            self.scope_id,
        )
    }

    pub fn flowinfo(&self) -> u32 {
        self.as_socketaddrv6().flowinfo()
    }
    pub fn ip(&self) -> &SIpv6Addr {
        &self.ip
    }
    pub fn new(ip: SIpv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> Self {
        Self {
            ip,
            port,
            flowinfo,
            scope_id,
        }
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn scope_id(&self) -> u32 {
        self.scope_id
    }
    pub fn set_flowinfo(&mut self, new_flowinfo: u32) {
        self.flowinfo = new_flowinfo;
    }
    pub fn set_ip(&mut self, new_ip: SIpv6Addr) {
        self.ip = new_ip;
    }
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }
    pub fn set_scope_id(&mut self, new_scope_id: u32) {
        self.scope_id = new_scope_id;
    }
}

impl Display for SSocketAddrV6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.as_socketaddrv6(), f)
    }
}

impl From<SocketAddrV6> for SSocketAddrV6 {
    fn from(x: SocketAddrV6) -> Self {
        Self::from_socketaddrv6(&x)
    }
}

impl From<SSocketAddrV6> for SocketAddrV6 {
    fn from(x: SSocketAddrV6) -> Self {
        x.as_socketaddrv6()
    }
}

impl FromStr for SSocketAddrV6 {
    type Err = <SocketAddrV6 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(SocketAddrV6::from_str(s)?))
    }
}

impl ToSocketAddrs for SSocketAddrV6 {
    type Iter = <SocketAddrV6 as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        ToSocketAddrs::to_socket_addrs(&self.as_socketaddrv6())
    }
}
