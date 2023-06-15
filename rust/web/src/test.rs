use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn test_user_summary() {
    let rocket = rocket::build().mount("/", routes![user_summary]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/users/summary/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("transactions"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_users() {
    let rocket = rocket::build().mount("/", routes![get_users]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/users").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_name"));
    assert!(body.contains("password"));
    assert!(body.contains("email"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_user() {
    let rocket = rocket::build().mount("/", routes![get_user]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_name"));
    assert!(body.contains("password"));
    assert!(body.contains("email"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_update_user() {
    let rocket = rocket::build().mount("/", routes![update_user]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_user = NewUser {
        user_name: "new_username".to_string(),
        password: "new_password".to_string(),
        email: "new_email@example.com".to_string(),
        created_on: None,
        modified_on: None,
    };

    let response = client
        .put("/users/1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_user).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_name"));
    assert!(body.contains("password"));
    assert!(body.contains("email"));
    // Add more specific assertions for the expected response body here
}

use rocket::http::{ContentType, HeaderMap, HeaderValue, Status};
use rocket::local::blocking::Client;

#[test]
fn test_delete_user() {
    let rocket = rocket::build().mount("/", routes![delete_user]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.delete("/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_name"));
    assert!(body.contains("password"));
    assert!(body.contains("email"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_register_user() {
    let rocket = rocket::build().mount("/", routes![register_user]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_user = NewUser {
        user_name: "test_user".to_string(),
        password: "test_password".to_string(),
        email: "test_email@example.com".to_string(),
        created_on: None,
        modified_on: None,
    };

    let response = client
        .post("/users/register")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_user).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_name"));
    assert!(body.contains("password"));
    assert!(body.contains("email"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_create_user() {
    let rocket = rocket::build().mount("/", routes![create_user]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_user = NewUser {
        user_name: "test_user".to_string(),
        password: "test_password".to_string(),
        email: "test_email@example.com".to_string(),
        created_on: None,
        modified_on: None,
    };

    let response = client
        .post("/users")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_user).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_name"));
    assert!(body.contains("password"));
    assert!(body.contains("email"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_fetch_symbol_price() {
    let rocket = rocket::build().mount("/", routes![fetch_symbol_price]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/fetch_symbol_price/BTCUSDT").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("symbol"));
    assert!(body.contains("price"));
    // Add more specific assertions for the expected response body here
}


#[test]
fn test_login() {
    let rocket = rocket::build().mount("/", routes![login]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let login_data = LoginData {
        user_name: "test_user".to_string(),
        password: "test_password".to_string(),
    };

    let response = client
        .post("/users/login")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&login_data).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_create_wallet() {
    let rocket = rocket::build().mount("/", routes![create_wallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_wallet = NewWallet {
        user_id: 1,
        cryptocurrency_id: 1,
        balance: 100.0,
        created_on: None,
        modified_on: None,
    };

    let response = client
        .post("/wallet")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_wallet).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_wallets() {
    let rocket = rocket::build().mount("/", routes![get_wallets]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/wallet").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_wallet() {
    let rocket = rocket::build().mount("/", routes![get_wallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/wallet/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_update_wallet() {
    let rocket = rocket::build().mount("/", routes![update_wallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_wallet = NewWallet {
        user_id: 1,
        cryptocurrency_id: 1,
        balance: 200.0,
        created_on: None,
        modified_on: None,
    };

    let response = client
        .put("/wallet/1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_wallet).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_delete_wallet() {
    let rocket = rocket::build().mount("/", routes![delete_wallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.delete("/wallet/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}


#[test]
fn test_get_cryptos() {
    let rocket = rocket::build().mount("/", routes![get_cryptos]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/crypto").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cname"));
    assert!(body.contains("symbol"));
    assert!(body.contains("price"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_crypto() {
    let rocket = rocket::build().mount("/", routes![get_crypto]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/crypto/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cname"));
    assert!(body.contains("symbol"));
    assert!(body.contains("price"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_create_crypto() {
    let rocket = rocket::build().mount("/", routes![create_crypto]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_crypto = NewCrypto {
        cname: "Bitcoin".to_string(),
        symbol: "BTC".to_string(),
        price: 50000.0,
        created_on: None,
        modified_on: None,
    };

    let response = client
        .post("/crypto")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_crypto).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cname"));
    assert!(body.contains("symbol"));
    assert!(body.contains("price"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_update_crypto() {
    let rocket = rocket::build().mount("/", routes![update_crypto]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_crypto = NewCrypto {
        cname: "Ethereum".to_string(),
        symbol: "ETH".to_string(),
        price: 3000.0,
        created_on: None,
        modified_on: None,
    };

    let response = client
        .put("/crypto/1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_crypto).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cname"));
    assert!(body.contains("symbol"));
    assert!(body.contains("price"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_delete_crypto() {
    let rocket = rocket::build().mount("/", routes![delete_crypto]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.delete("/crypto/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("cname"));
    assert!(body.contains("symbol"));
    assert!(body.contains("price"));
    // Add more specific assertions for the expected response body here
}


#[test]
fn test_create_rwallet() {
    let rocket = rocket::build().mount("/", routes![create_rwallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_rwallet = NewRealMoneyWallet {
        user_id: 1,
        currency: "USD".to_string(),
        balance: 100.0,
        created_on: None,
        modified_on: None,
    };

    let response = client
        .post("/realmoney")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_rwallet).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_id"));
    assert!(body.contains("currency"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_rwallets() {
    let rocket = rocket::build().mount("/", routes![get_rwallets]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/realmoney").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_id"));
    assert!(body.contains("currency"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_rwallet() {
    let rocket = rocket::build().mount("/", routes![get_rwallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/realmoney/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_id"));
    assert!(body.contains("currency"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_update_rwallet() {
    let rocket = rocket::build().mount("/", routes![update_rwallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_rwallet = NewRealMoneyWallet {
        user_id: 1,
        currency: "EUR".to_string(),
        balance: 200.0,
        created_on: None,
        modified_on: None,
    };

    let response = client
        .put("/realmoney/1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_rwallet).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_id"));
    assert!(body.contains("currency"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_delete_rwallet() {
    let rocket = rocket::build().mount("/", routes![delete_rwallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.delete("/realmoney/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_id"));
    assert!(body.contains("currency"));
    assert!(body.contains("balance"));
    // Add more specific assertions for the expected response body here
}


#[test]
fn test_create_trans() {
    let rocket = rocket::build().mount("/", routes![create_trans]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_trans = NewTransaction {
        user_id: 1,
        wallet_id: 1,
        rwallet_id: 1,
        cryptocurrency_id: 1,
        ttype: "deposit".to_string(),
        amount: 100.0,
        created_on: None,
        modified_on: None,
        payment_method: "stripe".to_string(),
        payment_amount: 100.0,
        payment_status: "completed".to_string(),
    };

    let response = client
        .post("/transactions")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_trans).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_id"));
    assert!(body.contains("wallet_id"));
    assert!(body.contains("rwallet_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("ttype"));
    assert!(body.contains("amount"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("payment_method"));
    assert!(body.contains("payment_amount"));
    assert!(body.contains("payment_status"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_withdraw_from_wallet() {
    let rocket = rocket::build().mount("/", routes![withdraw_from_wallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let withdrawal = NewTransaction {
        user_id: 1,
        wallet_id: 1,
        rwallet_id: 1,
        cryptocurrency_id: 1,
        ttype: "withdraw".to_string(),
        amount: 50.0,
        created_on: None,
        modified_on: None,
        payment_method: "realmoneywallet".to_string(),
        payment_amount: 50.0,
        payment_status: "completed".to_string(),
    };

    let response = client
        .post("/transactions/withdraw")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&withdrawal).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_id"));
    assert!(body.contains("wallet_id"));
    assert!(body.contains("rwallet_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("ttype"));
    assert!(body.contains("amount"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("payment_method"));
    assert!(body.contains("payment_amount"));
    assert!(body.contains("payment_status"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_desposit_into_wallet() {
    let rocket = rocket::build().mount("/", routes![desposit_into_wallet]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let deposit = NewTransaction {
        user_id: 1,
        wallet_id: 1,
        rwallet_id: 1,
        cryptocurrency_id: 1,
        ttype: "deposit".to_string(),
        amount: 100.0,
        created_on: None,
        modified_on: None,
        payment_method: "stripe".to_string(),
        payment_amount: 100.0,
        payment_status: "completed".to_string(),
    };

    let response = client
        .post("/transactions/deposit")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&deposit).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("user_id"));
    assert!(body.contains("wallet_id"));
    assert!(body.contains("rwallet_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("ttype"));
    assert!(body.contains("amount"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("payment_method"));
    assert!(body.contains("payment_amount"));
    assert!(body.contains("payment_status"));
    // Add more specific assertions for the expected response body here
}

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn test_get_transs() {
    let rocket = rocket::build().mount("/", routes![get_transs]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/transactions").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("wallet_id"));
    assert!(body.contains("rwallet_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("ttype"));
    assert!(body.contains("amount"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("payment_method"));
    assert!(body.contains("payment_amount"));
    assert!(body.contains("payment_status"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_trans() {
    let rocket = rocket::build().mount("/", routes![get_trans]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/transactions/1").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("wallet_id"));
    assert!(body.contains("rwallet_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("ttype"));
    assert!(body.contains("amount"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("payment_method"));
    assert!(body.contains("payment_amount"));
    assert!(body.contains("payment_status"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_update_trans() {
    let rocket = rocket::build().mount("/", routes![update_trans]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_trans = NewTransaction {
        user_id: 1,
        wallet_id: 1,
        rwallet_id: 1,
        cryptocurrency_id: 1,
        ttype: "deposit".to_string(),
        amount: 100.0,
        created_on: None,
        modified_on: None,
        payment_method: "stripe".to_string(),
        payment_amount: 100.0,
        payment_status: "completed".to_string(),
    };

    let response = client
        .put("/transactions/1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_trans).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("wallet_id"));
    assert!(body.contains("rwallet_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("ttype"));
    assert!(body.contains("amount"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("payment_method"));
    assert!(body.contains("payment_amount"));
    assert!(body.contains("payment_status"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_delete_trans() {
    let rocket = rocket::build().mount("/", routes![delete_trans]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.delete("/transactions/1").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("wallet_id"));
    assert!(body.contains("rwallet_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("ttype"));
    assert!(body.contains("amount"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("payment_method"));
    assert!(body.contains("payment_amount"));
    assert!(body.contains("payment_status"));
    // Add more specific assertions for the expected response body here
}


#[test]
fn test_create_order() {
    let rocket = rocket::build().mount("/", routes![create_order]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_order = NewOrder {
        user_id: 1,
        cryptocurrency_id: 1,
        amount: 10.0,
        price: 100.0,
        otype: "buy".to_string(),
        created_on: None,
        modified_on: None,
        ostatus: "open".to_string(),
        market_true: true,
    };

    let response = client
        .post("/orders")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_order).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("otype"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("ostatus"));
    assert!(body.contains("market_true"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_orders() {
    let rocket = rocket::build().mount("/", routes![get_orders]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/orders").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("otype"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("ostatus"));
    assert!(body.contains("market_true"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_order_history() {
    let rocket = rocket::build().mount("/", routes![get_order_history]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/orders/history/1").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("otype"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("ostatus"));
    assert!(body.contains("market_true"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_current_orders() {
    let rocket = rocket::build().mount("/", routes![get_current_orders]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/orders/current/1").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("otype"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("ostatus"));
    assert!(body.contains("market_true"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_order() {
    let rocket = rocket::build().mount("/", routes![get_order]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/orders/1").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("otype"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("ostatus"));
    assert!(body.contains("market_true"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_update_order() {
    let rocket = rocket::build().mount("/", routes![update_order]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_order = NewOrder {
        user_id: 1,
        cryptocurrency_id: 1,
        amount: 10.0,
        price: 100.0,
        otype: "buy".to_string(),
        created_on: None,
        modified_on: None,
        ostatus: "open".to_string(),
        market_true: true,
    };

    let response = client
        .put("/orders/1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_order).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("otype"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("ostatus"));
    assert!(body.contains("market_true"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_delete_order() {
    let rocket = rocket::build().mount("/", routes![delete_order]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.delete("/orders/1").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("user_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("otype"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    assert!(body.contains("ostatus"));
    assert!(body.contains("market_true"));
    // Add more specific assertions for the expected response body here
}


#[test]
fn test_create_trade() {
    let rocket = rocket::build().mount("/", routes![create_trade]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_trade = NewTrade {
        buyer_id: 1,
        seller_id: 2,
        cryptocurrency_id: 1,
        amount: 10.0,
        price: 100.0,
        created_on: None,
        modified_on: None,
    };

    let response = client
        .post("/trade")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_trade).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("buyer_id"));
    assert!(body.contains("seller_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_trades() {
    let rocket = rocket::build().mount("/", routes![get_trades]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/trade").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("buyer_id"));
    assert!(body.contains("seller_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_get_trade() {
    let rocket = rocket::build().mount("/", routes![get_trade]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/trade/1").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("buyer_id"));
    assert!(body.contains("seller_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_update_trade() {
    let rocket = rocket::build().mount("/", routes![update_trade]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_trade = NewTrade {
        buyer_id: 1,
        seller_id: 2,
        cryptocurrency_id: 1,
        amount: 10.0,
        price: 100.0,
        created_on: None,
        modified_on: None,
    };

    let response = client
        .put("/trade/1")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&new_trade).expect("valid request body"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("buyer_id"));
    assert!(body.contains("seller_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    // Add more specific assertions for the expected response body here
}

#[test]
fn test_delete_trade() {
    let rocket = rocket::build().mount("/", routes![delete_trade]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.delete("/trade/1").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert!(body.contains("id"));
    assert!(body.contains("buyer_id"));
    assert!(body.contains("seller_id"));
    assert!(body.contains("cryptocurrency_id"));
    assert!(body.contains("amount"));
    assert!(body.contains("price"));
    assert!(body.contains("created_on"));
    assert!(body.contains("modified_on"));
    // Add more specific assertions for the expected response body here
}

use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_matching_orders_no_matching_orders() {
    let rocket = rocket::build().mount("/", routes![matching_orders]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/matching_orders").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert_eq!(body, "No Orders matched currently, please wait or update your Order.");
}

#[test]
fn test_matching_orders_with_matching_orders() {
    // Set up the necessary data in the database to have matching orders

    let rocket = rocket::build().mount("/", routes![matching_orders]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/matching_orders").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert_eq!(body, "Orders matched and processed successfully.");
}

#[test]
fn test_matching_orders_with_multiple_matching_orders() {
    // Set up the necessary buy orders and sell orders in the database
    // Ensure there are multiple matching orders

    let rocket = rocket::build().mount("/", routes![matching_orders]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // Create 5 buy orders
    let buy_order1 = NewOrder {
        user_id: 1,
        cryptocurrency_id: 1,
        amount: 10.0,
        price: 100.0,
        otype: "buy".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    let buy_order2 = NewOrder {
        user_id: 2,
        cryptocurrency_id: 1,
        amount: 5.0,
        price: 90.0,
        otype: "buy".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    let buy_order3 = NewOrder {
        user_id: 3,
        cryptocurrency_id: 1,
        amount: 7.0,
        price: 95.0,
        otype: "buy".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    let buy_order4 = NewOrder {
        user_id: 4,
        cryptocurrency_id: 1,
        amount: 12.0,
        price: 85.0,
        otype: "buy".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    let buy_order5 = NewOrder {
        user_id: 5,
        cryptocurrency_id: 1,
        amount: 8.0,
        price: 105.0,
        otype: "buy".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    // Create 5 sell orders
    let sell_order1 = NewOrder {
        user_id: 6,
        cryptocurrency_id: 1,
        amount: 8.0,
        price: 80.0,
        otype: "sell".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    let sell_order2 = NewOrder {
        user_id: 7,
        cryptocurrency_id: 1,
        amount: 10.0,
        price: 95.0,
        otype: "sell".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    let sell_order3 = NewOrder {
        user_id: 8,
        cryptocurrency_id: 1,
        amount: 6.0,
        price: 100.0,
        otype: "sell".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    let sell_order4 = NewOrder {
        user_id: 9,
        cryptocurrency_id: 1,
        amount: 9.0,
        price: 110.0,
        otype: "sell".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    let sell_order5 = NewOrder {
        user_id: 10,
        cryptocurrency_id: 1,
        amount: 7.0,
        price: 88.0,
        otype: "sell".to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: "open".to_string(),
        market_true: false,
    };

    // Insert the buy orders and sell orders into the database

    // Make the request to the API
    let response = client.get("/matching_orders").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().expect("valid response body");
    assert_eq!(body, "Orders matched and processed successfully.");
}