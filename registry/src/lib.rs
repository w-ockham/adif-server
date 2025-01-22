use axum::extract::FromRef;
use common::config::AppConfig;
use shaku::module;
use std::sync::{Arc, Mutex};

use adapter::{
    database::connect::ConnectionPool,
    implement::geomag::{GeoMagRepositryImpl, GeoMagRepositryImplParameters},
};

use service::implement::{
    admin_periodic::{AdminPeriodicServiceImpl, AdminPeriodicServiceImplParameters},
    admin_service::{AdminServiceImpl, AdminServiceImplParameters},
    user_service::{UserServiceImpl, UserServiceImplParameters},
};

#[cfg(not(feature = "sqlite"))]
use adapter::implement::postgis::{
    activation::{ActivationRepositryImpl, ActivationRepositryImplParameters},
    healthcheck::{HealthCheckRepositryImpl, HealthCheckRepositryImplParameters},
    locator::{LocatorRepositryImpl, LocatorRepositryImplParameters},
    pota_reference::{POTARepositoryImpl, POTARepositoryImplParameters},
    sota_reference::{SOTARepositoryImpl, SOTARepositoryImplParameters},
};

#[cfg(feature = "sqlite")]
use adapter::implement::sqlite::{
    activation::{ActivationRepositryImpl, ActivationRepositryImplParameters},
    healthcheck::{HealthCheckRepositryImpl, HealthCheckRepositryImplParameters},
    locator::{LocatorRepositryImpl, LocatorRepositryImplParameters},
    pota_reference::{POTARepositoryImpl, POTARepositoryImplParameters},
    sota_reference::{SOTARepositoryImpl, SOTARepositoryImplParameters},
};

module! {
    pub AppRegistry {
        components = [UserServiceImpl, AdminServiceImpl, AdminPeriodicServiceImpl,ActivationRepositryImpl,
        SOTARepositoryImpl,POTARepositoryImpl,
        LocatorRepositryImpl,GeoMagRepositryImpl,
        HealthCheckRepositryImpl],
        providers = [],
    }
}

impl AppRegistry {
    pub fn new(config: &AppConfig, pool: ConnectionPool) -> Self {
        AppRegistry::builder()
            .with_component_parameters::<SOTARepositoryImpl>(SOTARepositoryImplParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<POTARepositoryImpl>(POTARepositoryImplParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<ActivationRepositryImpl>(
                ActivationRepositryImplParameters { pool: pool.clone() },
            )
            .with_component_parameters::<LocatorRepositryImpl>(LocatorRepositryImplParameters {
                config: config.clone(),
                pool: pool.clone(),
            })
            .with_component_parameters::<UserServiceImpl>(UserServiceImplParameters {
                config: config.clone(),
            })
            .with_component_parameters::<AdminServiceImpl>(AdminServiceImplParameters {})
            .with_component_parameters::<AdminPeriodicServiceImpl>(
                AdminPeriodicServiceImplParameters {
                    config: config.clone(),
                },
            )
            .with_component_parameters::<GeoMagRepositryImpl>(GeoMagRepositryImplParameters {
                latest_data: Arc::new(Mutex::new(None)),
            })
            .with_component_parameters::<HealthCheckRepositryImpl>(
                HealthCheckRepositryImplParameters { pool: pool.clone() },
            )
            .build()
    }
}

#[derive(Clone)]
pub struct AppState {
    module: Arc<AppRegistry>,
}

impl AppState {
    pub fn new(module: AppRegistry) -> Self {
        Self {
            module: Arc::new(module),
        }
    }
}

impl FromRef<AppState> for Arc<AppRegistry> {
    fn from_ref(app_state: &AppState) -> Arc<AppRegistry> {
        app_state.module.clone()
    }
}

impl From<&AppState> for Arc<AppRegistry> {
    fn from(app_state: &AppState) -> Arc<AppRegistry> {
        app_state.module.clone()
    }
}
