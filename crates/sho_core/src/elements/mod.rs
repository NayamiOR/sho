// 核心实体 (Core Entities)
pub mod entities;

// 时地 (Temporal & Spatial Context)
pub mod context;

// 事件与行为 (Events & Actions)
pub mod events;

// 情势与体系 (Situations & Systems)
pub mod structures;

// 史料与解读 (Historical Sources & Interpretation)
pub mod sources;

// Re-export commonly used types
pub use context::*;
pub use entities::*;
pub use events::*;
pub use sources::*;
pub use structures::*;
