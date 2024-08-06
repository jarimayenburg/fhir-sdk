//! Structures parsing.

use std::collections::BTreeMap;

use fhir_model::{r4b, r5, stu3};

use crate::model::structures::{
	ChoiceField, CodeField, Field, ObjectField, ReferenceField, StandardField, Type,
};

impl From<stu3::resources::StructureDefinition> for Type {
	fn from(structure_definition: stu3::resources::StructureDefinition) -> Self {
		let structure_definition = structure_definition.0;
		let version = structure_definition.version.expect("StructureDefinition.version");
		assert_eq!(
			structure_definition.fhir_version.expect("StructureDefinition.fhirVersion").to_string(),
			version
		);
		let base = structure_definition.base_definition.map(|base| {
			base.split_once("http://hl7.org/fhir/StructureDefinition/")
				.expect("parsing StructureDefinition.baseDefinition")
				.1
				.to_owned()
		});

		Self {
			name: structure_definition.name,
			version,
			url: structure_definition.url,
			description: structure_definition.description.expect("StructureDefinition.description"),
			kind: structure_definition.kind.into(),
			r#abstract: structure_definition.r#abstract,
			base,
			status: structure_definition.status.into(),
			experimental: structure_definition
				.experimental
				.expect("StructureDefinition.experimental"),
			r#type: structure_definition.r#type,
			elements: ObjectField::from(
				structure_definition.snapshot.expect("StructureDefinition.snapshot"),
			),
		}
	}
}

impl From<r4b::resources::StructureDefinition> for Type {
	fn from(structure_definition: r4b::resources::StructureDefinition) -> Self {
		let structure_definition = structure_definition.0;
		let version = structure_definition.version.expect("StructureDefinition.version");
		assert_eq!(
			structure_definition.fhir_version.expect("StructureDefinition.fhirVersion").to_string(),
			version
		);
		let base = structure_definition.base_definition.map(|base| {
			base.split_once("http://hl7.org/fhir/StructureDefinition/")
				.expect("parsing StructureDefinition.baseDefinition")
				.1
				.to_owned()
		});

		Self {
			name: structure_definition.name,
			version,
			url: structure_definition.url,
			description: structure_definition.description.expect("StructureDefinition.description"),
			kind: structure_definition.kind.into(),
			r#abstract: structure_definition.r#abstract,
			base,
			status: structure_definition.status.into(),
			experimental: structure_definition
				.experimental
				.expect("StructureDefinition.experimental"),
			r#type: structure_definition.r#type,
			elements: ObjectField::from(
				structure_definition.snapshot.expect("StructureDefinition.snapshot"),
			),
		}
	}
}

impl From<r5::resources::StructureDefinition> for Type {
	fn from(structure_definition: r5::resources::StructureDefinition) -> Self {
		let structure_definition = structure_definition.0;
		let version = structure_definition.version.expect("StructureDefinition.version");
		assert_eq!(
			structure_definition.fhir_version.expect("StructureDefinition.fhirVersion").to_string(),
			version
		);
		let base = structure_definition.base_definition.map(|base| {
			base.split_once("http://hl7.org/fhir/StructureDefinition/")
				.expect("parsing StructureDefinition.baseDefinition")
				.1
				.to_owned()
		});

		Self {
			name: structure_definition.name,
			version,
			url: structure_definition.url,
			description: structure_definition.description.expect("StructureDefinition.description"),
			kind: structure_definition.kind.into(),
			r#abstract: structure_definition.r#abstract,
			base,
			status: structure_definition.status.into(),
			experimental: structure_definition
				.experimental
				.expect("StructureDefinition.experimental"),
			r#type: structure_definition.r#type,
			elements: ObjectField::from(
				structure_definition.snapshot.expect("StructureDefinition.snapshot"),
			),
		}
	}
}

impl From<stu3::types::ElementDefinition> for Field {
	fn from(element: stu3::types::ElementDefinition) -> Self {
		if element.path.ends_with("[x]") {
			Self::Choice(ChoiceField::from(element))
		} else if element.binding.is_some() {
			Self::Code(CodeField::from(element))
		} else if element.r#type.iter().flatten().any(|t| t.code == "Reference") {
			Self::Reference(ReferenceField::from(element))
		} else if element
			.extension
			.iter()
			.any(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			}) || element.content_reference.is_some()
		{
			Self::Object(ObjectField::from(element))
		} else {
			Self::Standard(StandardField::from(element))
		}
	}
}

