use core::{fmt, str::FromStr};

use lwip_rust::bindings::{
    ip4_addr_t, ip_addr__bindgen_ty_1, ip_addr_t, lwip_ip_addr_type_IPADDR_TYPE_V4,
};

#[derive(Clone, Copy, Debug)]
pub enum IpAddr {
    Ipv4(Ipv4Addr),
}

#[derive(Clone, Copy, Debug)]
pub struct Ipv4Addr(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct SocketAddr {
    pub addr: IpAddr,
    pub port: u16,
}

impl FromStr for IpAddr {
    type Err = ();

    fn from_str(s: &str) -> Result<IpAddr, ()> {
        let mut parts = s.split('.');
        let mut addr: u32 = 0;
        for i in 0..4 {
            let part = parts.next().ok_or(())?;
            let part = part.parse::<u8>().map_err(|_| ())?;
            addr |= (part as u32) << (8 * i);
        }
        Ok(IpAddr::Ipv4(Ipv4Addr(addr)))
    }
}

impl Into<ip_addr_t> for IpAddr {
    fn into(self) -> ip_addr_t {
        match self {
            IpAddr::Ipv4(Ipv4Addr(addr)) => ip_addr_t {
                u_addr: ip_addr__bindgen_ty_1 {
                    ip4: ip4_addr_t { addr },
                },
                type_: lwip_ip_addr_type_IPADDR_TYPE_V4 as u8,
            },
        }
    }
}

impl From<ip_addr_t> for IpAddr {
    #[allow(non_upper_case_globals)]
    fn from(addr: ip_addr_t) -> IpAddr {
        match addr.type_ as u32 {
            lwip_ip_addr_type_IPADDR_TYPE_V4 => {
                IpAddr::Ipv4(Ipv4Addr(unsafe { addr.u_addr.ip4.addr }))
            }
            _ => panic!("unsupported ip type"),
        }
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddr::Ipv4(addr) => write!(f, "{addr}"),
        }
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.0;
        write!(
            f,
            "{}.{}.{}.{}",
            bytes & 0xff,
            (bytes >> 8) & 0xff,
            (bytes >> 16) & 0xff,
            (bytes >> 24) & 0xff
        )
    }
}

impl FromStr for SocketAddr {
    type Err = ();

    fn from_str(s: &str) -> Result<SocketAddr, ()> {
        let mut parts = s.split(':');
        let addr = parts.next().ok_or(())?.parse::<IpAddr>()?;
        let port = parts.next().ok_or(())?.parse::<u16>().map_err(|_| ())?;
        Ok(SocketAddr { addr, port })
    }
}

impl SocketAddr {
    pub fn new(addr: IpAddr, port: u16) -> SocketAddr {
        SocketAddr { addr, port }
    }
}

impl From<(IpAddr, u16)> for SocketAddr {
    fn from((addr, port): (IpAddr, u16)) -> SocketAddr {
        SocketAddr { addr, port }
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.addr, self.port)
    }
}
