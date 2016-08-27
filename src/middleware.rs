use std::sync::Arc;
use tera::{Tera, Context};
use nickel::{Request, Response, Continue, Middleware, MiddlewareResult, Halt};
use typemap::Key;
use plugin::Extensible;

pub struct TeraMiddleware {
    pub tmpl: Arc<Tera>,
}

impl TeraMiddleware {
    pub fn new(path: &str) -> TeraMiddleware {
        TeraMiddleware { tmpl: Arc::new(Tera::new(path)) }
    }
}

impl Key for TeraMiddleware {
    type Value = Arc<Tera>;
}

impl<D> Middleware<D> for TeraMiddleware {
    fn invoke<'mw, 'conn>(&self,
                          req: &mut Request<'mw, 'conn, D>,
                          res: Response<'mw, D>)
                          -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<TeraMiddleware>(self.tmpl.clone());
        Ok(Continue(res))
    }
}

pub trait TeraRequestExtensions {
    fn template_engine(&self) -> Arc<Tera>;
}

impl<'a, 'b, D> TeraRequestExtensions for Request<'a, 'b, D> {
    fn template_engine(&self) -> Arc<Tera> {
        self.extensions().get::<TeraMiddleware>().unwrap().clone()
    }
}
