use async_trait::async_trait;
use domain::model::common::id::UserId;
use shaku::Component;
use sqlx::PgConnection;

use common::error::{AppError, AppResult};

use domain::model::common::event::{DeleteLog, DeleteRef, FindRef, PagenatedResult};
use domain::model::pota::{
    POTAActivatorLog, POTAHunterLog, POTAReference, POTAReferenceWithLog, ParkCode,
};
use domain::model::AwardProgram::POTA;

use crate::database::model::pota::{
    POTAActivatorLogImpl, POTAHunterLogImpl, POTAReferenceImpl, POTAReferenceWithLogImpl,
};
use crate::database::ConnectionPool;
use crate::implement::querybuilder::findref_query_builder;

use domain::repository::pota::POTAReferenceRepositry;

#[derive(Component)]
#[shaku(interface = POTAReferenceRepositry)]
pub struct POTAReferenceRepositryImpl {
    pool: ConnectionPool,
}

impl POTAReferenceRepositryImpl {
    async fn create(&self, r: POTAReferenceImpl, db: &mut PgConnection) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO pota_references(
                    pota_code,
                    wwff_code,
                    park_name,
                    park_name_j,
                    park_location,
                    park_locid,
                    park_type,
                    park_inactive,
                    park_area,
                    coordinates,
                    update
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9,ST_SetSRID(ST_MakePoint($10, $11), 4326), 
                $12)
            "#,
            r.pota_code,
            r.wwff_code,
            r.park_name,
            r.park_name_j,
            r.park_location,
            r.park_locid,
            r.park_type,
            r.park_inactive,
            r.park_area,
            r.longitude,
            r.latitude,
            r.update
        )
        .execute(db)
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn update(&self, r: POTAReferenceImpl, db: &mut PgConnection) -> AppResult<()> {
        sqlx::query!(
            r#"
                UPDATE pota_references SET
                    wwff_code = $2,
                    park_name = $3,
                    park_name_j = $4,
                    park_location = $5,
                    park_locid = $6,
                    park_type = $7,
                    park_inactive = $8,
                    park_area = $9,
                    coordinates = ST_SetSRID(ST_MakePoint($10, $11), 4326),
                    update = $12
                WHERE pota_code = $1
            "#,
            r.pota_code,
            r.wwff_code,
            r.park_name,
            r.park_name_j,
            r.park_location,
            r.park_locid,
            r.park_type,
            r.park_inactive,
            r.park_area,
            r.longitude,
            r.latitude,
            r.update
        )
        .execute(db)
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn delete(&self, ref_id: ParkCode, db: &mut PgConnection) -> AppResult<()> {
        sqlx::query!(
            r#"
                DELETE FROM pota_references
               WHERE pota_code = $1
            "#,
            ref_id.inner_ref(),
        )
        .execute(db)
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn delete_all(&self, db: &mut PgConnection) -> AppResult<()> {
        sqlx::query!(
            r#"
                DELETE FROM pota_references
            "#
        )
        .execute(db)
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn update_activator_log(
        &self,
        r: POTAActivatorLogImpl,
        db: &mut PgConnection,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO pota_activator_log (user_id, dx_entity, location, hasc, pota_code, park_name, first_qso_date, attempts, activations, qsos, upload)
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (user_id, pota_code) DO UPDATE
                SET dx_entity = EXCLUDED.dx_entity,
                    location = EXCLUDED.location,
                    hasc = EXCLUDED.hasc,
                    pota_code = EXCLUDED.pota_code,
                    park_name = EXCLUDED.park_name,
                    first_qso_date = EXCLUDED.first_qso_date,
                    attempts = EXCLUDED.attempts,
                    activations = EXCLUDED.activations,
                    qsos = EXCLUDED.qsos,
                    upload = EXCLUDED.upload
            "#,
            r.user_id.raw(),
            r.dx_entity,
            r.location,
            r.hasc,
            r.pota_code,
            r.park_name,
            r.first_qso_date,
            r.attempts,
            r.activations,
            r.qsos,
            r.upload
        )
        .execute(db)
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn update_hunter_log(
        &self,
        r: POTAHunterLogImpl,
        db: &mut PgConnection,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO pota_hunter_log (user_id, dx_entity, location, hasc, pota_code, park_name, first_qso_date, qsos, upload)
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT (user_id, pota_code) DO UPDATE
                SET dx_entity = EXCLUDED.dx_entity,
                    location = EXCLUDED.location,
                    hasc = EXCLUDED.hasc,
                    pota_code = EXCLUDED.pota_code,
                    park_name = EXCLUDED.park_name,
                    first_qso_date = EXCLUDED.first_qso_date,
                    qsos = EXCLUDED.qsos,
                    upload = EXCLUDED.upload
            "#,
            r.user_id.raw(),
            r.dx_entity,
            r.location,
            r.hasc,
            r.pota_code,
            r.park_name,
            r.first_qso_date,
            r.qsos,
            r.upload
        )
        .execute(db)
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn delete_log(&self, d: DeleteLog, db: &mut PgConnection) -> AppResult<()> {
        tracing::info!("delete log before: {}", d.before);
        let before = d.before;
        sqlx::query!(
            r#"
                DELETE FROM pota_activator_log
                WHERE upload < $1
            "#,
            before,
        )
        .execute(&mut *db)
        .await
        .map_err(AppError::SpecificOperationError)?;

        sqlx::query!(
            r#"
                DELETE FROM pota_hunter_log
                WHERE upload < $1
            "#,
            before,
        )
        .execute(db)
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(())
    }

    async fn select_pagenated(&self, query: &str) -> AppResult<(i64, Vec<POTAReferenceImpl>)> {
        let row = sqlx::query!("SELECT COUNT(*) as count FROM pota_references")
            .fetch_one(self.pool.inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?;
        let total: i64 = row.count.unwrap_or(0);

        let mut select = r#"
            SELECT
                pota_code,
                wwff_code,
                park_name,
                park_name_j,
                park_location,
                park_locid,
                park_type,
                park_inactive,
                park_area,
                ST_X(coordinates) AS longitude,
                ST_Y(coordinates) AS latitude,
                update
            FROM pota_references AS p WHERE "#
            .to_string();

        select.push_str(query);

        let sql_query = sqlx::query_as::<_, POTAReferenceImpl>(&select);
        let rows: Vec<POTAReferenceImpl> = sql_query
            .fetch_all(self.pool.inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?;
        Ok((total, rows))
    }

    async fn select_by_condition(
        &self,
        user_id: Option<UserId>,
        query: &str,
    ) -> AppResult<Vec<POTAReferenceWithLogImpl>> {
        let mut select = String::new();
        if user_id.is_none() {
            select.push_str(
                r#"
                SELECT
                    pota_code,
                    wwff_code,
                    park_name,
                    park_name_j,
                    park_location,
                    park_locid,
                    park_type,
                    park_inactive,
                    park_area,
                    ST_X(coordinates) AS longitude,
                    ST_Y(coordinates) AS latitude,
                    NULL as attempts,
                    NULL as activations,
                    NULL as first_qso_date,
                    NULL as qsos
                FROM pota_references AS p WHERE "#,
            );
        } else {
            let user_id = user_id.unwrap().raw().to_string();
            select.push_str(&format!(
                r#"
                SELECT
                    p.pota_code AS pota_code,
                    p.wwff_code AS wwff_code,
                    p.park_name AS park_name,
                    p.park_name_j AS park_name_j,
                    p.park_location AS park_location,
                    p.park_locid AS park_locid,
                    p.park_type AS park_type,
                    p.park_inactive AS park_inactive,
                    p.park_area AS park_area,
                    ST_X(p.coordinates) AS longitude,
                    ST_Y(p.coordinates) AS latitude,
                    a.attempts as attempts,
                    a.activations AS activations,
                    h.first_qso_date AS first_qso_date,
                    h.qsos AS qsos
                FROM pota_references AS p 
                LEFT JOIN pota_activator_log AS a ON p.pota_code = a.pota_code AND a.user_id = '{}'
                LEFT JOIN pota_hunter_log AS h ON p.pota_code = h.pota_code AND h.user_id = '{}'
                WHERE "#,
                user_id, user_id
            ));
        }
        select.push_str(query);

        let sql_query = sqlx::query_as::<_, POTAReferenceWithLogImpl>(&select);
        let rows: Vec<POTAReferenceWithLogImpl> = sql_query
            .fetch_all(self.pool.inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?;
        Ok(rows)
    }
}

#[async_trait]
impl POTAReferenceRepositry for POTAReferenceRepositryImpl {
    async fn find_reference(&self, event: &FindRef) -> AppResult<Vec<POTAReferenceWithLog>> {
        let user_id = event.user_id;
        let query = findref_query_builder(POTA, event);
        let results = self.select_by_condition(user_id, &query).await?;
        let results = results
            .into_iter()
            .map(POTAReferenceWithLog::from)
            .collect();
        Ok(results)
    }

    async fn create_reference(&self, references: Vec<POTAReference>) -> AppResult<()> {
        let mut tx = self
            .pool
            .inner_ref()
            .begin()
            .await
            .map_err(AppError::TransactionError)?;

        for r in references.into_iter().enumerate() {
            self.create(POTAReferenceImpl::from(r.1), &mut tx).await?;
            if r.0 % 100 == 0 {
                tracing::info!("insert pota {} rescords", r.0);
            }
        }
        tx.commit().await.map_err(AppError::TransactionError)?;
        Ok(())
    }

    async fn show_reference(&self, event: &FindRef) -> AppResult<PagenatedResult<POTAReference>> {
        let limit = event.limit.unwrap_or(10);
        let offset = event.offset.unwrap_or(0);
        let query = findref_query_builder(POTA, event);
        let (total, results) = self.select_pagenated(&query).await?;
        Ok(PagenatedResult {
            total,
            limit,
            offset,
            results: results.into_iter().map(POTAReference::from).collect(),
        })
    }

    async fn update_reference(&self, references: Vec<POTAReference>) -> AppResult<()> {
        let mut tx = self
            .pool
            .inner_ref()
            .begin()
            .await
            .map_err(AppError::TransactionError)?;
        for r in references.into_iter() {
            self.update(POTAReferenceImpl::from(r), &mut tx).await?;
        }
        tx.commit().await.map_err(AppError::TransactionError)?;
        Ok(())
    }

    async fn delete_reference(&self, query: DeleteRef<ParkCode>) -> AppResult<()> {
        let mut tx = self
            .pool
            .inner_ref()
            .begin()
            .await
            .map_err(AppError::TransactionError)?;
        match query {
            DeleteRef::Delete(code) => self.delete(code, &mut tx).await?,
            DeleteRef::DeleteAll => self.delete_all(&mut tx).await?,
        }
        tx.commit().await.map_err(AppError::TransactionError)?;
        Ok(())
    }

    async fn upload_activator_log(&self, logs: Vec<POTAActivatorLog>) -> AppResult<()> {
        let mut tx = self
            .pool
            .inner_ref()
            .begin()
            .await
            .map_err(AppError::TransactionError)?;

        tracing::info!("upload activator log {} rescords", logs.len());

        for r in logs.into_iter() {
            self.update_activator_log(POTAActivatorLogImpl::from(r), &mut tx)
                .await?;
        }
        tx.commit().await.map_err(AppError::TransactionError)?;
        Ok(())
    }

    async fn upload_hunter_log(&self, logs: Vec<POTAHunterLog>) -> AppResult<()> {
        let mut tx = self
            .pool
            .inner_ref()
            .begin()
            .await
            .map_err(AppError::TransactionError)?;

        tracing::info!("upload hunter log {} rescords", logs.len());

        for r in logs.into_iter() {
            self.update_hunter_log(POTAHunterLogImpl::from(r), &mut tx)
                .await?;
        }
        tx.commit().await.map_err(AppError::TransactionError)?;
        Ok(())
    }

    async fn delete_log(&self, query: DeleteLog) -> AppResult<()> {
        let mut tx = self
            .pool
            .inner_ref()
            .begin()
            .await
            .map_err(AppError::TransactionError)?;
        self.delete_log(query, &mut tx).await?;
        tx.commit().await.map_err(AppError::TransactionError)?;
        Ok(())
    }
}
