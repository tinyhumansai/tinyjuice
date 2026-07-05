//! Typed compression pipeline primitives.
//!
//! These types make TinyJuice's safety contract explicit for new code:
//! lossless reformats produce ordinary transform output, while lossy offloads
//! must carry a CCR token from a store put result before model-facing text can
//! be emitted.

mod estimate;
mod report;
mod transform;

pub use estimate::{BloatEstimate, BloatReason, estimate_bloat};
pub use report::{PipelineReport, PipelineSkipReason, PipelineStep};
pub use transform::{
    OffloadOutput, OffloadTransform, PipelineInput, ReformatTransform, TransformOutput,
};
