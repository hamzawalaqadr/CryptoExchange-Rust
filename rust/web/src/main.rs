#[macro_use]
extern crate rocket;

use reqwest::header::{HeaderMap, HeaderValue};

use rocket::serde::json::Json;
use rocket::response::{status::Created, Debug};
use rocket::{get, post };
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use diesel::result::Error as DieselError;

use web::schema::user_details::dsl::*;
use web::schema::wallet;
use web::schema::realmoney;
use web::schema::transactions;
use web::schema::orders;
use web::schema::trade;
use web::schema::crypto;

use web::models::*;
use web::models::User;
use web::models::Price;
use web::models::Claims;
use web::establish_connection;

use diesel::prelude::*;

use chrono::Utc;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

use jsonwebtoken::{ 
    encode, EncodingKey, Header 
};

use stripe::{
    CardDetailsParams, CreatePaymentIntent, CreatePaymentMethod, 
    CreatePaymentMethodCardUnion, Currency, PaymentIntent, 
    PaymentIntentConfirmParams, PaymentMethod, PaymentMethodTypeFilter,
    UpdatePaymentIntent
};

use std::fmt;

pub enum MyError {
    DieselError(diesel::result::Error),
    NotFound,
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::DieselError(ref err) => write!(f, "Diesel Error: {}", err),
            MyError::NotFound => write!(f, "Not found"),
            MyError::Custom(ref err) => write!(f, "{}", err),
        }
    }
}

impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::DieselError(ref err) => write!(f, "Diesel Error: {:?}", err),
            MyError::NotFound => write!(f, "Not found"),
            MyError::Custom(ref err) => write!(f, "{}", err),
        }
    }
}

impl From<diesel::result::Error> for MyError {
    fn from(err: diesel::result::Error) -> MyError {
        MyError::DieselError(err)
    }
}

impl<'r> Responder<'r, 'static> for MyError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::response::Result::Err(rocket::http::Status::InternalServerError)
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![register_user, user_summary,
    withdraw_from_wallet, desposit_into_wallet,
    get_users,create_user,get_user,update_user,delete_user,login,
    create_wallet,get_wallet,get_wallets,update_wallet,delete_wallet,
    get_cryptos,get_crypto,create_crypto,update_crypto,delete_crypto, update_crypto_prices,
    create_rwallet,get_rwallet,get_rwallets,update_rwallet,delete_rwallet,
    create_trans,get_transs,get_trans,update_trans,delete_trans,
    create_order,get_order,get_orders,update_order,delete_order, matching_orders, 
    get_order_history, get_current_orders,
    create_trade,get_trade,get_trades,update_trade,delete_trade,
    ])
}

fn fetch_transactions(_user_id: i32) -> Result<Vec<Transaction>, Status> {

    use web::schema::transactions::dsl::*;
    let mut connection = establish_connection();
    
    let user_transactions = transactions
        .filter(user_id.eq(_user_id))
        .load::<Transaction>(&mut connection)
        .expect("Error loading transactions");

    Ok(user_transactions)
}

fn fetch_balance(_user_id: i32) -> Result<f64, Status> {

    use web::schema::realmoney::dsl::*;
    let mut connection = establish_connection();

    let user_balance = realmoney
    .filter(user_id.eq(_user_id))
    .select(balance)
    .first::<f64>(&mut connection)
    .expect("Could not find user");

    Ok(user_balance)
}

fn fetch_wallets(_user_id: i32) -> Result<Vec<Wallet>, Status> {

    use web::schema::wallet::dsl::*;
    let mut connection = establish_connection();
    
    let user_wallets = wallet
        .filter(user_id.eq(_user_id))
        .load::<Wallet>(&mut connection)
        .expect("Error loading wallets");

    Ok(user_wallets)
}


#[get("/users/summary/<_user_id>")]
async fn user_summary(_user_id: i32) -> Json<Vec<UserSummary>> {

    let user_transactions = match fetch_transactions(_user_id) {
        Ok(transactions) => transactions,
        Err(_status) => { vec![] }
    };

    let user_balance = match fetch_balance(_user_id) {
        Ok(balance) => balance,
        Err(_) => { 0.0 }
    };

    let user_wallets = match fetch_wallets(_user_id) {
        Ok(wallets) => wallets,
        Err(_) => { vec![] }
    };

    let user_summary = UserSummary {
        transactions: user_transactions,
        balance: user_balance,
        wallets: user_wallets,
    };

    Json(vec![user_summary])
}

#[get("/users")]
async fn get_users() -> Option<Json<Vec<User>>> {
    let mut connection = establish_connection();
    let results = user_details
        .limit(5)
        .load::<User>(&mut connection)
        .expect("Error loading users");
    Some(Json(results))
}
#[get("/users/<_id>")]
async fn get_user(_id: i32) -> Result<Json<User>, Status> {
    let mut connection = establish_connection();


    let user = user_details
        .filter(id.eq(_id))
        .first::<User>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(user))
}

#[put("/users/<_id>", data = "<new_user>")]
async fn update_user(_id: i32, new_user: Json<NewUser>) -> Result<Json<User>, Status>{
    use web::models::*;

    let mut connection = establish_connection();

    let new_user = new_user.into_inner();

    let user = user_details
        .filter(id.eq(_id))
        .first::<User>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(&user)
    .set((  
    id.eq(&user.id),
        user_name.eq(new_user.user_name),
        password.eq(new_user.password),
        email.eq(new_user.email),
        created_on.eq(user.created_on),
        modified_on.eq(Utc::now().naive_utc())
    ))
    .get_result::<User>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)

}

