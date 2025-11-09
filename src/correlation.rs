pub fn kendall_tau(x: &[f64], y: &[f64]) -> f64{
    let n = x.len();
    let mut concordant = 0;
    let mut discordant = 0;
    
    for i in 0..n-1 {
        for j in i+1..n {
            let x_diff = x[j] - x[i];
            let y_diff = y[j] - y[i];
            
            if x_diff * y_diff > 0.0 {
                concordant += 1;
            } else if x_diff * y_diff < 0.0 {
                discordant += 1;
            }
        }
    }
    
    let total_pairs = (n * (n - 1)) / 2;
    (concordant - discordant) as f64 / total_pairs as f64
}

pub fn spearman_rho(x: &[f64], y: &[f64]) -> f64{
    todo!()
}