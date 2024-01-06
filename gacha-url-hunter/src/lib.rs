
mod locator;
mod utils;
mod validator;
pub mod hunter;
pub mod error;
pub mod common;

#[cfg(test)]
mod tests {
    use crate::common::GameRegion::*;
    use crate::common::MihoyoGame::*;
    use crate::hunter::hunt_gacha_url;

    #[test]
    fn it_works() {
        hunt_gacha_url(&HonkaiStarRail, &OS).expect("Fuck");
    }
}