impl From<r4b::types::ElementDefinition> for Field {
	fn from(element: r4b::types::ElementDefinition) -> Self {
		if element.path.ends_with("[x]") || element.r#type.len() > 1 {
			Self::Choice(ChoiceField::from(element))
		} else if element.binding.is_some() {
			Self::Code(CodeField::from(element))
		} else if element.r#type.iter().flatten().any(|t| t.code == "Reference") {
			Self::Reference(ReferenceField::from(element))
		} else if element
			.extension
			.iter()
			.any(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			}) || element.content_reference.is_some()
		{
			Self::Object(ObjectField::from(element))
		} else {
			Self::Standard(StandardField::from(element))
		}
	}
}

impl From<r5::types::ElementDefinition> for Field {
	fn from(element: r5::types::ElementDefinition) -> Self {
		if element.path.ends_with("[x]") || element.r#type.len() > 1 {
			Self::Choice(ChoiceField::from(element))
		} else if element.binding.is_some() {
			Self::Code(CodeField::from(element))
		} else if element.r#type.iter().flatten().any(|t| t.code == "Reference") {
			Self::Reference(ReferenceField::from(element))
		} else if element
			.extension
			.iter()
			.any(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			}) || element.content_reference.is_some()
		{
			Self::Object(ObjectField::from(element))
		} else {
			Self::Standard(StandardField::from(element))
		}
	}
}

impl From<stu3::resources::StructureDefinitionSnapshot> for ObjectField {
	fn from(snapshot: stu3::resources::StructureDefinitionSnapshot) -> Self {
		let mut elements = snapshot.element.into_iter().flatten();
		let first = elements.next().expect("Found no ElementDefinition").0;
		let name = first.path;
		assert!(!name.contains('.'));
		let min = first.min.expect("ElementDefinition.min");
		let max = first.max.expect("ElementDefinition.max");
		let r#type = first.r#type.iter().flatten().next().map(stu3_type_to_string);

		let mut object = Self {
			name,
			short: first.short.expect("ElementDefinition.short"),
			definition: first.definition.expect("ElementDefinition.definition"),
			comment: first.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: false,
			r#type,
			type_name: None,
			content_reference: None,
			is_modifier: first.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: first.is_summary.unwrap_or(false),
			fields: Vec::new(),
			field_map: BTreeMap::new(),
		};

		for element in elements {
			let path = element.path.clone();
			let Some((top_name, remaining)) = path.split_once('.') else {
				panic!("Multiple top-level fields defined?");
			};
			assert_eq!(top_name, &object.name);

			let field = Field::from(element);
			object.add_field(remaining, field);
		}

		object
	}
}

impl From<r4b::resources::StructureDefinitionSnapshot> for ObjectField {
	fn from(snapshot: r4b::resources::StructureDefinitionSnapshot) -> Self {
		let mut elements = snapshot.element.into_iter().flatten();
		let first = elements.next().expect("Found no ElementDefinition").0;
		let name = first.path;
		assert!(!name.contains('.'));

		let min = first.min.expect("ElementDefinition.min");
		let max = first.max.expect("ElementDefinition.max");
		let r#type = first.r#type.iter().flatten().next().map(r4b_type_to_string);

		let mut object = Self {
			name,
			short: first.short.expect("ElementDefinition.short"),
			definition: first.definition.expect("ElementDefinition.definition"),
			comment: first.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: false,
			r#type,
			type_name: None,
			content_reference: None,
			is_modifier: first.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: first.is_summary.unwrap_or(false),
			fields: Vec::new(),
			field_map: BTreeMap::new(),
		};

		for element in elements {
			let path = element.path.clone();
			let Some((top_name, remaining)) = path.split_once('.') else {
				panic!("Multiple top-level fields defined?");
			};
			assert_eq!(top_name, &object.name);

			let field = Field::from(element);
			object.add_field(remaining, field);
		}

		object
	}
}

