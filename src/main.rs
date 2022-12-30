use pyo3::Python;

fn main() -> eyre::Result<()> {
    Python::with_gil(|py| {
        let np_round = py.import("numpy")?.getattr("round")?;
        let np_round = |v| np_round.call1((v,));

        for v in -10000..=10000 {
            let v = v as f64;
            for v in [v, v + 0.1, v + 0.4, v + 0.5, v + 0.6, v + 0.9] {
                let python = np_round(v)?.extract::<f64>()?;

                let rust = roundeven_by_llvm(v);
                assert_eq!(python, rust);

                let rust = roundeven_by_libm_rs(v);
                assert_eq!(python, rust);

                let rust = libm::rint(v);
                assert_eq!(python, rust);
            }
        }
        Ok(())
    })
}

fn roundeven_by_llvm(v: f64) -> f64 {
    let mut rounded = v.round();
    if (v - rounded).abs() == 0.5 {
        rounded = 2. * (v / 2.).round();
    }
    rounded
}

fn roundeven_by_libm_rs(v: f64) -> f64 {
    let mut rounded = libm::round(v);
    if libm::fabs(v - rounded) == 0.5 {
        rounded = 2. * libm::round(v / 2.);
    }
    rounded
}