#[delete("/users/<_id>")]
async fn delete_user(_id: i32) -> Result<Json<User>, Status>{
    use web::models::*;

    let mut connection = establish_connection();

    diesel::delete(user_details.filter(id.eq(_id)))
        .get_result::<User>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)

}

#[post("/users/register", data = "<new_user>")]
async fn register_user(new_user: rocket::serde::json::Json<NewUser>) -> String {
    let mut connection = establish_connection();

    let inserted_user = diesel::insert_into(web::schema::user_details::dsl::user_details)
        .values(NewUser {
            user_name: new_user.user_name.clone(),
            password: new_user.password.clone(),
            email: new_user.email.clone(),
            created_on: Some(Utc::now().naive_utc()),
            modified_on: Some(Utc::now().naive_utc()),
        })
        .get_result::<User>(&mut connection)
        .expect("Failed to insert new user");

    let inserted_wallet = diesel::insert_into(web::schema::realmoney::dsl::realmoney)
        .values(NewRealMoneyWallet {
            user_id: inserted_user.id,
            currency: "USD".to_string(), // Set the currency for the wallet here
            balance: 0.0,                // Set the initial balance for the wallet here
            created_on: Some(Utc::now().naive_utc()),
            modified_on: Some(Utc::now().naive_utc()),
        })
        .get_result::<RealMoneyWallet>(&mut connection)
        .expect("Failed to insert new wallet");

    format!(
        "Registered user {} with user ID {} and wallet ID {}",
        new_user.user_name, inserted_user.id, inserted_wallet.id
    )
}

#[post("/users", format = "json", data = "<new_user>")]
async fn create_user(new_user: Json<NewUser>) -> Result<Created<Json<NewUser>>> {
    let mut connection = establish_connection();

    let new_user1 = NewUser {
        user_name: new_user.user_name.to_string(),
        password: new_user.password.to_string(),
        email: new_user.email.to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(web::schema::user_details::dsl::user_details)
        .values(&new_user1)
        .execute(&mut connection)
        .expect("Error saving new post");

    Ok(Created::new("/").body(new_user))

}

async fn fetch_symbol_price(symbol: &str) -> Option<f64> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("rocket"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .ok()?;

    let response = client.get(&url).send().await.ok()?;
    if response.status().is_success() {
        let price: Price = response.json().await.ok()?;
        price.price.parse().ok()
    } else {
        None
    }
}

#[post("/users/login", format = "application/json", data = "<login_data>")]
async fn login(login_data: Json<LoginData>) -> Result<String, Status> {
    let mut connection = establish_connection();

    let user_result = user_details
        .filter(user_name.eq(&login_data.user_name))
        .first::<User>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    if user_result.password == login_data.password {
        // Generate the JWT token
        let token = generate_token(user_result.id)?;

        // Return the JWT token as the response
        Ok(token)
    } else {
        Err(Status::Unauthorized)
    }
}

fn generate_token(user_id: i32) -> Result<String, Status> {
    // Define the claims for the JWT token
    let claims = Claims {
        sub: user_id.to_string(),
        // Add any other required claims (e.g., expiration time)
    };

    // Define the JWT secret key
    let secret_key = "your-secret-key"; // Replace with your own secret key

    // Generate the JWT token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .map_err(|_| Status::InternalServerError)?;

    Ok(token)
}


#[post("/wallet", format = "application/json", data = "<new_wallet>")]
async fn create_wallet(new_wallet: rocket::serde::json::Json<NewWallet>) -> String {
    let mut connection = establish_connection();

    let mut response = String::new();

    // Try to fetch the existing wallet
    let existing_wallet: Result<Wallet, diesel::result::Error> = wallet::table
        .filter(wallet::user_id.eq(new_wallet.user_id))
        .filter(wallet::cryptocurrency_id.eq(new_wallet.cryptocurrency_id))
        .first(&mut connection);

    match existing_wallet {
        Ok(mut wallet) => {
            // If the wallet exists, increase the balance
            wallet.balance += new_wallet.balance;
            wallet.modified_on = Some(Utc::now().naive_utc());
            diesel::update(wallet::table.find(wallet.id))
                .set((wallet::balance.eq(wallet.balance), wallet::modified_on.eq(wallet.modified_on)))
                .execute(&mut connection)
                .expect("Failed to update balance in wallet");

            response = format!(
                "Updated Crypto Wallet with crypto ID {} for user ID {}. New balance: {}",
                new_wallet.cryptocurrency_id, new_wallet.user_id, wallet.balance
            );

            // Insert transaction for updated wallet
            diesel::insert_into(web::schema::transactions::dsl::transactions)
                .values(&NewTransaction {
                    user_id: new_wallet.user_id,
                    wallet_id: wallet.id, // Use the existing wallet's ID
                    rwallet_id: wallet.id, // Assuming the real wallet ID is the same as the wallet ID
                    cryptocurrency_id: new_wallet.cryptocurrency_id,
                    ttype: "deposit".to_string(),
                    amount: new_wallet.balance,
                    created_on: Some(Utc::now().naive_utc()),
                    modified_on: Some(Utc::now().naive_utc()),
                    payment_method: "".to_string(),
                    payment_amount: 0.0,
                    payment_status: "".to_string(),
                })
                .get_result::<Transaction>(&mut connection)
                .expect("Failed to insert new transaction into table");
        },
        Err(diesel::result::Error::NotFound) => {
            // If the wallet does not exist, create a new one
            let inserted_wallet = diesel::insert_into(wallet::table)
                .values(NewWallet {
                    user_id: new_wallet.user_id,
                    cryptocurrency_id: new_wallet.cryptocurrency_id,
                    balance: new_wallet.balance,
                    created_on: Some(Utc::now().naive_utc()),
                    modified_on: Some(Utc::now().naive_utc()),
                })
                .get_result::<Wallet>(&mut connection)
                .expect("Failed to insert new crypto into wallet");

            response = format!(
                "Created Crypto Wallet with crypto ID {} for user ID {}",
                new_wallet.cryptocurrency_id, new_wallet.user_id
            );

            // Insert transaction for new wallet
            diesel::insert_into(web::schema::transactions::dsl::transactions)
                .values(&NewTransaction {
                    user_id: new_wallet.user_id,
                    wallet_id: inserted_wallet.id, // Use the new wallet's ID
                    rwallet_id: inserted_wallet.id, // Assuming the real wallet ID is the same as the wallet ID
                    cryptocurrency_id: new_wallet.cryptocurrency_id,
                    ttype: "deposit".to_string(),
                    amount: new_wallet.balance,
                    created_on: Some(Utc::now().naive_utc()),
                    modified_on: Some(Utc::now().naive_utc()),
                    payment_method: "".to_string(),
                    payment_amount: 0.0,
                    payment_status: "".to_string(),
                })
                .get_result::<Transaction>(&mut connection)
                .expect("Failed to insert new transaction into table");
        },
        Err(err) => {
            println!("Failed to query wallet: {}", err);
            return format!("Failed to create or update wallet: {}", err);
        }
    }

    response
}



#[get("/wallet")]
async fn get_wallets() -> Option<Json<Vec<Wallet>>> {
    let mut connection = establish_connection();
    let results = wallet::table
        .limit(5)
        .load::<Wallet>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/wallet/<_id>")]
async fn get_wallet(_id: i32) -> Result<Json<Wallet>, Status> {

    let mut connection = establish_connection();

    let w = wallet::table
        .filter(web::schema::wallet::id.eq(_id))
        .first::<Wallet>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(w))
}

#[put("/wallet/<wallet_id>", data = "<new_wallet>")]
async fn update_wallet(wallet_id: i32, new_wallet: Json<NewWallet>) -> Result<Json<Wallet>, Status>{
    use web::schema::wallet::dsl::*;

    let connection = establish_connection();
    let mut connection = connection;

    let target = wallet.filter(id.eq(wallet_id));
    let new_wallet = new_wallet.into_inner();

    let w = web::schema::wallet::table
        .filter(id.eq(wallet_id))
        .first::<Wallet>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&w.id),
            user_id.eq(w.user_id),
            cryptocurrency_id.eq(new_wallet.cryptocurrency_id),
            balance.eq(new_wallet.balance),
            created_on.eq(w.created_on),
            modified_on.eq(Utc::now().naive_utc())
    ))
    .get_result::<Wallet>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/wallet/<_id>")]
