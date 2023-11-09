#[cfg(test)]
#[cfg(feature = "lbf")]
mod json_roundtrip_tests {
    use lbr_prelude::json::{Error, Json};
    fn from_to_json<T>(val: &T) -> Result<T, Error>
    where
        T: Json + PartialEq,
    {
        T::from_json(val.to_json()?)
    }

    mod v1 {
        use super::from_to_json;
        use plutus_ledger_api::generators::correct::v1::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn test_asset_class(val in arb_asset_class()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_value(val in arb_value()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_plutus_data(val in arb_plutus_data()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_plutus_interval(val in arb_plutus_interval_posix_time()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_address(val in arb_address()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_transaction_input(val in arb_transaction_input()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_transaction_output(val in arb_transaction_output()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_tx_in_info(val in arb_tx_in_info()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_redeemeer(val in arb_redeemer()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }
    }
    mod v2 {
        use super::from_to_json;
        use plutus_ledger_api::generators::correct::v2::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn test_transaction_output(val in arb_transaction_output()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_tx_in_info(val in arb_tx_in_info()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }

        proptest! {
            #[test]
            fn test_output_datum(val in arb_output_datum()) {
                assert_eq!(val, from_to_json(&val)?);
            }
        }
    }
}
