use async_trait::async_trait;
use common::error::AppResult;
use shaku::Interface;

use crate::model::event::{DeleteLog, DeleteRef, FindRef, PagenatedResult};
use crate::model::id::LogId;
use crate::model::pota::{
    POTAActivatorLog, POTAHunterLog, POTALogUser, POTAReference, POTAReferenceWithLog, ParkCode,
};

#[async_trait]
pub trait POTARepository: Send + Sync + Interface {
    async fn find_reference(&self, query: &FindRef) -> AppResult<Vec<POTAReferenceWithLog>>;

    async fn create_reference(&self, refernces: Vec<POTAReference>) -> AppResult<()>;
    async fn show_reference(&self, query: &FindRef) -> AppResult<POTAReference>;
    async fn show_all_references(
        &self,
        query: &FindRef,
    ) -> AppResult<PagenatedResult<POTAReference>>;
    async fn update_reference(&self, refernces: Vec<POTAReference>) -> AppResult<()>;
    async fn delete_reference(&self, query: DeleteRef<ParkCode>) -> AppResult<()>;

    async fn upload_activator_log(&self, logs: Vec<POTAActivatorLog>) -> AppResult<()>;
    async fn upload_hunter_log(&self, logs: Vec<POTAHunterLog>) -> AppResult<()>;
    async fn delete_log(&self, query: DeleteLog) -> AppResult<()>;

    async fn find_logid(&self, query: LogId) -> AppResult<POTALogUser>;
    async fn update_logid(&self, log: POTALogUser) -> AppResult<()>;
}