impl From<r5::resources::StructureDefinitionSnapshot> for ObjectField {
	fn from(snapshot: r5::resources::StructureDefinitionSnapshot) -> Self {
		let mut elements = snapshot.element.into_iter().flatten();
		let first = elements.next().expect("Found no ElementDefinition").0;
		let name = first.path;
		assert!(!name.contains('.'));
		let min = first.min.expect("ElementDefinition.min");
		let max = first.max.expect("ElementDefinition.max");
		let r#type = first.r#type.iter().flatten().next().map(r5_type_to_string);

		let mut object = Self {
			name,
			short: first.short.expect("ElementDefinition.short"),
			definition: first.definition.expect("ElementDefinition.definition"),
			comment: first.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: false,
			r#type,
			type_name: None,
			content_reference: None,
			is_modifier: first.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: first.is_summary.unwrap_or(false),
			fields: Vec::new(),
			field_map: BTreeMap::new(),
		};

		for element in elements {
			let path = element.path.clone();
			let Some((top_name, remaining)) = path.split_once('.') else {
				panic!("Multiple top-level fields defined?");
			};
			assert_eq!(top_name, &object.name);

			let field = Field::from(element);
			object.add_field(remaining, field);
		}

		object
	}
}

impl From<stu3::types::ElementDefinition> for ObjectField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let is_base_field = element.base.map_or(false, |base| base.path != element.path);
		let r#type = element.r#type.iter().flatten().next().map(stu3_type_to_string);
		let type_name = element
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				stu3::types::ExtensionValue::String(s) => s,
				_ => panic!("Wrong value type in ElementDefinition.extension"),
			});

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field,
			r#type,
			type_name,
			content_reference: element.content_reference,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
			fields: Vec::new(),
			field_map: BTreeMap::new(),
		}
	}
}

impl From<r4b::types::ElementDefinition> for ObjectField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let type_name = element
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				r4b::types::ExtensionValue::String(s) => s,
				_ => panic!("Wrong value type in ElementDefinition.extension"),
			});

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: element.base.map_or(false, |base| base.path != element.path),
			r#type: element.r#type.iter().flatten().next().map(r4b_type_to_string),
			type_name,
			content_reference: element.content_reference,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
			fields: Vec::new(),
			field_map: BTreeMap::new(),
		}
	}
}

impl From<r5::types::ElementDefinition> for ObjectField {
	fn from(element: r5::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let type_name = element
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url == "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				r5::types::ExtensionValue::String(s) => s,
				_ => panic!("Wrong value type in ElementDefinition.extension"),
			});

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: element.base.map_or(false, |base| base.path != element.path),
			r#type: element.r#type.iter().flatten().next().map(r5_type_to_string),
			type_name,
			content_reference: element.content_reference,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
			fields: Vec::new(),
			field_map: BTreeMap::new(),
		}
	}
}

impl From<stu3::types::ElementDefinition> for StandardField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
		let element = element.0;
		if element.r#type.is_empty() {
			panic!("Element without type: {element:#?}");
		}

		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let is_base_field = element.base.map_or(false, |base| base.path != element.path)
			|| element
				.r#type
				.first()
				.and_then(Option::as_ref)
				.map_or(false, |ty| &ty.code == "http://hl7.org/fhirpath/System.String");
		let r#type = element
			.r#type
			.iter()
			.flatten()
			.next()
			.map(stu3_type_to_string)
			.expect("ElementDefinition.type");

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field,
			r#type,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

impl From<r4b::types::ElementDefinition> for StandardField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
		let element = element.0;
		if element.r#type.is_empty() {
			panic!("Element without type: {element:#?}");
		}

		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let is_base_field = element.base.map_or(false, |base| base.path != element.path)
			|| element
				.r#type
				.first()
				.and_then(Option::as_ref)
				.map_or(false, |ty| &ty.code == "http://hl7.org/fhirpath/System.String");
		let r#type = element
			.r#type
			.iter()
			.flatten()
			.next()
			.map(r4b_type_to_string)
			.expect("ElementDefinition.type");

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field,
			r#type,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

