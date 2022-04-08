use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::string::String;

#[derive(Serialize, Deserialize)]
struct KFSMTransition {
    from: Box<[String]>,
    to: String,
}


#[derive(Serialize, Deserialize)]
struct KFSMHistory {
    from: String,
    to: String
}

#[derive(Serialize, Deserialize)]
struct KFSM {
    initial: String,
    current: String,
    transitions: Box<[KFSMTransition]>,
    history: Vec<KFSMHistory>,
    options: KFSMOptions
}

#[derive(Serialize, Deserialize)]
struct KFSMOptions {
    maxHistory: int64,
}

impl KFSM {
    pub fn new(initial: String, transitions: Box<[KFSMTransition]>, opts: KFSMOptions) -> KFSM {
        let o: KFSMOptions = KFSMOptions{ maxHistory: opts.maxHistory || -1 };
        let k = KFSM {
            initial: initial.clone(),
            current: initial.clone(),
            transitions,
            history: vec![],
            options: o
        };
        return k
    }
    pub fn update(&mut self, nextState: String) -> Result<&mut KFSM, &str> {
        let mut changed = false;
        for t in self.transitions.iter() {
            if t.from.contains(&self.current.to_string()) && t.to == nextState {
                if self.history.len() != -1 && self.history.len() > self.options.maxHistory {
                    self.history.remove(0);
                }
                self.history.push(KFSMHistory{from: self.current.clone(), to: nextState.clone()});
                self.current = nextState.clone();
                changed = true;
            }
        }
        if !changed {
            return Result::Err(JsError::new("update failed transition was not found"))
        }

        return Result::Ok(self);
    }
}

