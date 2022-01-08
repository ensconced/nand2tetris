pub struct NandGate;

impl NandGate {
    fn output(in_a: bool, in_b: bool) -> bool {
        !(in_a && in_b)
    }
}
