use std::io;

enum State {
    //Closed,
    // Listen,
    SynRcvd,
    Estab,
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: RecvSequenceSpace,
    ip: etherparse::Ipv4Header,
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
        let mut c = Connection {
            state: State::SynRcvd,
            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: iss + 1,
                wnd: 10,
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
        };

        let mut syn_ack = etherparse::TcpHeader::new(
            tcph.destination_port(),
            tcph.source_port(),
            c.send.iss,
            c.send.wnd,
        );
        syn_ack.acknowledgment_number = c.recv.nxt;
        syn_ack.syn = true;
        syn_ack.ack = true;
        c.ip.set_payload_len(syn_ack.header_len() as usize + 0);

        //syn_ack.checksum = syn_ack.calc_checksum_ipv4(&ip, &[]).expect("failed to compute checksum");
        eprintln!("got ip header: \n {:02x?}", iph);
        eprintln!("got tcp header: \n {:02x?}", tcph);

        let unwritten = {
            let mut unwritten = &mut buf[..];
            c.ip.write(&mut unwritten);
            syn_ack.write(&mut unwritten);
            unwritten.len();
        };
        nic.send(&buf[..unwritten])?;
        Ok(Some(c))
    }

    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
    ) -> io::Result<()> {
        // first check that sequence numbers are valid (RFC 793 S3.3)
        //
        // acceptable ack check
        // SND.UNA < SEG.ACK < SND.NXT
        // but remember wrapping!!!
        //
        let ackn = tcph.acknowledgment_number();
        if !is_between_wrapped(self.send.una, ackn, self.send.nxt.wrapping_add(1)) {
            return Ok(());
        }

        // valid segment check
        let seqn = tcph.sequence_number();
        let wend = self.recv.nxt.wrapping_add(self.recv.wnd as u32);
        if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend)
            && !is_between_wrapped(
                self.recv.nxt.wrapping_sub(1),
                seqn + data.len() as u32 - 1,
                wend,
            )
        {
            return Ok(());
        }

        match self.state {
            State::SynRcvd => {
                // expect to get an ACK for our SYN
            }

            State::Estab => {
                unimplemented!();
            }
        }

        Ok(())
    }
}
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