async fn delete_wallet(_id: i32) -> Result<Json<Wallet>, Status>{
    use web::schema::wallet::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(wallet.filter(id.eq(_id)))
        .get_result::<Wallet>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)

}

#[get("/crypto")]
async fn get_cryptos() -> Option<Json<Vec<Crypto>>> {
    let mut connection = establish_connection();
    use web::schema::crypto::dsl::*;
    let results = crypto
        .limit(5)
        .load::<Crypto>(&mut connection)
        .expect("Error loading crypto currencies");

    Some(Json(results))
}

#[get("/crypto/<_id>")]
async fn get_crypto(_id: i32) -> Result<Json<Crypto>, Status> {
    let mut connection = establish_connection();
    use web::schema::crypto::dsl::*;
    let currency = crypto
        .filter(web::schema::crypto::id.eq(_id))
        .first::<Crypto>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(currency))
}

#[post("/crypto", format = "json", data = "<new_crypto>")]
async fn create_crypto(new_crypto: Json<NewCrypto>) -> Result<Created<Json<Crypto>>, Status> {
    let mut connection = establish_connection();
    use web::schema::crypto::dsl::*;
    let new_crypto = NewCrypto {
        cname: new_crypto.cname.to_string(),
        symbol: new_crypto.symbol.to_string(),
        price: new_crypto.price,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    let result = diesel::insert_into(crypto)
        .values(&new_crypto)
        .get_result::<Crypto>(&mut connection)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Created::new("/crypto").body(Json(result)))
}

#[get("/update_crypto_prices")]
async fn update_crypto_prices() -> String {
    let symbols = vec![
        "BTCUSDT",
        "ETHUSDT",
        "BNBBTC",
        "XRPRUB",
        "LTCUSDT",
        "ADAUSDT",
        "DOGEUSDT",
        "LINKBTC",
        "XMRUSDT",
        "DOTUSDT",
    ];

    // Create a mapping of symbol to official name
    let crypto_names: std::collections::HashMap<&str, &str> = [
        ("BTCUSDT", "Bitcoin"),
        ("ETHUSDT", "Ethereum"),
        ("BNBBTC", "Binance Coin"),
        ("XRPRUB", "Ripple"),
        ("LTCUSDT", "Litecoin"),
        ("ADAUSDT", "Cardano"),
        ("DOGEUSDT", "Dogecoin"),
        ("LINKBTC", "Chainlink"),
        ("XMRUSDT", "Monero"),
        ("DOTUSDT", "Polkadot"),
    ].iter().cloned().collect();

    let mut connection = establish_connection();

    for symbol in symbols {
        if let Some(price) = fetch_symbol_price(&symbol).await {
            // Get official name, or default to symbol if not found
            let cname = crypto_names.get(symbol).unwrap_or(&symbol).to_string();

            // Check if crypto already exists
            let existing_crypto: Result<Crypto, diesel::result::Error> = crypto::table
                .filter(crypto::symbol.eq(symbol))
                .first(&mut connection);

            match existing_crypto {
                Ok(mut crypto) => {
                    // Update the price and modified_on date for the existing crypto
                    crypto.price = price;
                    crypto.modified_on = Some(Utc::now().naive_utc());
                    if let Err(err) = diesel::update(crypto::table.filter(crypto::symbol.eq(&symbol)))
                        .set((crypto::price.eq(crypto.price), crypto::modified_on.eq(crypto.modified_on)))
                        .execute(&mut connection)
                    {
                        println!("Failed to update crypto for symbol {}: {}", symbol, err);
                    }
                },
                Err(diesel::result::Error::NotFound) => {
                    // Create a new crypto
                    let new_crypto = NewCrypto {
                        cname,
                        symbol: symbol.to_string(),
                        price,
                        created_on: Some(Utc::now().naive_utc()),
                        modified_on: Some(Utc::now().naive_utc()),
                    };
                    if let Err(err) = diesel::insert_into(crypto::table)
                        .values(&new_crypto)
                        .execute(&mut connection)
                    {
                        println!("Failed to insert crypto for symbol {}: {}", symbol, err);
                    }
                },
                Err(err) => {
                    println!("Failed to query crypto for symbol {}: {}", symbol, err);
                },
            }
        } else {
            println!("Failed to fetch price for symbol: {}", symbol);
        }
    }

    "Crypto prices updated successfully.".to_string()
}


