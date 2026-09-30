use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    str::FromStr,
};

use thiserror::Error;

use crate::{models::MODELS, sealed::Sealed, AnyModel, Model, ModelAddr, Models, ParseError};

/// Information about a model which is known at runtime.
///
/// Every model provides this information via [`Model::INFO`]. All models
/// enabled via Cargo features are listed in [`MODELS`]. Use
/// [`ModelInfo::by_id`] or [`str::parse`] to look up a model and
/// [`ModelInfo::parse`] to parse its data without knowing its type at
/// compile time.
pub struct ModelInfo {
    /// Model ID
    pub id: u16,
    /// Name of the model as defined by the SunSpec specification,
    /// e.g. `"inverter_three_phase"`.
    pub name: &'static str,
    /// Label of the model as defined by the SunSpec specification,
    /// e.g. `"Inverter (Three Phase)"`.
    pub label: &'static str,
    addr: fn(&Models) -> ModelAddr<AnyModel>,
    parse: fn(&[u16]) -> Result<AnyModel, ParseError<AnyModel>>,
}

impl ModelInfo {
    /// Create the model information for the given model type.
    pub const fn of<M: Model>() -> Self {
        Self {
            id: M::ID,
            name: M::NAME,
            label: M::LABEL,
            addr: addr_of::<M>,
            parse: parse_any::<M>,
        }
    }
    /// Returns the model with the given id or `None` if the model
    /// is unknown or not enabled via Cargo features.
    pub fn by_id(id: u16) -> Option<&'static Self> {
        MODELS
            .binary_search_by_key(&id, |info| info.id)
            .ok()
            .map(|index| MODELS[index])
    }
    /// Returns the address of this model in the given discovered
    /// models. An address of `0` indicates that the model was not
    /// discovered.
    pub fn addr(&self, models: &Models) -> ModelAddr<AnyModel> {
        (self.addr)(models)
    }
    /// Parse model data.
    #[allow(clippy::result_large_err)]
    pub fn parse(&self, data: &[u16]) -> Result<AnyModel, ParseError<AnyModel>> {
        (self.parse)(data)
    }
}

fn addr_of<M: Model>(models: &Models) -> ModelAddr<AnyModel> {
    M::addr(models).cast()
}

#[allow(clippy::result_large_err)]
fn parse_any<M: Model>(data: &[u16]) -> Result<AnyModel, ParseError<AnyModel>> {
    M::parse(data)
        .map(Into::into)
        .map_err(|e| e.map_model(Into::into))
}

impl Debug for ModelInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModelInfo")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("label", &self.label)
            .finish_non_exhaustive()
    }
}

impl PartialEq for ModelInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ModelInfo {}

impl Hash for ModelInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl FromStr for &'static ModelInfo {
    type Err = ModelNotFound;
    /// Look up a model by its id (`"103"`, `"m103"`, `"model103"`) or
    /// its name (`"inverter_three_phase"`). The comparison is
    /// case-insensitive.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let lower = s.to_ascii_lowercase();
        let digits = lower
            .strip_prefix("model")
            .or_else(|| lower.strip_prefix('m'))
            .unwrap_or(&lower);
        digits
            .parse()
            .ok()
            .and_then(ModelInfo::by_id)
            .or_else(|| {
                MODELS
                    .iter()
                    .copied()
                    .find(|info| info.name.eq_ignore_ascii_case(s))
            })
            .ok_or_else(|| ModelNotFound(s.to_owned()))
    }
}

/// Error returned when looking up a [`ModelInfo`] by a string fails.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
#[error("Model not found: {0:?}")]
pub struct ModelNotFound(pub String);

/// Object safe part of the [`Model`] trait. It is implemented by all
/// models and used by [`AnyModel::as_dyn`] to provide methods which
/// work with any model.
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait DynModel: Sealed + Debug {
    /// Returns information about this model.
    fn info(&self) -> &'static ModelInfo;
}

impl<M: Model> DynModel for M {
    fn info(&self) -> &'static ModelInfo {
        &M::INFO
    }
}

impl Models {
    /// Returns an iterator over all discovered models which are enabled
    /// via Cargo features, sorted by id.
    ///
    /// Each item contains the model information and its address which
    /// can be passed to
    /// [`AsyncDevice::read_any_model`](crate::client::AsyncDevice::read_any_model).
    pub fn iter(&self) -> impl Iterator<Item = (&'static ModelInfo, ModelAddr<AnyModel>)> + '_ {
        MODELS
            .iter()
            .map(|&info| (info, info.addr(self)))
            .filter(|(_, addr)| addr.addr != 0)
    }
}

impl AnyModel {
    /// Returns information about the contained model.
    pub fn info(&self) -> &'static ModelInfo {
        self.as_dyn().info()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn models_are_sorted() {
        assert!(MODELS.windows(2).all(|w| w[0].id < w[1].id));
    }

    #[test]
    fn model_lookup() {
        for &info in MODELS {
            assert_eq!(ModelInfo::by_id(info.id), Some(info));
            for s in [
                info.id.to_string(),
                format!("m{}", info.id),
                format!("model{}", info.id),
                format!("M{}", info.id),
                info.name.to_owned(),
                info.name.to_ascii_uppercase(),
            ] {
                assert_eq!(s.parse::<&ModelInfo>(), Ok(info), "{s:?}");
            }
        }
        assert_eq!(ModelInfo::by_id(0), None);
        assert!("m".parse::<&ModelInfo>().is_err());
        assert!("no_such_model".parse::<&ModelInfo>().is_err());
    }
}
