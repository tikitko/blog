pub mod app_content_context;
pub mod date;
pub mod external;
#[cfg(feature = "client")]
pub mod get;
pub mod head;
pub mod logged_user_context;
pub mod use_load;

pub use app_content_context::*;
pub use external::*;
#[cfg(feature = "client")]
pub use get::*;
pub use logged_user_context::*;
pub use use_load::*;

#[cfg(not(feature = "client"))]
pub trait RequestableItem<P> {}

#[cfg(not(feature = "client"))]
impl<T, P> RequestableItem<P> for T {}
