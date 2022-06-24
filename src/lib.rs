use std::fmt;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use log::debug;

use hex;

use std::collections::HashMap;

use fadfada::control::{
    Controller,
};
use fadfada::control::graph::{
    ControllerGraph,
};
use fadfada::resolver::{
    ResolverError,
    ErrorDetail,
    Digest,
    Signature,
};
use fadfada::source::{
    Engine,
};
use fadfada::yaml::{
    FromYaml,
    yaml_from_str,
};


#[wasm_bindgen]
pub struct WasmResolverError {
}


impl fmt::Debug for WasmResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resolver error debug")
    }
}

#[wasm_bindgen]
pub struct WasmControllerGraph {
    graph: ControllerGraph,
}

impl WasmControllerGraph {
    pub fn new(graph: ControllerGraph) -> WasmControllerGraph {
        WasmControllerGraph{
            graph: graph,
        }
    }
}

#[wasm_bindgen]
impl WasmControllerGraph {

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn get_url(&self, i: usize) -> String {
        return self.graph.get(i).1;
    }

    pub fn get_offset(&self, i: usize) -> u64 {
        return self.graph.get(i).0;
    }
}

#[wasm_bindgen]
pub struct WasmController {
    ctrl: Controller, 
}

#[wasm_bindgen]
pub struct WasmResolver {
    resolvers: HashMap<Engine, WasmResolverItem>,
}

#[wasm_bindgen]
pub struct WasmResolverItem {
    digest: Digest,
    src: String,
}

#[wasm_bindgen]
impl WasmResolverItem {
    pub fn new(content: String) -> WasmResolverItem {
        WasmResolverItem{
            digest: hex::decode(&content).unwrap(),
            src: content,
        }
    }

    fn digest(&self) -> &Digest {
        return &self.digest;
    }

    fn pointer(&self) -> String {
        return self.src.clone();
    }

    fn signature(&self) -> Result<Signature, WasmResolverError> {
        //let err = ResolverError::new(ErrorDetail::UnknownEngineError);
        let err = WasmResolverError{};
        return Err(err);
    }
} 

#[wasm_bindgen]
impl WasmResolver {
    pub fn new() -> WasmResolver {
        WasmResolver {
            resolvers: HashMap::new(),
        }
    }
    /// Register a [ResolverItem] for an [source::Engine].
    /// 
    /// Will error if a record for [source::Engine] already exists.
    pub fn add(&mut self, e: Engine, r: WasmResolverItem) -> Result<(), WasmResolverError> {
        if self.resolvers.contains_key(&e) {
        //    let e = ResolverError::new(ErrorDetail::EngineExistsError);
            let e = WasmResolverError{};
            return Err(e);
        }
        debug!(">>>>> add resolver {}", e);
        self.resolvers.insert(e, r);
        Ok(())
    }
}

impl WasmResolver {
    /// Retrieve the [ResolverItem] registered for an [source::Engine].
    /// 
    /// Will error if a record for `Engine` doesn't exist.
    pub fn pointer_for(&self, e: &Engine) -> Result<String, WasmResolverError> {
        match self.resolvers.get(e) {
            Some(x) => {
                Ok(x.pointer())
            },
            None => {
                //let err_detail = ErrorDetail::UnknownEngineError;
                //let err = ResolverError::new(err_detail);
                let err = WasmResolverError{};
                return Err(err);
            },
        }
    }
}

#[wasm_bindgen]
impl WasmController {
    pub fn generate(&mut self, resolver: WasmResolver) -> WasmControllerGraph {
        let mut g: ControllerGraph = ControllerGraph::new();
        self.ctrl.sources.iter().enumerate().for_each(|(i, s)| {
            debug!("processing source {:?}", s);
            s.endpoints.iter().enumerate().for_each(|(j, e)| {
                let mut offset: u32 = self.ctrl.offsets[i] as u32;
                match &s.timing {
                    Some(x) => {
                        let pointer = resolver.pointer_for(&s.engine).unwrap();
                        offset += x.delay * (j as u32);
                        let pointer_url = e.url_for(&pointer);
                        g.add(offset as u64, &s.engine, pointer_url); //.url_for(&pointer));
                    },
                    None => {},
                }
               //write!(f, "{} {} {} {}\n", i, j, offset, e);
            });
        });
        g.keys();
        WasmControllerGraph{
            graph: g,
        }
    }
}

#[wasm_bindgen]
pub fn from_yaml(s: &str) -> WasmController {
    let yaml = yaml_from_str(s);
    let ctrl_under = Controller::from_yaml(&yaml, None);
    let ctrl = WasmController{
        ctrl: ctrl_under,
    };
    return ctrl;
}

#[cfg(test)]
mod tests {
    use log::debug;
    #[test]
    fn test_from_yaml() {
        env_logger::init();
        let s = "delay: 200
timeout: 4000
sources:
  - engine: foo
    endpoints:
      - url: file:///tmp/fadfada_curl/a
  - engine: bar
    endpoints:
      - url: file:///tmp/fadfada_curl/b
";
        let mut c = super::from_yaml(&s);
        let mut r = super::WasmResolver::new();
        let mut t = super::WasmResolverItem::new("deadbeef".to_string());
        r.add("foo".to_string(), t);
        t = super::WasmResolverItem::new("beeffeed".to_string());
        r.add("bar".to_string(), t);
        let mut g = c.generate(r);
        //let x = g.int|();
        debug!(">>>>> l {} g {} {}", g.len(), g.get(0), g.get(1));
    }
}
