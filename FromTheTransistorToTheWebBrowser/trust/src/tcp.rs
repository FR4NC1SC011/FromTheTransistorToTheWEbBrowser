pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,

}

impl Default for State {
    fn default() -> Self {
    //    State {}
        State::Listen
    }
}


impl State {
    pub fn on_packet<'a>(
        &mut self,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {

        match *self {
            State::Closed => {
                return;
            }
            State::Listen => {
                if !tcph.syn() {
                    // got unexpected SYN
                    return;
                }

                let mut syn_ack = etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), 0, 0);
                syn_ack.syn = true;
                syn_ack.ack = true;
            }
        }

        eprintln!(
            "{} : {} -> {} : {} {}b of tcp",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len(),
        );
        }
    }

