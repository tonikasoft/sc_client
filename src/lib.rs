extern crate config;
extern crate serde;
#[macro_use] extern crate serde_derive;
pub mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