impl From<r5::types::ElementDefinition> for StandardField {
	fn from(element: r5::types::ElementDefinition) -> Self {
		let element = element.0;
		if element.r#type.is_empty() {
			panic!("Element without type: {element:#?}");
		}

		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let is_base_field = element.base.map_or(false, |base| base.path != element.path)
			|| element
				.r#type
				.first()
				.and_then(Option::as_ref)
				.map_or(false, |ty| &ty.code == "http://hl7.org/fhirpath/System.String");
		let r#type = element
			.r#type
			.iter()
			.flatten()
			.next()
			.map(r5_type_to_string)
			.expect("ElementDefinition.type");
		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field,
			r#type,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

impl From<stu3::types::ElementDefinition> for CodeField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let is_array = &max != "1";
		let is_base_field = element.base.map_or(false, |base| base.path != element.path);
		let r#type = element
			.r#type
			.iter()
			.flatten()
			.next()
			.map(stu3_type_to_string)
			.expect("ElementDefinition.type");
		let binding = element.binding.expect("ElementDefinition.binding");
		let code_name = binding
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url
					== "http://hl7.org/fhir/StructureDefinition/elementdefinition-bindingName"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				stu3::types::ExtensionValue::String(s) => s,
				_ => panic!("unexpected extension value type"),
			});
		// Remove version string at the end (|5.0.0).
		let code_url = binding
			.value_set
			.map(|code_url| match code_url {
				stu3::types::ElementDefinitionBindingValueSet::Uri(u) => u,
				stu3::types::ElementDefinitionBindingValueSet::Reference(r) => {
					r.reference.clone().expect("ElementDefinition.valueSetReference.reference")
				}
			})
			.map(|code_url| {
				code_url.split_once('|').map_or(code_url.as_str(), |(start, _end)| start).to_owned()
			});
		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array,
			is_base_field,
			r#type,
			code_name,
			code_url,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

impl From<r4b::types::ElementDefinition> for CodeField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let r#type = element
			.r#type
			.iter()
			.flatten()
			.next()
			.map(r4b_type_to_string)
			.expect("ElementDefinition.type");
		let binding = element.binding.expect("ElementDefinition.binding");
		let code_name = binding
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url
					== "http://hl7.org/fhir/StructureDefinition/elementdefinition-bindingName"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				r4b::types::ExtensionValue::String(s) => s,
				_ => panic!("unexpected extension value type"),
			});
		// Remove version string at the end (|5.0.0).
		let code_url = binding.value_set.map(|code_url| {
			code_url.split_once('|').map_or(code_url.as_str(), |(start, _end)| start).to_owned()
		});
		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: element.base.map_or(false, |base| base.path != element.path),
			r#type,
			code_name,
			code_url,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

impl From<r5::types::ElementDefinition> for CodeField {
	fn from(element: r5::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let r#type = element
			.r#type
			.iter()
			.flatten()
			.next()
			.map(r5_type_to_string)
			.expect("ElementDefinition.type");
		let binding = element.binding.expect("ElementDefinition.binding");
		let code_name = binding
			.extension
			.into_iter()
			.find(|extension| {
				&extension.url
					== "http://hl7.org/fhir/StructureDefinition/elementdefinition-bindingName"
			})
			.and_then(|extension| extension.0.value)
			.map(|value| match value {
				r5::types::ExtensionValue::String(s) => s,
				_ => panic!("unexpected extension value type"),
			});
		// Remove version string at the end (|5.0.0).
		let code_url = binding.value_set.map(|code_url| {
			code_url.split_once('|').map_or(code_url.as_str(), |(start, _end)| start).to_owned()
		});

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: element.base.map_or(false, |base| base.path != element.path),
			r#type,
			code_name,
			code_url,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

impl From<stu3::types::ElementDefinition> for ChoiceField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let mut types: Vec<_> = element.r#type.iter().flatten().map(stu3_type_to_string).collect();
		types.dedup();

		let mut reference_target_resource_types = vec![];

		if element.r#type.iter().flatten().any(|t| t.code == "Reference") {
			let target_resource_types_iter =
				element.r#type.into_iter().flatten().flat_map(|t| t.target_profile).flat_map(|t| {
					t.strip_prefix("http://hl7.org/fhir/StructureDefinition/")
						.map(|t| t.to_string())
				});

			reference_target_resource_types.extend(target_resource_types_iter);

			if reference_target_resource_types.is_empty() {
				reference_target_resource_types.push("Resource".to_string());
			}
		}

		reference_target_resource_types.sort();
		reference_target_resource_types.dedup();

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: element.base.map_or(false, |base| base.path != element.path),
			types,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
			reference_target_resource_types,
		}
	}
}

