use bitflags::bitflags;
use std::collections::VecDeque;
use std::{io, time};

bitflags! {
    pub (crate)struct Available: u32 {
        const READ = 0b00000001;
        const WRITE = 0b00000010;
    }
}

#[derive(Debug)]
pub enum State {
    //Closed,
    // Listen,
    SynRcvd,
    Estab,
    FinWait1,
    FinWait2,
    TimeWait,
}

impl State {
    fn is_synchronized(&self) -> bool {
        match *self {
            State::SynRcvd | State::Estab => false,
            State::FinWait1 | State::FinWait2 | State::TimeWait => true,
        }
    }
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: RecvSequenceSpace,
    ip: etherparse::Ipv4Header,
    tcp: etherparse::TcpHeader,
    timers: Timers,

    pub(crate) incoming: VecDeque<u8>,
    pub(crate) unacked: VecDeque<u8>,
    pub(crate) closed: bool,
    closed_at: Option<u32>,
}

struct Timers {
    send_times: BTreeMap<u32, time::Instant>,
    srtt: time::Duration,
}

impl Connection {
    pub(crate) fn is_rcvd_closed(&self) -> bool {
        if let State::TimeWait = self.state {
            // TODO: any state after rcvd FIN, so also CLOSE-WAIT, LAST-ACK, CLOSED, CLOSING
            true
        } else {
            false
        }
    }

    fn availability(&self) -> Available {
        let mut a = Available::empty();
        if self.is_rcvd_closed() || !self.incoming.is_empty() {
            a |= Available::READ;
        }
        //TODO: take into account self.state
        //TODO: set Available::WRITE
        a
    }
}

struct SendSequenceSpace {
    ///SND.UNA - send unacknowledged
    una: u32,
    ///SND.NXT - send next
    nxt: u32,
    ///SND.WND - send window
    wnd: u16,
    ///SND.UP  - send urgent pointer
    up: bool,
    ///SND.WL1 - segment sequence number used for last window update
    wl1: usize,
    ///SND.WL2 - segment acknowledgment number used for last window update
    wl2: usize,
    ///ISS     - initial send sequence number
    iss: u32,
}

struct RecvSequenceSpace {
    ///RCV.NXT - receive next
    nxt: u32,
    ///RCV.WND - receive window
    wnd: u16,
    ///RCV.UP  - receive urgent pointer
    up: bool,
    ///IRS     - initial receive sequence number
    irs: u32,
}

