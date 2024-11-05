#[cfg(test)]
mod display_serialisation_tests {
    mod golden_v1 {
        use plutus_ledger_api::{
            goldens::v1::{
                sample_address, sample_asset_class, sample_currency_symbol,
                sample_transaction_input, sample_value,
            },
            v1::value::CurrencySymbol,
            v3::value::TokenName,
        };

        #[test]
        fn v1_currency_symbol_display_1() {
            goldie::assert!(format!("{}", sample_currency_symbol()))
        }

        #[test]
        fn v1_currency_symbol_display_2() {
            goldie::assert!(format!("{}", CurrencySymbol::Ada))
        }

        #[test]
        fn v1_currency_symbol_display_3() {
            goldie::assert!(format!("{:#}", CurrencySymbol::Ada))
        }

        #[test]
        fn v1_token_name_display_1() {
            goldie::assert!(format!(
                "{}",
                TokenName::from_bytes(vec![255, 244, 233, 222])
            ))
        }

        #[test]
        fn v1_token_name_display_2() {
            goldie::assert!(format!("{}", TokenName::from_string("TestToken")))
        }

        #[test]
        fn v1_token_name_display_3() {
            goldie::assert!(format!(
                "{:#}",
                TokenName::from_bytes(vec![255, 244, 233, 222])
            ))
        }

        #[test]
        fn v1_token_name_display_4() {
            goldie::assert!(format!("{:#}", TokenName::from_string("TestToken")))
        }

        #[test]
        fn v1_asset_class_display() {
            goldie::assert!(format!("{}", sample_asset_class()))
        }

        #[test]
        fn v1_value_display_1() {
            goldie::assert!(format!("{}", sample_value()))
        }

        #[test]
        fn v1_value_display_2() {
            goldie::assert!(format!("{:#}", sample_value()))
        }

        #[test]
        fn v1_address_display_1() {
            goldie::assert!(format!("{}", sample_address().with_extra_info(0)))
        }

        #[test]
        fn v1_address_display_2() {
            goldie::assert!(format!("{}", sample_address().with_extra_info(1)))
        }

        #[test]
        fn v1_transaction_input_display() {
            goldie::assert!(format!("{}", sample_transaction_input()))
        }
    }
}
