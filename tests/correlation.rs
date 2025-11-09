use copulae::correlation::kendall_tau;

const X: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
const Y: [f64; 4] = [10.0, 20.0, 30.0, 40.0];
const Z: [f64; 4] = [400.0, 300.0, 200.0, 100.0];

#[test]
fn test_perfect_correlation() {
    let tau_1 = kendall_tau(&X, &Y);
    assert_eq!(tau_1, 1.0);
}

#[test]
fn test_perfect_anticorrelation() {
    let tau_minus1 = kendall_tau(&X, &Z);
    assert_eq!(tau_minus1, -1.0)
}