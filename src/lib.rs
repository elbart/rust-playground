use std::cell::RefCell;

#[derive(Debug)]
struct YakeTarget {
    pub name: String,
    pub dependencies: Option<Vec<YakeTarget>>,
}

#[derive(Debug)]
struct Yake {
    pub targets: Vec<YakeTarget>
}

impl Yake {
    fn new(targets: Vec<YakeTarget>) -> Self {
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
        let t1 = YakeTarget {
            name: "psql".into(),
            dependencies: None
        };

        let t2 = YakeTarget {
            name: "docker".into(),
            dependencies: Some(vec![&t1])
        };

        let targets = vec![t1, t2];
        let y = Yake::new(targets);

        println!("{:#?}", y.targets);

    }
}
