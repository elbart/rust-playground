use std::rc::Rc;

#[derive(Debug)]
struct YakeTarget {
    pub name: String,
    pub dependencies: Vec<Rc<YakeTarget>>,
    pub sub_targets: Vec<Rc<YakeTarget>>,
}

#[derive(Debug)]
struct Yake {
    pub targets: Vec<Rc<YakeTarget>>,
}

impl YakeTarget {
    pub fn new(name: String) -> Self {
        YakeTarget {
            name,
            dependencies: Vec::new(),
            sub_targets: Vec::new(),
        }
    }

    pub fn dependency(mut self, dep: Rc<YakeTarget>) -> Self {
        self.dependencies.push(dep);
        self
    }

    pub fn sub_target(mut self, dep: Rc<YakeTarget>) -> Self {
        self.sub_targets.push(dep);
        self
    }
}

impl Yake {
    pub fn new(targets: Vec<Rc<YakeTarget>>) -> Self {
        Yake { targets: targets }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_layout() {
        let t1 = Rc::new(YakeTarget::new("psql".into()));
        let t2 = Rc::new(YakeTarget::new("docker".into()).dependency(Rc::clone(&t1)));

        let t3 = Rc::new(
            YakeTarget::new("foo".into())
                .dependency(Rc::clone(&t1))
                .dependency(Rc::clone(&t2))
                .sub_target(Rc::clone(&t2)),
        );

        let targets = vec![t1, t2, t3];
        let y = Yake::new(targets);

        println!("{:#?}", y.targets);
        println!("{:#?}", y.targets.get(1).unwrap().dependencies);
    }
}
