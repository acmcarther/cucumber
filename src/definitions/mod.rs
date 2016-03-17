use request::InvokeArgument;
use response::InvokeResponse;
use state::CucumberState;

pub mod registration;

pub trait SendableStep<World>: Send + Fn(&mut CucumberState, &mut World, Vec<InvokeArgument>) -> InvokeResponse {}

impl<T, World> SendableStep<World> for T where T: Send + Fn(&mut CucumberState, &mut World, Vec<InvokeArgument>) -> InvokeResponse {}

pub type Step<World> = Box<SendableStep<World, Output=InvokeResponse>>;

pub type StepId = u32;
