Introduction
This report presents the final project report for the Rust Cryptocurrency Trading
Platform developed by Muhammad Hamza Wala Qadr, Muhammad Mustafa, and
Ibrahim Abdul Rehman. The project aims to provide a comprehensive
cryptocurrency trading platform with core functionalities such as user
registration, authentication, order placement, a trading engine for matching buy
and sell orders, order book and trade history maintenance, and account balance
and transaction listing for users.
About the project
→ A crypto trading or exchange platform provides the necessary infrastructure
and functionality to facilitate the trading of cryptocurrencies.
→ The trade engine serves as the main processing hub, responsible for
executing buy and sell orders, matching buy and sell orders from users, and
updating the order book in real-time.
→ It incorporates features such as cryptocurrency wallets and real money
wallets and allows users to register, log in, and conduct various transactions.
Core functionalities
1. User registration
2. User authentication and login
3. Order placement (buy or sell)
4. Trading engine (matching buy and sell orders)
5. Order book and trade history maintained for every user
6. Users can easily view account balance and list of transactions

Technologies used
The Rust Cryptocurrency Trading Platform leveraged a range of technologies to
ensure its functionality, performance, and security:
1. Rust with Rocket and Diesel: Rust is a powerful programming language known
for its performance, safety, and concurrency features. Rocket is a web framework
designed for Rust, facilitating the development of web applications. Diesel is an
ORM (Object-Relational Mapping) library for Rust, enabling convenient database
interactions.
2. PostgreSQL: PostgreSQL is an open-source relational database management
system (RDBMS) used to store and manage the project's data. It provides robust
data storage and retrieval capabilities.
3. Stripe API: The Stripe API integration enables seamless payment processing
and credit card transactions within the platform. It allows users to deposit money
from their credit cards into their real money wallets.

Tools used
The development of the Rust Cryptocurrency Trading Platform made use of the
following tools:
1. Visual Studio Code (VS Code): VS Code served as the primary integrated
development environment (IDE) for writing, editing, and debugging Rust code.
It offers various features that enhance development productivity.
2. PgAdmin: PgAdmin is a widely used PostgreSQL administration and
management tool. It provided a user-friendly interface for managing the project's
PostgreSQL database, allowing efficient database operations and maintenance.
3. Postman: Postman, a popular API testing tool, played a crucial role in testing
and validating the functionality of the implemented APIs. It enabled
comprehensive testing of the endpoints and facilitated the detection and
resolution of potential issues.

Libraries used
To enhance functionality and efficiency, several Rust libraries were utilized in the
development of the Rust Cryptocurrency Trading Platform:
1. Rust serde: The Rust serde library provided support for JSON serialization and
deserialization. It allowed easy conversion between Rust structs and JSON data,
facilitating data exchange with external systems and APIs.
2. Rust chrono: The Rust chrono library proved essential for handling date and
time-related operations within the platform. It offered convenient and reliable
functionality for working with timestamps and time-based calculations.
3. Rust diesel: Diesel, a powerful ORM library for Rust, simplified the interaction
with the PostgreSQL database. It provided an intuitive query API, enabling
seamless database operations such as CRUD (Create, Read, Update, Delete)
functionality.
4. Rust dotenv: The Rust dotenv library provided a convenient mechanism for
loading environmental variables from a .env file. It enhanced configuration
management by separating sensitive information from the source code.
5. Rust rocket_contrib: The rocket_contrib crate extended the capabilities of the
Rocket web framework, providing additional functionalities and utilities. It
streamlined the development process and contributed to the efficiency of the
platform.
6. Rust async-stripe: The Rust async-stripe library provided Rust bindings to the
Stripe API. It enabled seamless integration with the payment processing
functionality of the platform, allowing secure and reliable credit card
transactions.

