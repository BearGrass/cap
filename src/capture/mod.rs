pub mod device;
pub mod live;
pub mod offline;

pub trait Capture {
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
