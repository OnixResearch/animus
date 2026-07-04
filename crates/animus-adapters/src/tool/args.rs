/// One borrowed tool-call argument before preparation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArgumentFieldDraft<'a> {
    /// Optional argument name, required by validation.
    pub name: Option<&'a str>,
    /// Optional argument value, required by validation.
    pub value: Option<&'a str>,
}

/// Pure compatibility rule from a legacy argument name to the current name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArgumentCompatibilityRule<'a> {
    /// Legacy argument name.
    pub legacy_name: &'a str,
    /// Current strict-contract argument name.
    pub current_name: &'a str,
}

/// Borrowed argument preparation inputs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArgumentPreparationRequest<'a> {
    /// Current public parameter schema identity.
    pub current_schema_identity: &'a str,
    /// Current allowed argument field names.
    pub allowed_field_names: &'a [&'a str],
    /// Compatibility rules applied before strict validation.
    pub compatibility_rules: &'a [ArgumentCompatibilityRule<'a>],
    /// Raw or resumed argument fields.
    pub fields: &'a [ArgumentFieldDraft<'a>],
}

/// Prepared argument field after compatibility mapping and validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArgumentField {
    name: super::Text,
    value: super::Text,
}

impl ArgumentField {
    /// Returns the current argument name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the validated argument value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

/// Prepared current-shape arguments ready for schema validation by the host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedArguments {
    schema_identity: super::Text,
    fields: super::Collection<ArgumentField>,
}

impl PreparedArguments {
    /// Prepares legacy/resumed arguments and validates the current field set.
    ///
    /// # Errors
    ///
    /// Returns [`super::ContractError`] when the schema identity or fields are
    /// malformed, compatibility creates duplicates, or unknown fields remain
    /// after preparation.
    pub fn prepare(request: &ArgumentPreparationRequest<'_>) -> Result<Self, super::ContractError> {
        let schema_identity =
            super::required_text_value(super::SCHEMA_FIELD, request.current_schema_identity)?;
        let allowed =
            super::normalized_label_set(super::ARGUMENT_NAME_FIELD, request.allowed_field_names)?;
        let rules = normalized_rules(request.compatibility_rules)?;
        let fields = prepared_fields(request.fields, &allowed, &rules)?;
        Ok(Self {
            schema_identity,
            fields,
        })
    }

    /// Returns the schema identity used for strict validation.
    #[must_use]
    pub fn schema_identity(&self) -> &str {
        self.schema_identity.as_str()
    }

    /// Returns prepared argument fields.
    #[must_use]
    pub fn fields(&self) -> &[ArgumentField] {
        self.fields.as_slice()
    }
}

fn prepared_fields(
    drafts: &[ArgumentFieldDraft<'_>],
    allowed: &[super::Text],
    rules: &[(super::Text, super::Text)],
) -> Result<super::Collection<ArgumentField>, super::ContractError> {
    super::validate_item_count(super::ARGUMENT_NAME_FIELD, drafts.len())?;
    let mut fields = super::Collection::with_capacity(drafts.len());
    for draft in drafts {
        let name = super::required_text(super::ARGUMENT_NAME_FIELD, draft.name)?;
        let current_name = mapped_argument_name(&name, rules);
        validate_known_unique_field(&fields, allowed, &current_name)?;
        let value = super::required_text(super::ARGUMENT_VALUE_FIELD, draft.value)?;
        fields.push(ArgumentField {
            name: current_name,
            value,
        });
    }
    Ok(fields)
}

fn validate_known_unique_field(
    fields: &[ArgumentField],
    allowed: &[super::Text],
    current_name: &str,
) -> Result<(), super::ContractError> {
    if !contains_text(allowed, current_name) {
        return Err(super::ContractError::UnknownArgument {
            name: super::Text::from(current_name),
        });
    }
    if fields
        .iter()
        .any(|existing| existing.name() == current_name)
    {
        return Err(super::ContractError::DuplicateArgument {
            name: super::Text::from(current_name),
        });
    }
    Ok(())
}

fn normalized_rules(
    rules: &[ArgumentCompatibilityRule<'_>],
) -> Result<super::Collection<(super::Text, super::Text)>, super::ContractError> {
    super::validate_item_count(super::ARGUMENT_NAME_FIELD, rules.len())?;
    let mut normalized = super::Collection::with_capacity(rules.len());
    for rule in rules {
        normalized.push((
            super::required_text_value(super::ARGUMENT_NAME_FIELD, rule.legacy_name)?,
            super::required_text_value(super::ARGUMENT_NAME_FIELD, rule.current_name)?,
        ));
    }
    Ok(normalized)
}

fn mapped_argument_name(name: &str, rules: &[(super::Text, super::Text)]) -> super::Text {
    for (legacy_name, current_name) in rules {
        if legacy_name == name {
            return current_name.clone();
        }
    }
    super::Text::from(name)
}

fn contains_text(values: &[super::Text], needle: &str) -> bool {
    values.iter().any(|value| value.as_str() == needle)
}
