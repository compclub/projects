mod circuit_tests {
    use jp_quantum::gates::*;
    use jp_quantum::{QCircuit, Vector};

    // for dealing with floating point imprecision
    fn similar(v1: Vector, v2: Vector) -> bool {
        (v1 - v2).norm() < 0.001
    }

    #[test]
    fn boring_hadamard_test() {
        let circuit = h();
        let output = circuit.run();
        assert_eq!(format!("{}", output.measure()), "0.500: 0\n0.500: 1\n");
    }

    #[test]
    // From https://medium.com/@adubey40/quantum-circuit-d53b5485ef91
    fn hadamard_test() {
        let circuit = id() * x() + h() * h();
        let output = circuit.run();
        let expected = Vector::new(2, &[ONE / 2.0, -ONE / 2.0, ONE / 2.0, -ONE / 2.0]);
        assert!(similar(output, expected));
    }

    #[test]
    fn cz_test() {
        let circuit = h() * h() + cz();
        let output = circuit.run();
        let expected = Vector::new(2, &[ONE / 2.0, ONE / 2.0, ONE / 2.0, -ONE / 2.0]);
        assert!(similar(output, expected));
    }

    #[test]
    fn not_cz_test() {
        let circuit = h() * h() + x() * x() + cz() + x() * x();
        let output = circuit.run();
        let expected = Vector::new(2, &[-ONE / 2.0, ONE / 2.0, ONE / 2.0, ONE / 2.0]);
        assert!(similar(output, expected));
    }

    #[test]
    fn z_cz_test() {
        let circuit = h() * h() + z() * z() + cz();
        let output = circuit.run();
        let expected = Vector::new(2, &[ONE / 2.0, -ONE / 2.0, -ONE / 2.0, -ONE / 2.0]);
        assert!(similar(output, expected));
    }

    #[test]
    fn grovers_search_2qbit_test() {
        let oracle_00 = x() * x() + cz() + x() * x();
        let oracle_01 = x() * id() + cz() + x() * id();
        let oracle_10 = id() * x() + cz() + id() * x();
        let oracle_11 = cz();
        fn amplifier() -> QCircuit {
            z() * z() + cz()
        }

        let circuit_00 = h() * h() + oracle_00 + h() * h() + amplifier() + h() * h();
        let circuit_01 = h() * h() + oracle_01 + h() * h() + amplifier() + h() * h();
        let circuit_10 = h() * h() + oracle_10 + h() * h() + amplifier() + h() * h();
        let circuit_11 = h() * h() + oracle_11 + h() * h() + amplifier() + h() * h();

        assert!(similar(
            circuit_00.run(),
            Vector::new(2, &[ONE, ZERO, ZERO, ZERO])
        ));
        assert!(similar(
            circuit_01.run(),
            Vector::new(2, &[ZERO, ONE, ZERO, ZERO])
        ));
        assert!(similar(
            circuit_10.run(),
            Vector::new(2, &[ZERO, ZERO, ONE, ZERO])
        ));
        assert!(similar(
            circuit_11.run(),
            Vector::new(2, &[ZERO, ZERO, ZERO, ONE])
        ));
    }

    #[test]
    fn grovers_search_3qbit_test() {
        // Selects 011 and 110
        let oracle = cz() * id() + id() * cz();
        let amplifier = x() * x() * x() + ccz() + x() * x() * x();

        let circuit = h() * h() * h() + oracle + h() * h() * h() + amplifier + h() * h() * h();
        let expected = Vector::new(
            3,
            &[
                ZERO,
                ZERO,
                ZERO,
                -0.7071 * ONE,
                ZERO,
                ZERO,
                -0.7071 * ONE,
                ZERO,
            ],
        );
        assert!(similar(circuit.run(), expected));
    }
}
