#[cfg(feature = "ffi")]
mod ffi;

use crate::core::{Measurement, PrivacyMap};
use crate::error::*;
use crate::measures::MaxDivergence;
use crate::traits::samplers::SampleDiscreteLaplaceLinear;
use crate::traits::{Float, InfCast, Integer};

use super::DiscreteLaplaceDomain;

pub fn make_base_discrete_laplace_linear<D, QO>(
    scale: QO,
    bounds: Option<(D::Atom, D::Atom)>,
) -> Fallible<Measurement<D, D, D::InputMetric, MaxDivergence<QO>>>
where
    D: DiscreteLaplaceDomain,
    D::Atom: Integer + SampleDiscreteLaplaceLinear<QO>,
    QO: Float + InfCast<D::Atom>,
{
    if scale.is_sign_negative() {
        return fallible!(MakeMeasurement, "scale must not be negative");
    }
    if bounds
        .as_ref()
        .map(|(lower, upper)| lower > upper)
        .unwrap_or(false)
    {
        return fallible!(MakeMeasurement, "lower may not be greater than upper");
    }

    Ok(Measurement::new(
        D::default(),
        D::default(),
        D::new_map_function(move |v: &D::Atom| {
            D::Atom::sample_discrete_laplace_linear(*v, scale, bounds)
        }),
        D::InputMetric::default(),
        MaxDivergence::default(),
        PrivacyMap::new_fallible(move |d_in: &D::Atom| {
            let d_in = QO::inf_cast(*d_in)?;
            if d_in.is_sign_negative() {
                return fallible!(InvalidDistance, "sensitivity must be non-negative");
            }
            if d_in.is_zero() {
                return Ok(QO::zero());
            }
            if scale.is_zero() {
                return Ok(QO::infinity());
            }
            // d_in / scale
            d_in.inf_div(&scale)
        }),
    ))
}

#[deprecated(
    since = "0.5.0",
    note = "Use `make_base_discrete_laplace` instead. For a constant-time algorithm, pass bounds into `make_base_discrete_laplace_linear`."
)]
pub fn make_base_geometric<D, QO>(
    scale: QO,
    bounds: Option<(D::Atom, D::Atom)>,
) -> Fallible<Measurement<D, D, D::InputMetric, MaxDivergence<QO>>>
where
    D: DiscreteLaplaceDomain,
    D::Atom: Integer + SampleDiscreteLaplaceLinear<QO>,
    QO: Float + InfCast<D::Atom>,
{
    make_base_discrete_laplace_linear(scale, bounds)
}

#[cfg(test)]
mod tests {
    use crate::domains::{AllDomain, VectorDomain};

    use super::*;

    #[test]
    fn test_make_discrete_laplace_mechanism_bounded() {
        let measurement =
            make_base_discrete_laplace_linear::<AllDomain<_>, f64>(10.0, Some((200, 210)))
                .unwrap_test();
        let arg = 205;
        let _ret = measurement.invoke(&arg).unwrap_test();
        println!("{:?}", _ret);

        assert!(measurement.check(&1, &0.5).unwrap_test());
    }

    #[test]
    fn test_make_vector_discrete_laplace_mechanism_bounded() {
        let measurement =
            make_base_discrete_laplace_linear::<VectorDomain<_>, f64>(10.0, Some((200, 210)))
                .unwrap_test();
        let arg = vec![1, 2, 3, 4];
        let _ret = measurement.invoke(&arg).unwrap_test();
        println!("{:?}", _ret);

        assert!(measurement.check(&1, &0.5).unwrap_test());
    }

    #[test]
    fn test_make_discrete_laplace_mechanism() {
        let measurement =
            make_base_discrete_laplace_linear::<AllDomain<_>, f64>(10.0, None).unwrap_test();
        let arg = 205;
        let _ret = measurement.invoke(&arg).unwrap_test();
        println!("{:?}", _ret);

        assert!(measurement.check(&1, &0.5).unwrap_test());
    }

    #[test]
    fn test_make_vector_discrete_laplace_mechanism() {
        let measurement =
            make_base_discrete_laplace_linear::<VectorDomain<_>, f64>(10.0, None).unwrap_test();
        let arg = vec![1, 2, 3, 4];
        let _ret = measurement.invoke(&arg).unwrap_test();
        println!("{:?}", _ret);

        assert!(measurement.check(&1, &0.5).unwrap_test());
    }
}
