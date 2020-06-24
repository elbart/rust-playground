use std::rc::Rc;

#[derive(Debug)]
struct YakeTarget {
    pub name: String,
    pub dependencies: Option<Vec<Rc<YakeTarget>>>,
}

#[derive(Debug)]
struct Yake {
    pub targets: Vec<Rc<YakeTarget>>
}

impl Yake {
    fn new(targets: Vec<Rc<YakeTarget>>) -> Self {
        Yake {
            targets: targets
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_layout() {
        let t1 = Rc::new(YakeTarget {
            name: "psql".into(),
            dependencies: None
        });

        let t2 = Rc::new(YakeTarget {
            name: "docker".into(),
            dependencies: Some(vec![t1.clone()])
        });

        let targets = vec![t1, t2];
        let y = Yake::new(targets);

        println!("{:#?}", y.targets);

    }
}