impl From<r4b::types::ElementDefinition> for ChoiceField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let types = element.r#type.iter().flatten().map(r4b_type_to_string).collect();
		let reference_target_resource_types =
			match element.r#type.iter().flatten().find(|t| t.code == "Reference") {
				Some(r#type) => {
					let mut reference_target_resource_types: Vec<_> = r#type
						.target_profile
						.iter()
						.flatten()
						.flat_map(|t| t.strip_prefix("http://hl7.org/fhir/StructureDefinition/"))
						.map(|t| t.to_string())
						.collect();

					if reference_target_resource_types.is_empty() {
						reference_target_resource_types.push("Resource".to_string());
					}

					reference_target_resource_types.sort();
					reference_target_resource_types.dedup();

					reference_target_resource_types
				}
				None => vec![],
			};

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: element.base.map_or(false, |base| base.path != element.path),
			types,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
			reference_target_resource_types,
		}
	}
}

impl From<r5::types::ElementDefinition> for ChoiceField {
	fn from(element: r5::types::ElementDefinition) -> Self {
		let element = element.0;
		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let types = element.r#type.iter().flatten().map(r5_type_to_string).collect();
		let reference_target_resource_types =
			match element.r#type.iter().flatten().find(|t| t.code == "Reference") {
				Some(r#type) => {
					let mut target_types: Vec<_> = r#type
						.target_profile
						.iter()
						.flatten()
						.flat_map(|t| t.strip_prefix("http://hl7.org/fhir/StructureDefinition/"))
						.map(|t| t.to_string())
						.collect();

					if target_types.is_empty() {
						target_types.push("Resource".to_string());
					}

					target_types.sort();
					target_types.dedup();

					target_types
				}
				None => vec![],
			};

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: element.base.map_or(false, |base| base.path != element.path),
			types,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
			reference_target_resource_types,
		}
	}
}

impl From<stu3::types::ElementDefinition> for ReferenceField {
	fn from(element: stu3::types::ElementDefinition) -> Self {
		let element = element.0;
		if !element.r#type.iter().flatten().any(|t| t.code == "Reference") {
			panic!("Element not a Reference: {element:#?}");
		}

		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let mut target_resource_types: Vec<_> = element
			.r#type
			.into_iter()
			.flatten()
			.flat_map(|t| t.target_profile)
			.flat_map(|t| {
				t.strip_prefix("http://hl7.org/fhir/StructureDefinition/").map(|t| t.to_string())
			})
			.collect();

		if target_resource_types.is_empty() {
			target_resource_types.push("Resource".to_string());
		}

		target_resource_types.sort();
		target_resource_types.dedup();

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: false,
			target_resource_types,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

impl From<r4b::types::ElementDefinition> for ReferenceField {
	fn from(element: r4b::types::ElementDefinition) -> Self {
		let element = element.0;
		if !element.r#type.iter().flatten().any(|t| t.code == "Reference") {
			panic!("Element not a Reference: {element:#?}");
		}

		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let r#type = element.r#type.iter().flatten().find(|t| t.code == "Reference").unwrap();

		let mut target_resource_types: Vec<_> = r#type
			.target_profile
			.iter()
			.flatten()
			.flat_map(|t| t.strip_prefix("http://hl7.org/fhir/StructureDefinition/"))
			.map(|t| t.to_string())
			.collect();

		if target_resource_types.is_empty() {
			target_resource_types.push("Resource".to_string());
		}

		target_resource_types.sort();
		target_resource_types.dedup();

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: false,
			target_resource_types,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

impl From<r5::types::ElementDefinition> for ReferenceField {
	fn from(element: r5::types::ElementDefinition) -> Self {
		let element = element.0;
		if !element.r#type.iter().flatten().any(|t| t.code == "Reference") {
			panic!("Element not a Reference: {element:#?}");
		}