Database tables
The Rust Cryptocurrency Trading Platform utilized a set of well-structured
database tables to store and manage different aspects of the system:
1. User: The User table stored details of registered users, including their names,
email addresses, and other relevant information.
2. Wallet: The Wallet table stored the cryptocurrency balance for each user. It
associated a user with their respective cryptocurrency holdings.
3. Cryptocurrency: The Cryptocurrency table maintained information about
various cryptocurrencies, including their symbols and other relevant details.
4. Real Money Wallet: The Real Money Wallet table stored real currency balances
(e.g., USD, EUR) for users. It facilitated the management of fiat currency
transactions within the platform.
5. Transaction: The Transaction table stored information related to different types
of transactions occurring within the platform. This included buy and sell
transactions, deposits, and other financial operations.
6. Order: The Order table stored information about buy and sell orders created
by users. It maintained data such as the user ID, order type (buy or sell),
cryptocurrency ID, order amount, price, and status.
7. Trade: The Trade table stored information about trades that occurred between
users. It captured details such as the buyer ID, seller ID, cryptocurrency ID, trade
amount, price, and relevant timestamps.

List of APIs we created
1. CRUD APIs for all 7 tables
a. Create (Post) API adds an entry to a table
b. Read (Get) API fetches an entry for a provided user_id
c. Read All (Get) API fetches all entries from a table
d. Update (Put) API updates an entry from a table provided a user_id
e. Delete (Delete) API removes an entry from a table provided a user_id
2. Register User API:
→ This API first establishes a connection with the database.
→ This API will then register a new user and create a new wallet for them. The
user's username, password, and email address are required. The currency for
the wallet is set to USD by default, but you can change this to any currency you
want. The initial balance for the wallet is set to 0.0 by default, but you can
change this to any amount you want.
3. User Summary API:
→ This API calls 2 functions that access the Real Money and Transactions tables
to fetch all the transactions and real money wallet balance for a provided user
id and return them.
→ The returned information from these functions is formatted and returned as
a JSON object by the API.
4. Fetch Symbol Price API:
→ This API fetches the price of a cryptocurrency symbol from the Binance API.
The function takes a symbol as input and returns an Option of a f64 value,
representing the price of the symbol.
→ The function first constructs a URL for the Binance API endpoint that returns
the price of the symbol. It then creates a reqwest::Client object and uses it to
send a GET request to the URL. The function then checks the status code of the
response and returns None if the status code is not successful. If the status code
is successful, the function parses the response body as JSON and returns the
price of the symbol.
5. Login API:
→ This API logs a user in. The function takes a LoginData struct as input and
returns a Result of a Status value, representing the success or failure of the login
attempt.
→ The function first establishes a connection to the database. It then uses the
user_details table to find the user with the username provided in the LoginData
struct. If the user is found, the function checks if the password provided in the
LoginData struct matches the password stored in the database. If the passwords
match, the function returns Status::Ok. Otherwise, the function returns
Status::Unauthorized. If the user is not found, the function returns
Status::NotFound.
6. Update Crypto Prices API:
→ This API updates the prices of cryptocurrencies. The function returns a String
representing the success or failure of the update.
→ The function first creates a vector of symbols. It then establishes a connection
to the database. It then iterates over the symbols vector and calls the
fetch_symbol_price() function to fetch the price of the cryptocurrency
represented by the symbol. If the fetch_symbol_price() function returns Some
for the price, the function creates a new NewCrypto struct with the symbol,
price, and creation and modification timestamps. It then inserts the NewCrypto
struct into the crypto table in the database. If the fetch_symbol_price() function
returns None for the price, the function prints an error message.
7. Withdraw from Wallet API:
→ This API allows a user to withdraw money from their real money wallet
→ The code first establishes a connection to the database. Then, it retrieves the
user's real money wallet from the database. Next, it checks if the withdrawal
amount is valid. If the withdrawal amount is greater than the user's balance, the
code returns an error. Otherwise, the code creates a new transaction record for
the withdrawal. The code then updates the user's real money wallet balance and
inserts the new transaction record into the database. Finally, the code returns a
Created response with the new transaction record.
8. Deposit into Wallet API:
→ This API allows a user to deposit money from their credit card to their real
money wallet using a Stripe API
→ It first creates a NewTransaction object and stores the information passed into
the function from new_t2 JSON object. It then creates a payment intent using
the Stripe API and a PaymentMethod object that contains the credit card details
and deposit amount details.
→ Once the payment intent processes and is confirmed, the user’s real money
wallet balance is updated
9. Get Order History API:
→ this API fetches all the “closed”(completed) orders of a user
→ The code first establishes a connection to the database. Then, it uses the
orders table from the web::schema::orders schema to filter for orders that
belong to the user with the ID _user_id and that have a status of “closed”. Finally,
it loads the results into a vector of Order structs and returns them as JSON.
10. Get Current Orders API:
→ this API fetches all the “open”(ongoing) orders of a user
→ The code first establishes a connection to the database. Then, it uses the
orders table from the web::schema::orders schema to filter for orders that
belong to the user with the ID _user_id and that have a status of “open”. Finally,
it loads the results into a vector of Order structs and returns them as JSON.
11. Matching Orders API:
→ This API implements the main trading engine of our program
→ It matches the earliest open buy order with an open sell order and creates a
trade. Also updates all the tables involved in the trade for both buyer and seller.
→ It first creates the earliest_buy_order object and fills it by retrieving the
earliest open buy order from the Orders table. It then creates a
matching_sell_orders object and populates it by retrieving all open sell orders
with price <= buy order's price and a matching cryptocurrency_id.
→ The API then matches and processes the orders, creates a new trade record,
and inserts the trade record into the database. It also updates the status of the
buy and sell orders to "closed" or mark them as processed.
→ Next, the API deducts money from the buyer's real money wallet and
increments the money in the seller's real money wallet. If the buyer has a wallet
for the cryptocurrency, their wallet balance is updated. If not, a new wallet is
created for the buyer in the Wallet table.
→ The whole previous step is repeated for the seller as well (except that the
money is incremented).
→ Finally, the wallet IDs of the buyer and seller are retrieved. A new transaction
record is created for the buyer and is inserted into the Transactions table.
Similarly, a new transaction record is created for the seller and is inserted into
the Transactions table.
→ A success message is returned.