#[put("/crypto/<_id>", format = "json", data = "<new_crypto>")]
async fn update_crypto(_id: i32,new_crypto: Json<NewCrypto>,) -> Result<Json<Crypto>, Status> {
    let mut connection = establish_connection();
    use web::schema::crypto::dsl::*;
    let target = crypto.filter(id.eq(_id));
    let new_crypto = new_crypto.into_inner();

    let c = web::schema::crypto::table
        .filter(id.eq(_id))
        .first::<Crypto>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&c.id),
            cname.eq(new_crypto.cname),
            symbol.eq(new_crypto.symbol),
            price.eq(new_crypto.price),
            created_on.eq(c.created_on),
            modified_on.eq(Utc::now().naive_utc())
    ))
    .get_result::<Crypto>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/crypto/<_id>")]
async fn delete_crypto(_id: i32) -> Result<Json<Crypto>, Status> {
    use web::schema::crypto::dsl::*;
    let mut connection = establish_connection();

    diesel::delete(crypto.filter(id.eq(_id)))
        .get_result::<Crypto>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/realmoney", format = "application/json", data = "<new_rwallet>")]
async fn create_rwallet(new_rwallet: Json<NewRealMoneyWallet>) -> Result<Created<Json<NewRealMoneyWallet>>> {
    let mut connection = establish_connection();

    let new_rwallet1 = NewRealMoneyWallet {
        user_id: new_rwallet.user_id,
        currency: new_rwallet.currency.to_string(),
        balance: new_rwallet.balance,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(web::schema::realmoney::dsl::realmoney)
        .values(&new_rwallet1)
        .execute(&mut connection)
        .expect("Error saving new wallet");
    Ok(Created::new("/").body(Json(new_rwallet1)))
}

#[get("/realmoney")]
async fn get_rwallets() -> Option<Json<Vec<RealMoneyWallet>>> {
    let mut connection = establish_connection();
    let results = realmoney::table
        .limit(5)
        .load::<RealMoneyWallet>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/realmoney/<_id>")]
async fn get_rwallet(_id: i32) -> Result<Json<RealMoneyWallet>, Status> {

    let mut connection = establish_connection();

    let r = realmoney::table
        .filter(web::schema::realmoney::id.eq(_id))
        .first::<RealMoneyWallet>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(r))
}

#[put("/realmoney/<rwallet_id>", data = "<new_rwallet>")]
async fn update_rwallet(rwallet_id: i32, new_rwallet: Json<NewRealMoneyWallet>) -> Result<Json<RealMoneyWallet>, Status>{
    use web::schema::realmoney::dsl::*;

    let mut connection = establish_connection();

    let target = realmoney.filter(id.eq(rwallet_id));
    let new_rwallet = new_rwallet.into_inner();

    let r = web::schema::realmoney::table
        .filter(id.eq(rwallet_id))
        .first::<RealMoneyWallet>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&r.id),
            user_id.eq(r.user_id),
            currency.eq(new_rwallet.currency),
            balance.eq(new_rwallet.balance),
            created_on.eq(r.created_on),
            modified_on.eq(Utc::now().naive_utc())
    ))
    .get_result::<RealMoneyWallet>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/realmoney/<_id>")]
