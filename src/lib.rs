pub mod biguint;

type BigUint256 = biguint::BigUint<4>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biguint() {
        let one = BigUint256::one();

        println!("one {:?}", one);

        let zero = BigUint256::zero();

        println!("one {:?}", zero);

        println!("{}", one == BigUint256::new([1u64; 4]));

        println!("{}", zero == BigUint256::new([0u64; 4]));
    }

    #[test]
    fn test_adc() {
        let mut a = [0xffffffffffffffffu64; 4];
        a[3] = 0x1;
        a[2] = 0x1;
        let res = BigUint256::new(a) + &BigUint256::one();
        for num in res.0.iter() {
            print!("{:0x} ", num);
        }
    }

    #[test]
    fn test_mul() {
        let one = BigUint256::new([0xffffffffffffffff, 0x2000000000000000, 0x3000000000000000, 0x4000000000000000]);
        let zero = BigUint256::new([0x5000000000000000, 0x6000000000000000, 0x7000000000000000, 0x9000000000000000]);
        // 200000000000000034000000000000003d00000000000000b4000000000000000afffffffffffffff9ffffffffffffffefffffffffffffffb000000000000000 = 0x400000000000000030000000000000002000000000000000ffffffffffffffff * 0x8000000000000000700000000000000060000000000000005000000000000000
        let res = one * &zero;
        println!("{:?}", res);
    }
}
