extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

use fadafada::control::{
    Controller,
};
use fadafada::control::graph::{
    ControllerGraph,
};
use fadafada::resolver::{
    SimpleResolverItem,
};
use fadafada::source::{
    Engine,
};
use fadafada::timing::{Scheduler};
use fadafada::yaml::{
    FromYaml,
    yaml_from_str,
};

#[wasm_bindgen]
pub struct GraphItem {
    t: u64,
    v: String,
    engine: String,
}

#[wasm_bindgen]
pub struct WasmControllerGraph {
    v: ControllerGraph,
}

impl WasmControllerGraph {
    pub fn new() -> WasmControllerGraph {
        let g: ControllerGraph = ControllerGraph::new();
        WasmControllerGraph{
            v: g,
        }
    }

    pub fn all(&mut self) -> Vec<GraphItem> {
        let mut gv: Vec<GraphItem> = Vec::<GraphItem>::new();
        loop {
            let v = self.v.next();
            match v {
                Some(x) => {
                    let g = GraphItem{
                        t: x.0,
                        v: x.1,
                        engine: x.2,
                    };
                    gv.push(g);
                },
                None => {
                    break;
                },
            }
        }
        gv
    }
}

#[wasm_bindgen]
pub struct WasmController {
    ctrl: Controller, 
}

#[wasm_bindgen]
pub struct WasmResolver {
    resolvers: HashMap<Engine, SimpleResolverItem>,
}

impl WasmResolver {
    pub fn new() -> WasmResolver {
        WasmResolver {
            resolvers: HashMap::new(),
        }
    }
}


#[wasm_bindgen]
impl WasmController {
    pub fn generate(&mut self, resolver: WasmResolver) -> WasmControllerGraph {
        WasmControllerGraph::new()
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
    #[test]
    fn test_from_yaml() {
        let s = "delay: 200
timeout: 4000
sources:
  - engine: foo
    endpoints:
      - url: file:///tmp/fadafada_curl/a
  - engine: bar
    endpoints:
      - url: file:///tmp/fadafada_curl/b
";
        let mut c = super::from_yaml(&s);
        let r = super::WasmResolver::new();
        let mut g = c.generate(r);
        g.all();

    }
}