async fn delete_rwallet(_id: i32) -> Result<Json<RealMoneyWallet>, Status>{
    use web::schema::realmoney::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(realmoney.filter(id.eq(_id)))
        .get_result::<RealMoneyWallet>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/transactions", format = "application/json", data = "<new_t>")]
async fn create_trans(new_t: Json<NewTransaction>) -> Result<Created<Json<NewTransaction>>> {
    let mut connection = establish_connection();

    let new_t1 = NewTransaction {
        user_id: new_t.user_id,
        wallet_id: new_t.wallet_id,
        rwallet_id: new_t.rwallet_id,
        cryptocurrency_id: new_t.cryptocurrency_id,
        ttype: "deposit".to_string(),
        amount: new_t.amount,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        payment_method: new_t.payment_method.to_string(),
        payment_amount: new_t.payment_amount,
        payment_status: new_t.payment_status.to_string()
    };

    diesel::insert_into(web::schema::transactions::dsl::transactions)
        .values(&new_t1)
        .execute(&mut connection)
        .expect("Error saving new wallet");
    Ok(Created::new("/transactions/deposit").body(Json(new_t1)))
}

#[post("/transactions/withdraw", format = "application/json", data = "<withdrawal>")]
async fn withdraw_from_wallet(withdrawal: Json<NewTransaction>) -> Result<Created<Json<Transaction>>, Status> {
    let mut connection = establish_connection();

    // Retrieve the user's real money wallet
    let user_wallet = realmoney::table
        .filter(realmoney::user_id.eq(withdrawal.user_id))
        .first::<RealMoneyWallet>(&mut connection)
        .map_err(|_| Status::InternalServerError)?;

    // Check if the withdrawal amount is valid
    if withdrawal.amount > user_wallet.balance {
        return Err(Status::BadRequest);
    }

    // Create a new transaction record for the withdrawal
    let new_transaction = NewTransaction {
        user_id: withdrawal.user_id,
        wallet_id: withdrawal.wallet_id,
        rwallet_id: user_wallet.id,
        cryptocurrency_id: withdrawal.cryptocurrency_id,
        ttype: "withdraw".to_string(),
        amount: 0.0,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        payment_method: "realmoneywallet".to_string(), // Update with the actual payment method
        payment_amount: withdrawal.payment_amount,            // Update with the actual payment amount
        payment_status: "completed".to_string(), // Update with the actual payment status
    };

    // Perform additional checks and validations here, if needed

    // Update the user's real money wallet balance
    let updated_balance = user_wallet.balance - withdrawal.amount;

    diesel::update(realmoney::table.find(user_wallet.id))
        .set(realmoney::balance.eq(updated_balance))
        .execute(&mut connection)
        .map_err(|_| Status::InternalServerError)?;

    // Insert the new transaction record into the database
    let result = diesel::insert_into(web::schema::transactions::dsl::transactions)
        .values(&new_transaction)
        .get_result::<Transaction>(&mut connection)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Created::new("/transactions").body(Json(result)))
}


