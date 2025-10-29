use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("ganon_volley");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}