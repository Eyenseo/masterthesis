struct Pinky;
struct Becci;
trait Who { fn who(&self); }

impl Who for Pinky {
    fn who(&self) {
        println!("Pinky")
    }
}
impl Who for Becci {
    fn who(&self) {
        println!("Becci")
    }
}

fn fun_gen<T: Who>(obj: &T) {
    obj.who()
}
fn fun_obj(obj: &Who) {
    obj.who()
}

fn main() {
    let p = Pinky;
    let b = Becci;

    fun_gen(&p);
    fun_obj(&b);
}