#[post("/transactions/deposit", format = "application/json", data = "<new_t2>")]
async fn desposit_into_wallet(new_t2: Json<NewTransaction>) -> Result<Created<Json<Transaction>>, Status> {
    let mut connection = establish_connection();
    // use web::schema::transaction::dsl::*;
    let new_transaction = NewTransaction {
        user_id: new_t2.user_id,
        wallet_id: new_t2.wallet_id,
        rwallet_id: new_t2.rwallet_id,
        cryptocurrency_id: new_t2.cryptocurrency_id,
        ttype: new_t2.ttype.to_string(),
        amount: new_t2.amount,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        payment_method: new_t2.payment_method.to_string(),
        payment_amount: new_t2.payment_amount,
        payment_status: "completed".to_string()
    };

    // Create a payment intent and confirm it
    // let secret_key = std::env::var("sk_test_51MfTJqASW7Sg3dtTqoXv4ADxyD509b9h6SWzC52gLPlcrgPhwEm9PcrnAQkdzHzviGLlDzIrFdmhuq7VyGnz1Jmm00Lk0pmBeI").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new("sk_test_51MfTJqASW7Sg3dtTqoXv4ADxyD509b9h6SWzC52gLPlcrgPhwEm9PcrnAQkdzHzviGLlDzIrFdmhuq7VyGnz1Jmm00Lk0pmBeI");

    let card_number = &new_transaction.payment_method; // Assuming this is the card number
    let exp_month = 3; // Replace with actual value
    let exp_year = 2024; // Replace with actual value
    let cvc = "314"; // Replace with actual value
    let amount = (new_transaction.payment_amount * 100.0) as i64; // Amount in cents
    let mut create_intent = CreatePaymentIntent::new(amount, Currency::USD);
    create_intent.payment_method_types = Some(vec!["card".to_string()]);
    let payment_intent = PaymentIntent::create(&client, create_intent).await.unwrap();

    let pm = PaymentMethod::create(
        &client,
        CreatePaymentMethod {
            type_: Some(PaymentMethodTypeFilter::Card),
            card: Some(CreatePaymentMethodCardUnion::CardDetailsParams(CardDetailsParams {
                number: card_number.to_string(),
                exp_year: exp_year,
                exp_month: exp_month,
                cvc: Some(cvc.to_string()),
                ..Default::default()
            })),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let payment_intent = PaymentIntent::update(
        &client,
        &payment_intent.id,
        UpdatePaymentIntent {
            payment_method: Some(pm.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    PaymentIntent::confirm(
        &client,
        &payment_intent.id,
        PaymentIntentConfirmParams { ..Default::default() },
    )
    .await
    .unwrap();

    // new_transaction.payment_status = "completed".to_string();

    let result = diesel::insert_into(web::schema::transactions::dsl::transactions)
        .values(&new_transaction)
        .get_result::<Transaction>(&mut connection)
        .map_err(|_| Status::InternalServerError)?;

    // Update the user's real money wallet balance
    let user_wallet = realmoney::table
    .filter(realmoney::user_id.eq(new_transaction.user_id))
    .first::<RealMoneyWallet>(&mut connection)
    .map_err(|_| Status::InternalServerError)?;

    let updated_balance = user_wallet.balance + new_transaction.payment_amount;

    diesel::update(realmoney::table.find(user_wallet.id))
    .set(realmoney::balance.eq(updated_balance))
    .execute(&mut connection)
    .map_err(|_| Status::InternalServerError)?;

    Ok(Created::new("/transactions").body(Json(result)))
}

#[get("/transactions")]
async fn get_transs() -> Option<Json<Vec<Transaction>>> {
    let mut connection = establish_connection();
    let results = transactions::table
        .limit(5)
        .load::<Transaction>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/transactions/<_id>")]
async fn get_trans(_id: i32) -> Result<Json<Transaction>, Status> {

    let mut connection = establish_connection();

    let t = transactions::table
        .filter(web::schema::transactions::id.eq(_id))
        .first::<Transaction>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(t))
}

#[put("/transactions/<_id>", data = "<new_t>")]
async fn update_trans(_id: i32, new_t: Json<NewTransaction>) -> Result<Json<Transaction>, Status>{
    use web::schema::transactions::dsl::*;

    let mut connection = establish_connection();

    let target = transactions.filter(id.eq(_id));
    let new_t = new_t.into_inner();

    let t = web::schema::transactions::table
        .filter(id.eq(_id))
        .first::<Transaction>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&t.id),
            user_id.eq(new_t.user_id),
            wallet_id.eq(new_t.wallet_id),
            rwallet_id.eq(new_t.rwallet_id),
            cryptocurrency_id.eq(new_t.cryptocurrency_id),
            ttype.eq(new_t.ttype),
            amount.eq(new_t.amount),
            created_on.eq(t.created_on),
            modified_on.eq(Utc::now().naive_utc()),
            payment_method.eq(new_t.payment_method),
            payment_amount.eq(new_t.payment_amount),
            payment_status.eq(new_t.payment_status)
    ))
    .get_result::<Transaction>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/transactions/<_id>")]
async fn delete_trans(_id: i32) -> Result<Json<Transaction>, Status>{
    use web::schema::transactions::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(transactions.filter(id.eq(_id)))
        .get_result::<Transaction>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)

}

#[post("/orders", format = "application/json", data = "<new_o>")]
async fn create_order(new_o: Json<NewOrder>) -> Result<Created<Json<NewOrder>>, MyError> {
    let mut connection = establish_connection();

    let mut new_o1 = NewOrder {
        user_id: new_o.user_id,
        cryptocurrency_id: new_o.cryptocurrency_id,
        amount: new_o.amount,
        price: new_o.price,
        otype: new_o.otype.to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: new_o.ostatus.to_string(),
        market_true: new_o.market_true
    };

    // If market_true is true, retrieve the market price of the cryptocurrency from the crypto table
    if new_o1.market_true {
        update_crypto_prices().await;
        let cryptocurrency = crypto::table
            .find(new_o1.cryptocurrency_id)
            .first::<Crypto>(&mut connection)?;

        new_o1.price = cryptocurrency.price;
    }

    // Check for the 'sell' order if the user has enough balance in his crypto wallet
    if new_o1.otype == "sell" {
        let wallet: Wallet = wallet::table
            .filter(wallet::user_id.eq(new_o1.user_id))
            .filter(wallet::cryptocurrency_id.eq(new_o1.cryptocurrency_id))
            .first(&mut connection)?;

        if wallet.balance < new_o1.amount {
            return Err(MyError::Custom("Not enough balance in crypto wallet".to_string()));
        }
    }

    // Check for the 'buy' order if the user has enough balance in his real money wallet
    if new_o1.otype == "buy" {
        let realmoney_wallet: RealMoneyWallet = realmoney::table
            .filter(realmoney::user_id.eq(new_o1.user_id))
            .first(&mut connection)?;

        if realmoney_wallet.balance < new_o1.price * new_o1.amount {
            return Err(MyError::Custom("Not enough balance in real money wallet".to_string()));
        }
    }

    diesel::insert_into(web::schema::orders::dsl::orders)
        .values(&new_o1)
        .execute(&mut connection)?;

    Ok(Created::new("/").body(Json(new_o1)))
}




#[get("/orders")]
async fn get_orders() -> Option<Json<Vec<Order>>> {
    let mut connection = establish_connection();
    let results = orders::table
        .limit(5)
        .load::<Order>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}

#[get("/orders/history/<_user_id>")]
fn get_order_history(_user_id: i32) -> Json<Vec<Order>> {
    use web::schema::orders::dsl::*;

    let mut connection = establish_connection();
    let results = orders
        .filter(user_id.eq(_user_id).and(ostatus.eq("closed")))
        .load::<Order>(&mut connection)
        .expect("Error loading orders");

    Json(results)
}

#[get("/orders/current/<_user_id>")]
fn get_current_orders(_user_id: i32) -> Json<Vec<Order>> {
    use web::schema::orders::dsl::*;

    let mut connection = establish_connection();
    let results = orders
        .filter(user_id.eq(_user_id).and(ostatus.eq("open")))
        .load::<Order>(&mut connection)
        .expect("Error loading orders");

    Json(results)
}

#[get("/orders/<_id>")]
async fn get_order(_id: i32) -> Result<Json<Order>, Status> {

    let mut connection = establish_connection();

    let o = orders::table
        .filter(web::schema::orders::id.eq(_id))
        .first::<Order>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(o))
}

#[put("/orders/<_id>", data = "<new_o>")]
async fn update_order(_id: i32, new_o: Json<NewOrder>) -> Result<Json<Order>, Status>{
    use web::schema::orders::dsl::*;

    let mut connection = establish_connection();

    let target = orders.filter(id.eq(_id));
    let new_o = new_o.into_inner();

    let o = web::schema::orders::table
        .filter(id.eq(_id))
        .first::<Order>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&o.id),
            user_id.eq(o.user_id),
            cryptocurrency_id.eq(new_o.cryptocurrency_id),
            amount.eq(new_o.amount),
            price.eq(new_o.price),
            otype.eq(new_o.otype),
            created_on.eq(o.created_on),
            modified_on.eq(Utc::now().naive_utc()),
            ostatus.eq(new_o.ostatus),
            market_true.eq(new_o.market_true)
    ))
    .get_result::<Order>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/orders/<_id>")]
