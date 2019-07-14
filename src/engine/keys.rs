pub struct Keys {
    pub w: bool,
    pub a: bool,
    pub s: bool, 
    pub d: bool,
    pub q: bool,
    pub e: bool,
}

impl Keys {
    pub fn init() -> Keys {
        let w = false;
        let a = false;
        let s = false;
        let d = false;
        let q = false;
        let e = false;
        Keys {
            w,a,s,d,q,e
        }
    }
}