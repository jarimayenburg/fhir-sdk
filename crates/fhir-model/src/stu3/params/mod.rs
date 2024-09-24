//! Search parameter definitions.

#[rustfmt::skip] // Too much for rustfmt
mod generated;

pub use generated::*;

use crate::Resolve;

use super::resources;

impl Resolve for resources::Observation {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			ObservationSearchParameter::Date => match self.effective.as_ref()? {
				resources::ObservationEffective::DateTime(dt) => Some(dt.clone()),
				resources::ObservationEffective::Period(p) if p.start.is_some() => p.start.clone(),
				resources::ObservationEffective::Period(p) => p.end.clone(),
			},
			_ => unimplemented!("Currently only Observation:date is implemented"),
		}
	}
}

impl Resolve for resources::Immunization {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			ImmunizationSearchParameter::Date => self.date.as_ref(),
			_ => unimplemented!("Currently only Immunization:date is implemented"),
		}
	}
}