async fn delete_order(_id: i32) -> Result<Json<Order>, Status>{
    use web::schema::orders::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(orders.filter(id.eq(_id)))
        .get_result::<Order>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)

}

#[post("/trade", format = "application/json", data = "<new_tr>")]
async fn create_trade(new_tr: Json<NewTrade>) -> Result<Created<Json<NewTrade>>> {
    let mut connection = establish_connection();

    let new_tr1 = NewTrade {
        buyer_id: new_tr.buyer_id,
        seller_id: new_tr.seller_id,
        cryptocurrency_id: new_tr.cryptocurrency_id,
        amount: new_tr.amount,
        price: new_tr.price,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(web::schema::trade::dsl::trade)
        .values(&new_tr1)
        .execute(&mut connection)
        .expect("Error saving new wallet");
    Ok(Created::new("/").body(Json(new_tr1)))
}

#[get("/trade")]
async fn get_trades() -> Option<Json<Vec<Trade>>> {
    let mut connection = establish_connection();
    let results = trade::table
        .limit(5)
        .load::<Trade>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/trade/<_id>")]
async fn get_trade(_id: i32) -> Result<Json<Trade>, Status> {

    let mut connection = establish_connection();

    let tr = trade::table
        .filter(web::schema::trade::id.eq(_id))
        .first::<Trade>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(tr))
}

#[put("/trade/<_id>", data = "<new_tr>")]
async fn update_trade(_id: i32, new_tr: Json<NewTrade>) -> Result<Json<Trade>, Status>{
    use web::schema::trade::dsl::*;

    let mut connection = establish_connection();

    let target = trade.filter(id.eq(_id));
    let new_tr = new_tr.into_inner();

    let tr = web::schema::trade::table
        .filter(id.eq(_id))
        .first::<Trade>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&tr.id),
            buyer_id.eq(new_tr.buyer_id),
            seller_id.eq(new_tr.seller_id),
            cryptocurrency_id.eq(new_tr.cryptocurrency_id),
            amount.eq(new_tr.amount),
            price.eq(new_tr.price),
            created_on.eq(tr.created_on),
            modified_on.eq(Utc::now().naive_utc()),
    ))
    .get_result::<Trade>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/trade/<_id>")]
