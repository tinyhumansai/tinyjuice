//! Host-side seams for embedding TinyJuice in an agent product.
//!
//! - [`focus`]: the `summary_focus` tool argument a caller uses to steer the
//!   summary of a large result.
//! - [`generate`] (feature `host`): the turn-bound registry behind
//!   `MlHost.Generate`, so the module's summary stage can reach the model the
//!   turn is running on without task-local state crossing the bus.
//! - [`retrieve_tool`] (feature `tinytools`): the `retrieve_tool_output` agent
//!   tool over a host-supplied lookup.

pub mod focus;
#[cfg(feature = "host")]
pub mod generate;
#[cfg(feature = "tinytools")]
pub mod retrieve_tool;