Flow of the program
The Rust Cryptocurrency Trading Platform follows the following flow:
1. User Registration: Users can register by providing their details, including
username, password, and email address.
2. User Authentication and Login: Registered users can authenticate themselves
and log into the platform using their credentials.
3. Fetch Symbol Price: Users can view the latest prices of various
cryptocurrencies using the Fetch Symbol Price API, which retrieves the prices
from the Binance API.
4. Deposit into Wallet: Users can deposit money from their credit cards into their
real money wallets, enabling them to engage in trading and transactions within
the platform.
5. Order Placement: Users can create "buy" or "sell" orders to trade
cryptocurrencies. These orders are stored in the Order table.
6. Account Balance and Transaction Listing: Users can view their account balance
and transaction history, including buy and sell transactions, deposits, and other
financial operations.
7. Trading Cryptocurrencies: The Matching Orders API serves as the trading
engine of the platform. It matches the earliest open buy order with a
corresponding open sell order, creating a trade. It updates the relevant tables,
modifies the order status, deducts or increments money in wallets, and updates
cryptocurrency balances in user wallets.
Overall, the Cryptocurrency Trading Platform provides a comprehensive and
secure environment for users to engage in cryptocurrency trading while
ensuring efficient order matching and transparent transaction tracking.

Conclusion
The Rust Cryptocurrency Trading Platform developed by Muhammad Hamza
Wala Qadr, Muhammad Mustafa, and Ibrahim Abdul Rehman offers a robust and
feature-rich solution for cryptocurrency trading. Leveraging Rust, Rocket, and
Diesel, the platform provides essential functionalities such as user registration,
authentication, order placement, trading engine, order book management, and
transaction tracking.
By utilizing PostgreSQL for data storage, integrating with the Stripe API for
secure payment processing, and incorporating various libraries, the platform
ensures efficient performance, reliability, and data integrity.
With its user-friendly APIs, seamless trading experience, and comprehensive
transaction management, the Rust Cryptocurrency Trading Platform presents a
compelling solution for cryptocurrency enthusiasts and traders.
