use data_feed;
use strategy;
use portfolio;

fn main() {
    let data = data_feed::get_data();
    let decision = strategy::get_decision(data);
    portfolio::make_order(decision);
}
