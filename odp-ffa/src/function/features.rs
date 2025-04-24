use crate::{exec_simple, Error, ExecResult, Function, FunctionId, SmcParams};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Features {
    feature_id: u64,
    input_properties: u64,
}

pub struct FeaturesResult {
    pub interface_properties: u64,
}

impl Function for Features {
    const ID: FunctionId = FunctionId::Features;
    type ReturnType = FeaturesResult;

    fn exec(self) -> ExecResult<Self::ReturnType> {
        exec_simple(self, |result| {
            Ok(FeaturesResult {
                // TODO - spec says w2 & w3 are used, but we only use w2 (x2?)
                interface_properties: result.params.x2,
            })
        })
    }
}

impl TryInto<SmcParams> for Features {
    type Error = Error;

    fn try_into(self) -> Result<SmcParams, Self::Error> {
        Ok(SmcParams {
            x1: self.feature_id,
            x2: self.input_properties,
            ..Default::default()
        })
    }
}

impl TryFrom<SmcParams> for Features {
    type Error = Error;

    fn try_from(value: SmcParams) -> Result<Self, Self::Error> {
        Ok(Features {
            feature_id: value.x1,
            input_properties: value.x2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features_round_trip() {
        let original_features = Features {
            feature_id: 0x1234,
            input_properties: 0x5678,
        };

        let params: SmcParams = original_features.clone().try_into().unwrap();
        let new_features: Features = params.try_into().unwrap();

        assert_eq!(original_features.feature_id, new_features.feature_id);
        assert_eq!(original_features.input_properties, new_features.input_properties);
    }
}
