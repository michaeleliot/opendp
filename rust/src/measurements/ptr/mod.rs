#[cfg(feature="ffi")]
mod ffi;

/// Algorithms that depend on the propose-test-release framework.

use std::collections::HashMap;

use crate::core::{Function, Measurement, PrivacyMap};
use crate::metrics::L1Distance;
use crate::measures::{SmoothedMaxDivergence, SMDCurve};
use crate::domains::{AllDomain, MapDomain};
use crate::error::Fallible;
use crate::traits::samplers::SampleDiscreteLaplaceZ2k;
use crate::traits::{Float, Hashable, ExactIntCast};

use super::get_discretization_consts;

// propose-test-release count grouped by unknown categories,
// IMPORTANT: Assumes that dataset distance is bounded above by d_in.
//  This assumption holds for count queries in L1-space.
pub fn make_base_ptr<TK, TV>(
    scale: TV, threshold: TV, k: Option<i32>
) -> Fallible<Measurement<MapDomain<AllDomain<TK>, AllDomain<TV>>, MapDomain<AllDomain<TK>, AllDomain<TV>>, L1Distance<TV>, SmoothedMaxDivergence<TV>>>
    where TK: Hashable,
          TV: Float + SampleDiscreteLaplaceZ2k,
          i32: ExactIntCast<TV::Bits> {
    let _2 = TV::exact_int_cast(2)?;
    let (k, relaxation) = get_discretization_consts(k)?;
    Ok(Measurement::new(
        MapDomain::new(AllDomain::new(), AllDomain::new()),
        MapDomain::new(AllDomain::new(), AllDomain::new()),
        Function::new_fallible(move |data: &HashMap<TK, TV>| {
            data.clone().into_iter()
                // noise output count
                .map(|(key, v)| TV::sample_discrete_laplace_Z2k(v, scale, k).map(|v| (key, v)))
                // remove counts that fall below threshold
                .filter(|res| res.as_ref().map(|(_k, c)| c >= &threshold).unwrap_or(true))
                // fail the whole computation if any cast or noise addition failed
                .collect()
        }),
        L1Distance::default(),
        SmoothedMaxDivergence::default(),
        PrivacyMap::new_fallible(move |&d_in: &TV| {
            Ok(SMDCurve::new(
                move |&del: &TV| {
                    if del.is_sign_negative() || del.is_zero() {
                        return fallible!(FailedRelation, "delta must be positive");
                    }
                    let d_in = d_in.inf_add(&relaxation)?;
                    let min_eps = d_in / scale;
                    let min_threshold = (d_in / (_2 * del)).ln() * scale + d_in;
                    if threshold < min_threshold {
                        return fallible!(RelationDebug, "threshold must be at least {:?}", min_threshold);
                    }
                    Ok(min_eps)
                }
            ))
        })))
}


#[cfg(test)]
mod tests {
    use crate::transformations::make_count_by;
    use super::*;

    #[test]
    fn test_count_by_ptr() -> Fallible<()> {

        let max_influence = 1;
        let sensitivity = max_influence as f64;
        let epsilon = 2.;
        let delta = 1e-6;
        let scale = sensitivity / epsilon;
        let threshold = (max_influence as f64 / (2. * delta)).ln() * scale + max_influence as f64;
        println!("{:?}", threshold);

        let measurement = (
            make_count_by()? >>
            make_base_ptr::<char, f64>(scale, threshold, None)?
        )?;
        let ret = measurement.invoke(
            &vec!['a', 'b', 'a', 'a', 'a', 'a', 'b', 'a', 'a', 'a', 'a'])?;
        println!("stability eval: {:?}", ret);

        let epsilon_p = measurement.map(&max_influence)?.epsilon(&delta)?;
        assert_eq!(epsilon_p, epsilon);
        Ok(())
    }
}