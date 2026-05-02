mod bandwith_calculator;
fn main() {
    bandwith_calculator::run();
}

// --- UNIT TESTS ---
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_calculate() {
        let result = 4*25;
        assert_eq!(result,100);
    }
}