//!
//! Logic of `btm daemon ...`
//!

use crate::{
    api::{
        model::{Req, Resp},
        SERVER_US,
    },
    BtmCfg,
};
use ruc::*;

/// Run `btm daemon ...` server
pub fn run_daemon(cfg: &BtmCfg) -> Result<()> {
    loop {
        if let Ok((msg, peer)) = SERVER_US.recv_128() {
            if let Ok(r) = info!(serde_json::from_slice::<Req>(&msg)) {
                let success = info!(cfg.snapshot(r.idx())).is_ok();
                SERVER_US
                    .send(&Resp::new(r.idx(), success).to_bytes(), &peer)
                    .c(d!())?;
            }
        }
    }
}
