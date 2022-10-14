use assets_plugin::resources::heroes::HeroType;

#[derive(Debug, Clone)]
pub struct Reward(pub Vec<HeroType>);