impl Connection {
    // fn send()
    pub fn accept<'a>(
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8; 1500];

        if !tcph.syn() {
            // got unexpected SYN
            return Ok(None);
        }

        let iss = 0;
        let wnd = 1024;
        let mut c = Connection {
            timers: Timers {
                send_times: Default::default(),
                srtt: time::Duration::from_secs(1 * 60),
            },
            state: State::SynRcvd,
            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: iss,
                wnd: wnd,
                up: false,
                wl1: 0,
                wl2: 0,
            },

            recv: RecvSequenceSpace {
                irs: tcph.sequence_number(),
                nxt: tcph.sequence_number() + 1,
                wnd: tcph.window_size(),
                up: false,
            },
            tcp: etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), iss, wnd),

            ip: etherparse::Ipv4Header::new(
                0,
                64,
                etherparse::IpTrafficClass::Tcp,
                [
                    iph.destination()[0],
                    iph.destination()[1],
                    iph.destination()[2],
                    iph.destination()[3],
                ],
                [
                    iph.source()[0],
                    iph.source()[1],
                    iph.source()[2],
                    iph.source()[3],
                ],
            ),

            incoming: Default::default(),
            unacked: Default::default(),

            closed: false,
            closed_at: None,
        };

        c.tcp.syn = true;
        c.tcp.ack = true;
        c.write(nic, c.send.nxt, &[], &[], 0)?;
        Ok(Some(c))
    }

    fn write(
        &mut self,
        nic: &mut tun_tap::Iface,
        seq: u32,
        payload1: &[u8],
        payload2: &[u8],
        len: usize,
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        self.tcp.sequence_number = seq;
        self.tcp.acknowledgment_number = self.recv.nxt;
        let max_data = std::cmp::min(len, payload1.len() + payload2.len());

        let size = std::cmp::min(
            buf.len(),
            self.tcp.header_len() as usize + self.ip.header_len() as usize + max_data,
        );
        self.ip
            .set_payload_len(size - self.ip.header_len() as usize);

        self.tcp.checksum = self
            .tcp
            .calc_checksum_ipv4(&self.ip, &[])
            .expect("failed to compute checksum");

        use std::io::Write;
        let mut unwritten = &mut buf[..];
        self.ip.write(&mut unwritten);
        self.tcp.write(&mut unwritten);

        let payload_bytes = {
            let mut written = 0;
            let mut len = max_data;
            let mut p1l = std::cmp::min(len, payload1.len());
            written += unwritten.write(&payload1[..p1l])?;
            len -= written;

            let mut p2l = std::cmp::min(len, payload2.len());
            written += unwritten.write(&payload2[..p2l])?;

            written
        };

        let unwritten = unwritten.len();
        let next_seq = seq.wrapping_add(payload_bytes as u32);
        if self.tcp.syn {
            next_seq = next_seq.wrapping_add(1);
            self.tcp.syn = false;
        }
        if self.tcp.fin {
            next_seq = next_seq.wrapping_add(1);
            self.tcp.fin = false;
        }
        if wrapping_lt(self.send.nxt, next_seq) {
            self.send.nxt = next_seq;
        }
        self.send_times.insert(seq, time::Instant::now());

        nic.send(&buf[..buf.len() - unwritten])?;
        Ok(payload_bytes)
    }

    fn send_rst(&mut self, nic: &mut tun_tap::Iface) -> io::Result<()> {
        self.tcp.rst = true;
        self.tcp.sequence_number = 0;
        self.tcp.acknowledgment_number = 0;
        self.write(nic, self.send.nxt, &[], &[], 0)?;
        Ok(())
    }

    pub(crate) fn on_tick(&mut self, nic: &mut tun_tap::Iface) -> io::Result<()> {
        // decide if it needs to send something
        let nunacked = self.send.nxt.wrapping_sub(self.send.una);
        let unsent = self.unacked.len() as u32 - nunacked;

        let waited_for = self
            .timers
            .send_times
            .range(self.send.una..)
            .next()
            .map(time::Instant::elapsed);

        let should_retransmit = if let Some(waited_for) = waited_for {
            waited_for > time::Duration::from_secs(1) && waited_for.as_float_secs() > 1.5 * self.timers.srtt.as_float_secs()
        } else {
            false
        };

        if should_retransmit { 
            let resend = std::cmp::min(self.unacked.len() as u32, self.send.wnd as u32);
            if resend < self.send.wnd as u32 && self.closed {
                self.tcp.fin = true;
                self.closed_at = Some(self.send.una + self.unacked.len());
            }

            let (h, t) = self.unacked.as_slices();
            self.write(nic, self.send.una, h, t, resend as usize)?;
        } else {
            if unsent == 0 && self.closed_at.is_some() {
                return Ok(());
            }

            let allowed = self.send.wnd - nunacked;
            if allowed == 0 {
                return Ok(());
            }

            let send = std::cmp::min(unsent, allowed);
            if send < allowed && self.closed && self.closed_at.is_none() {
                self.tcp.fin = true;
                self.closed_at = Some(self.send.nxt.wrapping_add(unsent));
            }
            let (mut h, mut t) = self.unacked.as_slices();

            if h.len() >= nunacked {
                h = &h[nunacked..];
            } else {
                let skipped = h.len();
                h = &[];
                t = &t[(nunacked - skipped)..];
            }
            self.write(nic, self.send.nxt, h, t, send)?;
        }

        Ok(())
    }

    pub(crate) fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<Available> {
        // first check that sequence numbers are valid (RFC 793 S3.3)

        // valid segment check
        let seqn = tcph.sequence_number();
        let mut slen = data.len() as u32;
        if tcph.fin() {
            slen += 1;
        }
        if tcph.syn() {
            slen += 1;
        }
        let wend = self.recv.nxt.wrapping_add(self.recv.wnd as u32);
        let okay = if slen == 0 {
            if self.recv.wnd == 0 {
                if seqn != self.recv.nxt {
                    false
                } else {
                    true
                }
            } else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend) {
                false
            } else {
                true
            }
        } else {
            if self.recv.wnd == 0 {
                false
            } else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend)
                && !is_between_wrapped(
                    self.recv.nxt.wrapping_sub(1),
                    seqn.wrapping_add(slen - 1),
                    wend,
                )
            {
                false
            } else {
                true
            }
        };

        if !okay {
            eprintln!("NOT OKAY");
            self.write(nic, self.send.nxt, &[], &[], 0)?;
            return Ok(self.availability());
        }

        if !tcph.ack() {
            if tcph.syn() {
                assert!(data.is_empty());
                self.recv.nxt = seqn.wrapping_add(1);
            }
            return Ok(self.availability());
        }

        let ackn = tcph.acknowledgment_number();
        if let State::SynRcvd = self.state {
            if is_between_wrapped(
                self.send.una.wrapping_sub(1),
                ackn,
                self.send.nxt.wrapping_add(1),
            ) {
                self.state = State::Estab;
            } else {
                //TODO: Reset
            }
        }

        if let State::Estab | State::FinWait1 | State::FinWait2 = self.state {
            if is_between_wrapped(self.send.una, ackn, self.send.nxt.wrapping_add(1)) {
                self.send.una = ackn;
            }

            // TODO: prune self.unacked
            // TODO: if unacked empty and waiting flush, notify
            // TODO: update window

            if let State::Estab = self.state {
                // terminate the connection
                self.tcp.fin = true;
                //self.write(nic, &[])?;
                self.state = State::FinWait1;
            }
        }

        if let State::FinWait1 = self.state {
            if self.send.una == self.send.iss + 2 {
                // our FIN has been ACKed!
                self.state = State::FinWait2;
            }
        }

        if let State::Estab | State::FinWait1 | State::FinWait2 = self.state {
            let mut unread_data_at = (self.recv.nxt - seqn) as usize;

            if unread_data_at > data.len() {
                // we must have received a retransmitted FIN that we have already seen
                // nxt points to beyond the FIN, but the FIN is not in data!
                assert_eq!(unread_data_at, data.len() + 1);
                unread_data_at = 0;
            }

            self.incoming.extend(&data[unread_data_at..]);

            self.recv.nxt = seqn
                .wrapping_add(data.len() as u32)
                .wrapping_add(if tcph.fin() { 1 } else { 0 });

            self.write(nic, self.send.nxt, &[], &[], 0)?;
        }

        if tcph.fin() {
            match self.state {
                State::FinWait2 => {
                    // we're done with the connection
                    self.write(nic, self.send.nxt, &[], &[], 0)?;
                    self.state = State::TimeWait;
                }
                _ => unimplemented!(),
            }
        }

        Ok(self.availability())
    }

    pub(crate) fn close(&mut self) -> io::Result<()> {
        self.closed = true;
        match self.state {
            State::SynRcvd | State::Estab => {
                self.state = State::FinWait1;
            }

            State::FinWait1 | State::FinWait2 => {}

            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "already closing",
                ))
            }
        };
        Ok(())
    }
}

fn wrapping_lt(lhs: u32, rhs: u32) -> bool {
    // From RFC1323:
    //     TCP determines if a data segment is "old" or "new" by testing
    //     whether its sequence number is within 2**31 bytes of the left edge
    //     of the window, and if it is not, discarding the data as "old".  To
    //     insure that new data is never mistakenly considered old and vice-
    //     versa, the left edge of the sender's window has to be at most
    //     2**31 away from the right edge of the receiver's window.
    lhs.wrapping_sub(rhs) > (1 << 31)
}

fn is_between_wrapped(start: u32, x: u32, end: u32) -> bool {
    wrapping_lt(start, x) && wrapping_lt(x, end)
}

/*
fn is_between_wrapped(start: u32, x: u32, end: u32) -> bool {
    use std::cmp::Ordering;
    match start.cmp(&x) {
        Ordering::Equal => return false,
        Ordering::Less => {
            if end >= start && end <= x {
                return false;
            }
        }
        Ordering::Greater => {
            // check is ok if only if n is between u and a
            if end < start && end > x {
            } else {
                return false;
            }
        }
    }
    true
}


*/