async fn delete_trade(_id: i32) -> Result<Json<Trade>, Status>{
    use web::schema::trade::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(trade.filter(id.eq(_id)))
        .get_result::<Trade>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/matching_orders")]
async fn matching_orders() -> Result<String, Status> {
    let mut connection = establish_connection();

    // Retrieve all open buy orders sorted by creation time
    let mut open_buy_orders = orders::table
        .filter(orders::ostatus.eq("open").and(orders::otype.eq("buy")))
        .order_by(orders::created_on.asc())
        .load::<Order>(&mut connection)
        .map_err(|_| Status::InternalServerError)?;

    for earliest_buy_order in &mut open_buy_orders {
        // Retrieve open sell orders with price <= buy order's price and matching cryptocurrency_id
        let matching_sell_orders = orders::table
            .filter(
                orders::ostatus.eq("open")
                    .and(orders::otype.eq("sell"))
                    .and(orders::price.le(earliest_buy_order.price))
                    .and(orders::cryptocurrency_id.eq(earliest_buy_order.cryptocurrency_id)),
            )
            .load::<Order>(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        // If there are no matching sell orders, skip to the next buy order
        if matching_sell_orders.is_empty() {
            continue;
        }
        

    // Match and process the orders
    for mut sell_order in matching_sell_orders {
        // Determine the trade amount
        let trade_amount = sell_order.amount.min(earliest_buy_order.amount);
        // Create a trade record
        let new_trade = NewTrade {
            buyer_id: earliest_buy_order.user_id,
            seller_id: sell_order.user_id,
            cryptocurrency_id: earliest_buy_order.cryptocurrency_id,
            amount: trade_amount,
            price: sell_order.price,
            created_on: Some(Utc::now().naive_utc()),
            modified_on: Some(Utc::now().naive_utc()),
        };

        // Insert the trade record into the database
        diesel::insert_into(trade::table)
            .values(&new_trade)
            .execute(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        // Deduct the trade amount from the buyer's wallet
        earliest_buy_order.amount -= trade_amount;

        // If the buy order amount is exhausted, close it
        if earliest_buy_order.amount == 0.0 {
            diesel::update(orders::table.find(earliest_buy_order.id))
                .set(orders::ostatus.eq("closed"))
                .execute(&mut connection)
                .map_err(|_| Status::InternalServerError)?;
        } else {
            // Otherwise, update the remaining amount
            diesel::update(orders::table.find(earliest_buy_order.id))
                .set(orders::amount.eq(earliest_buy_order.amount))
                .execute(&mut connection)
                .map_err(|_| Status::InternalServerError)?;
        }

        // Deduct the trade amount from the sell order
        sell_order.amount -= trade_amount;

        // If the sell order amount is exhausted, close it
        if sell_order.amount == 0.0 {
            diesel::update(orders::table.find(sell_order.id))
                .set(orders::ostatus.eq("closed"))
                .execute(&mut connection)
                .map_err(|_| Status::InternalServerError)?;
        } else {
            // Otherwise, update the remaining amount
            diesel::update(orders::table.find(sell_order.id))
                .set(orders::amount.eq(sell_order.amount))
                .execute(&mut connection)
                .map_err(|_| Status::InternalServerError)?;
        }

        // Deduct money from the buyer's user realmoneywallet
        let buyer_realmoneywallet = realmoney::table
            .filter(realmoney::user_id.eq(earliest_buy_order.user_id))
            .first::<RealMoneyWallet>(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        let updated_buyer_balance = buyer_realmoneywallet.balance - (sell_order.price * new_trade.amount);

        diesel::update(realmoney::table.find(buyer_realmoneywallet.id))
            .set(realmoney::balance.eq(updated_buyer_balance))
            .execute(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        // Increment money in the seller's user realmoneywallet
        let seller_realmoneywallet = realmoney::table
            .filter(realmoney::user_id.eq(sell_order.user_id))
            .first::<RealMoneyWallet>(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        let updated_seller_balance = seller_realmoneywallet.balance + (sell_order.price * new_trade.amount);

        diesel::update(realmoney::table.find(seller_realmoneywallet.id))
            .set(realmoney::balance.eq(updated_seller_balance))
            .execute(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        // Check if the buyer has a wallet for the cryptocurrency
        let buyer_wallet = wallet::table
            .filter(wallet::user_id.eq(earliest_buy_order.user_id))
            .filter(wallet::cryptocurrency_id.eq(earliest_buy_order.cryptocurrency_id))
            .first::<Wallet>(&mut connection)
            .optional()
            .map_err(|_| Status::InternalServerError)?;

        if let Some(mut buyer_wallet) = buyer_wallet {
            // Update the buyer's wallet balance
            buyer_wallet.balance += new_trade.amount;

            diesel::update(wallet::table.find(buyer_wallet.id))
                .set(&buyer_wallet)
                .execute(&mut connection)
                .map_err(|_| Status::InternalServerError)?;
        } else {
            // Create a new wallet entry for the buyer
            let new_wallet = NewWallet {
                user_id: earliest_buy_order.user_id,
                cryptocurrency_id: earliest_buy_order.cryptocurrency_id,
                balance: new_trade.amount,
                created_on: Some(Utc::now().naive_utc()),
                modified_on: Some(Utc::now().naive_utc()),
            };

            diesel::insert_into(wallet::table)
                .values(&new_wallet)
                .execute(&mut connection)
                .map_err(|_| Status::InternalServerError)?;
        }

        // Check if the seller has a wallet for the cryptocurrency
        let seller_wallet = wallet::table
            .filter(wallet::user_id.eq(sell_order.user_id))
            .filter(wallet::cryptocurrency_id.eq(earliest_buy_order.cryptocurrency_id))
            .first::<Wallet>(&mut connection)
            .optional()
            .map_err(|_| Status::InternalServerError)?;

        if let Some(mut seller_wallet) = seller_wallet {
            // Update the seller's wallet balance
            seller_wallet.balance -= new_trade.amount;

            diesel::update(wallet::table.find(seller_wallet.id))
                .set(&seller_wallet)
                .execute(&mut connection)
                .map_err(|_| Status::InternalServerError)?;
        } else {
            // Create a new wallet entry for the seller
            let new_wallet = NewWallet {
                user_id: sell_order.user_id,
                cryptocurrency_id: earliest_buy_order.cryptocurrency_id,
                balance: new_trade.amount,
                created_on: Some(Utc::now().naive_utc()),
                modified_on: Some(Utc::now().naive_utc()),
            };

            diesel::insert_into(wallet::table)
                .values(&new_wallet)
                .execute(&mut connection)
                .map_err(|_| Status::InternalServerError)?;
        }

        // Retrieve the wallet IDs of the buyer and seller
        let buyer_walletid = wallet::table
            .filter(wallet::user_id.eq(earliest_buy_order.user_id).and(wallet::cryptocurrency_id.eq(earliest_buy_order.cryptocurrency_id)))
            .select(wallet::id)
            .first::<i32>(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        let seller_walletid = wallet::table
            .filter(wallet::user_id.eq(sell_order.user_id).and(wallet::cryptocurrency_id.eq(sell_order.cryptocurrency_id)))
            .select(wallet::id)
            .first::<i32>(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        // Create a transaction record for the buyer
        let new_buyer_transaction = NewTransaction {
            user_id: earliest_buy_order.user_id,
            wallet_id: buyer_walletid,
            rwallet_id: buyer_realmoneywallet.id,
            cryptocurrency_id: earliest_buy_order.cryptocurrency_id,
            ttype: "buy".to_string(),
            amount: trade_amount,
            created_on: Some(Utc::now().naive_utc()),
            modified_on: Some(Utc::now().naive_utc()),
            payment_method: "RealMoneyWallet".to_string(),
            payment_amount: trade_amount * sell_order.price,
            payment_status: "completed".to_string(),
        };

        // Insert the buyer transaction record into the database
        diesel::insert_into(transactions::table)
            .values(&new_buyer_transaction)
            .execute(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        // Create a transaction record for the seller
        let new_seller_transaction = NewTransaction {
            user_id: sell_order.user_id,
            wallet_id: seller_walletid,
            rwallet_id: seller_realmoneywallet.id,
            cryptocurrency_id: sell_order.cryptocurrency_id,
            ttype: "sell".to_string(),
            amount: trade_amount,
            created_on: Some(Utc::now().naive_utc()),
            modified_on: Some(Utc::now().naive_utc()),
            payment_method: "CryptoWallet".to_string(),
            payment_amount: trade_amount * sell_order.price,
            payment_status: "completed".to_string(),
        };

        // Insert the seller transaction record into the database
        diesel::insert_into(transactions::table)
            .values(&new_seller_transaction)
            .execute(&mut connection)
            .map_err(|_| Status::InternalServerError)?;

        //if the buy order is complete, then break
        if earliest_buy_order.amount == 0.0{
            break
        }
    }
}
    // Return a success message
    Ok("Orders matched and processed successfully.".to_string())
}
   