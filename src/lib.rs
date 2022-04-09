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
    max_history: i64,
}

impl KFSM {
    pub fn new(initial: String, transitions: Box<[KFSMTransition]>, opts: Option<KFSMOptions>) -> KFSM {
        let o: KFSMOptions = opts.unwrap_or(KFSMOptions{max_history: -1});
        let k = KFSM {
            initial: initial.clone(),
            current: initial.clone(),
            transitions,
            history: vec![],
            options: o
        };
        return k
    }
    pub fn update(&mut self, next_state: String) -> Result<&mut KFSM, &str> {
        let mut changed = false;
        for t in self.transitions.iter() {
            if t.from.contains(&self.current.to_string()) && t.to == next_state {
                if self.history.len() >= self.options.max_history as usize {
                    self.history.remove(0);
                }
                self.history.push(KFSMHistory{from: self.current.clone(), to: next_state.clone()});
                self.current = next_state.clone();
                changed = true;
            }
        }
        if !changed {
            return Result::Err("update failed transition was not found")
        }

        return Result::Ok(self);
    }
}


#[cfg(test)]
mod test {
    use crate::{KFSM, KFSMTransition};

    #[test]
    fn test_new() {
        let ab: KFSMTransition = KFSMTransition{from: Box::from(["A".to_string()]), to: "B".to_string()};
        let t: Box<[KFSMTransition]> = Box::from([ab]);
        let k = KFSM::new("A".to_string(), t, None);
        assert_eq!(k.initial,"A".to_string());
        assert_eq!(k.current,"A".to_string());
        assert_eq!(k.history.len(),0);
        assert_eq!(k.options.max_history, -1)
    }

    #[test]
    fn test_update() {
        let ab: KFSMTransition = KFSMTransition{from: Box::from(["A".to_string()]), to: "B".to_string()};
        let bc: KFSMTransition = KFSMTransition{from: Box::from(["B".to_string()]), to: "C".to_string()};
        let t: Box<[KFSMTransition]> = Box::from([ab, bc]);
        let mut k = KFSM::new("A".to_string(), t, None);
        assert_eq!(k.initial,"A".to_string());
        assert_eq!(k.current,"A".to_string());
        assert_eq!(k.history.len(),0);
        assert_eq!(k.options.max_history, -1);
        k.update("B".to_string());
        assert_eq!(k.current, "B".to_string())
    }
}
