#[cfg(feature = "ffi")]
mod ffi;

use num::{Float as _, One, Zero};

use crate::core::{Function, StabilityMap, Transformation};
use crate::metrics::{AbsoluteDistance, SymmetricDistance};
use crate::domains::{AllDomain, BoundedDomain, SizedDomain, VectorDomain};
use crate::error::Fallible;
use crate::traits::{ExactIntCast, InfAdd, InfCast, InfDiv, InfMul, InfSub, Float};

use super::UncheckedSum;

pub fn make_sized_bounded_sum_of_squared_deviations<S>(
    size: usize,
    bounds: (S::Item, S::Item),
) -> Fallible<
    Transformation<
        SizedDomain<VectorDomain<BoundedDomain<S::Item>>>,
        AllDomain<S::Item>,
        SymmetricDistance,
        AbsoluteDistance<S::Item>,
    >,
>
where
    S: UncheckedSum,
    S::Item: 'static + Float,
{
    if size == 0 {
        return fallible!(MakeTransformation, "size must be greater than zero")
    }
    let size_ = S::Item::exact_int_cast(size)?;
    let (lower, upper) = bounds;
    let _1 = S::Item::one();

    // DERIVE RELAXATION TERM
    // Let x_bar_approx = x_bar + 2e, the approximate mean on finite data types
    // Let e = (n^2/2^k) / n, the mean error
    let mean_error = S::error(size, lower, upper)?.inf_div(&size_)?;

    // Let L' = L - e, U' = U + e
    let (lower, upper) = (lower.neg_inf_sub(&mean_error)?, upper.inf_add(&mean_error)?);

    // Let range = U' - L'
    let range = upper.inf_sub(&lower)?;

    // Let sens = range^2 * (n - 1) / n
    let sensitivity = range
        .inf_mul(&range)?
        .inf_mul(&size_.inf_sub(&_1)?)?
        .inf_div(&size_)?;

    // each deviation is bounded between 0 and range^2
    let relaxation = S::relaxation(size, S::Item::zero(), range.inf_mul(&range)?)?;

    // OVERFLOW CHECKS
    // Bound the magnitude of the sum when computing the mean
    lower.inf_mul(&size_)?;
    upper.inf_mul(&size_)?;
    // The squared difference from the mean is bounded above by range^2
    range.inf_mul(&range)?.inf_mul(&size_)?;

    Ok(Transformation::new(
        SizedDomain::new(VectorDomain::new(BoundedDomain::new_closed(bounds)?), size),
        AllDomain::new(),
        Function::new(move |arg: &Vec<S::Item>| {
            let mean = S::unchecked_sum(arg) / size_;
            S::unchecked_sum(
                &arg.iter()
                    .map(|v| (*v - mean).powi(2))
                    .collect::<Vec<S::Item>>(),
            )
        }),
        SymmetricDistance::default(),
        AbsoluteDistance::default(),
        // d_in / 2 * sensitivity + relaxation
        StabilityMap::new_fallible(move |d_in| {
            S::Item::inf_cast(d_in / 2)?
                .inf_mul(&sensitivity)?
                .inf_add(&relaxation)
        }),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{error::ExplainUnwrap, transformations::Pairwise};

    use super::*;

    #[test]
    fn test_make_bounded_deviations() {
        let arg = vec![1., 2., 3., 4., 5.];

        let transformation_sample =
            make_sized_bounded_sum_of_squared_deviations::<Pairwise<_>>(5, (0., 10.)).unwrap_test();
        let ret = transformation_sample.invoke(&arg).unwrap_test();
        let expected = 10.;
        assert_eq!(ret, expected);
        assert!(transformation_sample.check(&1, &(100. / 5.)).unwrap_test());

        let transformation_pop =
            make_sized_bounded_sum_of_squared_deviations::<Pairwise<_>>(5, (0., 10.)).unwrap_test();
        let ret = transformation_pop.invoke(&arg).unwrap_test();
        let expected = 10.0;
        assert_eq!(ret, expected);
        assert!(transformation_pop
            .check(&1, &(100. * 4. / 25.))
            .unwrap_test());
    }
}
