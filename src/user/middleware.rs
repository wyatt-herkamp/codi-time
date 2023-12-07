use std::{rc::Rc, sync::Arc};

use actix_service::{forward_ready, Service, Transform};
use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{ServiceRequest, ServiceResponse},
    http::{header, header::HeaderValue, Method},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use tracing::{instrument, log::warn, trace};

use crate::user::{
    session::{Session, SessionManager, SessionManagerType},
    AuthenticationRaw,
};

pub struct HandleSession {
    pub session_manager: Arc<SessionManagerType>,
}

impl<S, B> Transform<S, ServiceRequest> for HandleSession
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Transform = SessionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionMiddleware {
            service: Rc::new(service),
            session_manager: self.session_manager.clone(),
        }))
    }
}
pub struct SessionMiddleware<S> {
    service: Rc<S>,
    session_manager: Arc<SessionManagerType>,
}

impl<S, B> SessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    #[instrument]
    async fn handle_session(
        session_manager: Arc<SessionManagerType>,
        req: &ServiceRequest,
        cookie_value: &str,
    ) -> Result<(), HttpResponse> {
        let session: Option<Session> = match session_manager.get_session(cookie_value) {
            Ok(ok) => ok,
            Err(e) => {
                warn!("Session Manager Error: {}", e);
                return Err(HttpResponse::InternalServerError().body("Session Manager Error"));
            }
        };
        if let Some(session) = session {
            let raw = AuthenticationRaw::Session(session);
            req.extensions_mut().insert(raw);
        }
        Ok(())
    }
    ///
    /// Inserts the RawAuthentication into the request extension
    /// # Arguments
    ///
    /// * `req`: A reference to the request
    /// * `auth`: The authorization header
    /// * `session_manager`:  The session manager.
    /// Will be unused if feature `allow-session-in-header` is not enabled
    ///
    /// returns: Result<(), HttpResponse<BoxBody>>
    ///   - Ok: You can continue with the request
    ///  - Err: The error - Will be an error response
    #[instrument]
    async fn handle_auth_header(
        req: &ServiceRequest,
        auth: &HeaderValue,
        session_manager: Arc<SessionManagerType>,
    ) -> Result<(), HttpResponse> {
        let Ok(str) = auth.to_str() else {
            return Err(HttpResponse::BadRequest().body("Invalid Authorization Header"));
        };

        let auth = str.splitn(2, ' ').collect::<Vec<&str>>();
        if auth.len() != 2 {
            return Err(
                HttpResponse::BadRequest().body(r#"Invalid Authorization Header. Please use Format "Bearer <token>" or "Session <session>""#),
            );
        }
        trace!("Auth: {:?}", auth);
        let session_config = session_manager.get_session_config_ref();
        if auth[0] == "Bearer" {
            let raw = AuthenticationRaw::APIToken(auth[1].to_owned());
            req.extensions_mut().insert(raw);
        } else if session_config.allow_in_header && auth[0] == session_config.cookie_name {
            return Self::handle_session(session_manager, req, auth[1]).await;
        } else {
            return Err(HttpResponse::BadRequest()
                .body(format!("Unsupported Authorization Header: {}", auth[0])));
        }
        Ok(())
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `service`: A clone of the service
    /// * `req`: The request
    /// * `session_manager`:  The session manager
    ///
    /// returns: Result<ServiceResponse<EitherBody<B, BoxBody>>, Error>
    ///    - Ok: The response  - Will just be the call to the next handler
    ///   - Err: The error - Will be an error response
    #[instrument(skip(service))]
    async fn handle_authentication(
        service: Rc<S>,
        req: ServiceRequest,
        session_manager: Arc<SessionManagerType>,
    ) -> Result<ServiceResponse<EitherBody<B, BoxBody>>, Error> {
        if let Some(auth) = req.headers().get(header::AUTHORIZATION) {
            if let Err(e) = Self::handle_auth_header(&req, auth, session_manager).await {
                return Ok(req.into_response(e.map_into_right_body()));
            }
        } else if let Some(cookie) = req.cookie("session") {
            if let Err(e) = Self::handle_session(session_manager, &req, cookie.value()).await {
                return Ok(req.into_response(e.map_into_right_body()));
            }
        }
        let fut = service.call(req);

        let res = fut.await?;
        Ok(res.map_into_left_body())
    }
}
impl<S, B> Service<ServiceRequest> for SessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Check if its an OPTIONS request. If so exit early and let the request pass through
        if req.method() == Method::OPTIONS {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            });
        }
        // Move into an async block
        let session =
            Self::handle_authentication(self.service.clone(), req, self.session_manager.clone());
        Box::pin(async move {
            let res = session.await?;
            Ok(res)
        })
    }
}
