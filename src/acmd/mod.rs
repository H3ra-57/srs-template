use super::*;

mod taunts;
mod neutral;
mod smashes;
mod aerials;
mod specials;

pub fn install(agent: &mut Agent) {
    taunts::install();
    neutral::install(agent);
    smashes::install(agent);
    aerials::install();
    specials::install(agent);
}