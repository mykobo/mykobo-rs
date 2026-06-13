use crate::read_file;
use mykobo_rs::anchor::models::{
    AnchorRpcResponse, AnchorRpcResponseResult, StellarTransaction as Transaction,
};

#[test]
fn test_transaction_response_extract() {
    let payload = r#"
    {
      "id": "febb7461-64b9-4cab-85cd-313c55c30906",
      "sep": "24",
      "kind": "deposit",
      "status": "pending_user_transfer_start",
      "amount_expected": {
        "amount": "20.00",
        "asset": "stellar:EURC:GDBDEI3NV72XSORX7DNYMGRRNXAXF62RPTGGVEXM2RLXIUIUU5DNZWWH"
      },
      "amount_in": {
        "amount": "20.00",
        "asset": "iso4217:EUR"
      },
      "amount_out": {
        "amount": "19.80",
        "asset": "stellar:EURC:GDBDEI3NV72XSORX7DNYMGRRNXAXF62RPTGGVEXM2RLXIUIUU5DNZWWH"
      },
      "fee_details": {
        "total": "0.20",
        "asset": "iso4217:EUR",
        "details": [
          {
            "name": "DEPOSIT",
            "description": "Deposit fee",
            "amount": "0.20"
          }
        ]
      },
      "started_at": "2025-06-29T16:57:19.726407Z",
      "updated_at": "2025-06-29T16:58:02.644560Z",
      "message": "Transaction ready for bank payment",
      "destination_account": "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6",
      "client_domain": "app.beansapp.com",
      "customers": {
        "sender": {
          "account": "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6"
        },
        "receiver": {
          "account": "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6"
        }
      },
      "creator": {
        "account": "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6"
      }
    }
    "#;

    let transaction: Transaction = serde_json::from_str(payload).unwrap();
    assert!(transaction
        .destination_account
        .is_some_and(|a| a == "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6"));
}
#[test]
fn test_stellar_transaction_deserialisation() {
    let content = read_file("tests/stubs/stellar_anchor_transaction_response.json");
    let transaction: AnchorRpcResponse = serde_json::from_str(content.as_str()).unwrap();
    assert!(matches!(
        transaction.result,
        AnchorRpcResponseResult::Transaction(_)
    ));
}
