// sortcrab — library root

pub mod cli;
pub mod config;
pub mod rules;
pub mod classify;
pub mod semester;
pub mod mover;
pub mod error;

/// Entry point for the sortcrab CLI.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
