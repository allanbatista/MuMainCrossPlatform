pub use crate::routes::UiRouteDescriptor as UiFixtureRoute;

pub const FIXTURE_ROUTES: &[UiFixtureRoute] = crate::routes::ROUTES;

pub fn fixture_routes() -> &'static [UiFixtureRoute] {
    FIXTURE_ROUTES
}
