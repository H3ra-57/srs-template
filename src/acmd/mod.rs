use super::*;

mod taunts;
mod neutral;
mod smashes;
mod aerials;
mod specials;

pub fn install(agent: &mut Agent) {
    taunts::install();
    neutral::install();
    smashes::install();
    aerials::install();
    specials::install(agent);
}