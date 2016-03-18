use request::InvokeArgument;
use response::InvokeResponse;
use state::Cucumber;

pub mod registration;

pub trait SendableStep<World>: Send + Fn(&Cucumber<World>, &mut World, Vec<InvokeArgument>) -> InvokeResponse {}

impl<T, World> SendableStep<World> for T where T: Send + Fn(&Cucumber<World>, &mut World, Vec<InvokeArgument>) -> InvokeResponse {}

pub type Step<World> = Box<SendableStep<World, Output=InvokeResponse>>;

pub type StepId = u32;