		let min = element.min.expect("ElementDefinition.min");
		let max = element.max.expect("ElementDefinition.max");
		let r#type = element.r#type.iter().flatten().find(|t| t.code == "Reference").unwrap();

		let mut target_resource_types: Vec<_> = r#type
			.target_profile
			.iter()
			.flatten()
			.flat_map(|t| t.strip_prefix("http://hl7.org/fhir/StructureDefinition/"))
			.map(|t| t.to_string())
			.collect();

		if target_resource_types.is_empty() {
			target_resource_types.push("Resource".to_string());
		}

		target_resource_types.sort();
		target_resource_types.dedup();

		Self {
			name: element.path.rsplit_once('.').map_or(element.path.clone(), |(_, n)| n.to_owned()),
			short: element.short.expect("ElementDefinition.short"),
			definition: element.definition.expect("ElementDefinition.definition"),
			comment: element.comment,
			optional: min == 0,
			is_array: &max != "1",
			is_base_field: false,
			target_resource_types,
			is_modifier: element.is_modifier.expect("ElementDefinition.isModifier"),
			is_summary: element.is_summary.unwrap_or(false),
		}
	}
}

fn stu3_type_to_string(r#type: &stu3::types::ElementDefinitionType) -> String {
	if !r#type.extension.is_empty() {
		for extension in r#type.extension.iter() {
			if &extension.url
				== "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
			{
				return extension
					.0
					.value
					.as_ref()
					.map(|v| match v {
						stu3::types::ExtensionValue::Uri(uri) => uri.to_string(),
						_ => panic!("ElementDefinition.type.extension.value is not URI"),
					})
					.expect("ElementDefinition.type.extension.value");
			}
		}
	}

	r#type.code.to_string()
}

/// Convert a type value to a proper string of the type name.
fn r4b_type_to_string(r#type: &r4b::types::ElementDefinitionType) -> String {
	if !r#type.extension.is_empty() {
		for extension in r#type.extension.iter() {
			if &extension.url
				== "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
			{
				return extension
					.0
					.value
					.as_ref()
					.map(|v| match v {
						r4b::types::ExtensionValue::Url(url) => url.to_string(),
						_ => panic!("ElementDefinition.type.extension.value is not URL"),
					})
					.expect("ElementDefinition.type.extension.value");
			}
		}
	}

	r#type.code.to_string()
}

/// Convert a type value to a proper string of the type name.
fn r5_type_to_string(r#type: &r5::types::ElementDefinitionType) -> String {
	if !r#type.extension.is_empty() {
		for extension in r#type.extension.iter() {
			if &extension.url
				== "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
			{
				return extension
					.0
					.value
					.as_ref()
					.map(|v| match v {
						r5::types::ExtensionValue::Url(url) => url.to_string(),
						_ => panic!("ElementDefinition.type.extension.value is not URL"),
					})
					.expect("ElementDefinition.type.extension.value");
			}
		}
	}

	r#type.code.to_string()
}

/// Parse a Bundle into Types.
pub fn parse_stu3(input: &str) -> Vec<Type> {
	let bundle: stu3::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing StructureDefinition Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			stu3::resources::Resource::StructureDefinition(structure_definition) => {
				Some(structure_definition)
			}
			_ => None,
		})
		.map(Type::from)
		.collect()
}

/// Parse a Bundle into Types.
pub fn parse_r4b(input: &str) -> Vec<Type> {
	let bundle: r4b::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing StructureDefinition Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			r4b::resources::Resource::StructureDefinition(structure_definition) => {
				Some(structure_definition)
			}
			_ => None,
		})
		.map(Type::from)
		.collect()
}

/// Parse a Bundle into Types.
pub fn parse_r5(input: &str) -> Vec<Type> {
	let bundle: r5::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing StructureDefinition Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			r5::resources::Resource::StructureDefinition(structure_definition) => {
				Some(structure_definition)
			}
			_ => None,
		})
		.map(Type::from)
		.collect()
}
