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
				resources::ObservationEffective::DateTime(dt) => Some(dt),
				resources::ObservationEffective::Period(p) if p.start.is_some() => p.start.as_ref(),
				resources::ObservationEffective::Period(p) => p.end.as_ref(),
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

impl Resolve for resources::ImmunizationRecommendation {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			ImmunizationRecommendationSearchParameter::Date => {
				self.recommendation.iter().flatten().map(|r| &r.date).next()
			}
			_ => unimplemented!("Currently only ImmunizationRecommendation:date is implemented"),
		}
	}
}

impl Resolve for resources::EpisodeOfCare {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			EpisodeOfCareSearchParameter::Date => match self.period.as_ref()? {
				p if p.start.is_some() => p.start.as_ref(),
				p => p.end.as_ref(),
			},
			_ => unimplemented!("Currently only EpisodeOfCare:date is implemented"),
		}
	}
}

impl Resolve for resources::Encounter {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			EncounterSearchParameter::Date => match self.period.as_ref()? {
				p if p.start.is_some() => p.start.as_ref(),
				p => p.end.as_ref(),
			},
			_ => unimplemented!("Currently only Encounter:date is implemented"),
		}
	}
}

impl Resolve for resources::DiagnosticReport {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			DiagnosticReportSearchParameter::Date => match self.effective.as_ref()? {
				resources::DiagnosticReportEffective::DateTime(dt) => Some(dt),
				resources::DiagnosticReportEffective::Period(p) if p.start.is_some() => {
					p.start.as_ref()
				}
				resources::DiagnosticReportEffective::Period(p) => p.end.as_ref(),
			},
			_ => unimplemented!("Currently only DiagnosticReport:date is implemented"),
		}
	}
}

impl Resolve for resources::Consent {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			ConsentSearchParameter::Date => self.date_time.as_ref(),
			_ => unimplemented!("Currently only Consent:date is implemented"),
		}
	}
}

impl Resolve for resources::Flag {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			FlagSearchParameter::Date => match self.period.as_ref()? {
				p if p.start.is_some() => p.start.as_ref(),
				p => p.end.as_ref(),
			},
			_ => unimplemented!("Currently only Flag:date is implemented"),
		}
	}
}

impl Resolve for resources::MedicationRequest {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			MedicationRequestSearchParameter::Date => self
				.dosage_instruction
				.iter()
				.flatten()
				.find_map(|di| di.timing.as_ref())
				.map(|t| &t.event),
			_ => unimplemented!("Currently only MedicationRequest:date is implemented"),
		}
	}
}

impl Resolve for resources::NutritionOrder {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			NutritionOrderSearchParameter::Datetime => Some(&self.date_time),
			_ => unimplemented!("Currently only NutritionOrder:datetime is implemented"),
		}
	}
}

impl Resolve for resources::DocumentReference {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			DocumentReferenceSearchParameter::Indexed => Some(&self.indexed),
			_ => unimplemented!("Currently only DocumentReference:indexed is implemented"),
		}
	}
}

impl Resolve for resources::AllergyIntolerance {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			AllergyIntoleranceSearchParameter::Date => self.asserted_date.as_ref(),
			_ => unimplemented!("Currently only AllergyIntolerance:date is implemented"),
		}
	}
}

impl Resolve for resources::Condition {
	fn resolve(&self, param: &Self::Params) -> Option<impl Ord> {
		match param {
			ConditionSearchParameter::AssertedDate => self.asserted_date.as_ref(),
			_ => unimplemented!("Currently only Condition:asserted-date is implemented"),
		}
	}
}
