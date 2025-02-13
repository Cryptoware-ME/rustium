use axum::async_trait;
// use di::ServiceProvider;

use crate::prelude::*; //{prelude::*, settings::interface::IRustiumSettings};

pub trait RustiumThreadSafe: Send + Sync {}

#[async_trait]
pub trait RustiumService: Send + Sync {
    fn as_rustium(&self) -> RustiumResult<Option<Box<&dyn RustiumService>>>;
    async fn init(&mut self) -> RustiumResult<()>;
    async fn run(&self) -> RustiumResult<()>;
}

// pub fn init_services<S>(provider: ServiceProvider, services: Vec<S>) -> RustiumResult<bool>
// where
//     S: RustiumService,
// {
//     let service = match provider.get_mut::<dyn IRustiumSettings>() {
//         Some(set) => set,
//         None => {
//             return Err(RustiumError::ServiceNotFound(
//                 "The required settings service is missing".into(),
//             ))
//         }
//     };

//     let mut settings = match service.write() {
//         Ok(setting) => setting,
//         Err(_) => {
//             return Err(RustiumError::PoisonedRef(
//                 "Service is poisoned".into(),
//             ))
//         }
//     };

//     settings
//         .as_rustium()
//         .expect("Settings Service is poisoned")
//         .expect("Settings Service is poisoned")
//         .init()
//         .await?;
// }
