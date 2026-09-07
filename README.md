# Apricot

Simulated Bank RESTful-API used for CTF hosted by Loyola University Chicago's
[Women in Cybersecurity (WiCys)](https://wicysluc.carrd.co/?utm_source=ig&utm_medium=social&utm_content=link_in_bio) and 
[Cybersecurity Competition Group (7968)](https://7968.org/). This repository was written by [finch](https://westwardfishdme.github.io),
the Event Coordinator of 7968, and pentested by __pentesters will be inserted here.__ []()[]()[]()[]() 

## Important Note
**PLEASE READ**: This program is an intentionally vulnerable REST-API that was
made for a CTF challenge event hosted at Loyola University. While there is real
built-in functionality that simulates a real banking system, this program 
**SHOULD not be used as a real system for banking systems or account setup.**

With that said, please enjoy the challenge. If you have issues, please ask the staff hosting 
the event for help. 

# Documentation
*Apricot* is a Core Banking System based on a RESTful-API that handles transactions, authentication, and account 
management for banks. This project is licensed under the GPL-3.0. The service is intended to be used by banking 
firms within the United States; Ensure that your bank institution is compliant with the following Risk and 
Compliance Standards before using our software:

- **Know Your Customer (KYC) compliance**

## How it works:

At start, *Apricot* creates two databases:

- Authentication Database
- User/Account Database

These databases use the [sled](https://sled.rs/) embedded database system to locally host both the authentication and user information.

## Authentication
*Apricot* authenticates the users by username and password, which is stored as a **SHA-256** hash along with a cryptographically secure
randomly generated salt appended to the end of each password. Each salt is 16 characters long.

# Interacting with Apricot
At its core, *Apricot* uses the [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) model to 
interact with the underlying databases. There are 4 routes that you may use to interact with the actual CBS system.:

- `/login`
- `/account`
- `/account/create`

The entire system uses `application/json` headers to interact with the underlying system. An example of this
using the curl command will be given below. 

You may send the following requests to the routes:

### /login
|HTTP request type| JSON fields                            | Description                   |
|-----------------|----------------------------------------|-------------------------------|
| `GET`           | "username", "password"                 | Gets the user's login details |
| `PATCH`         | "username", "password", "new_password" | Updates the user's password   |

Example JSON header:

```json
{
  "username":"username_goes_here",
  "password":"plaintext_password_goes_here",
  "new_password":"new_password_goes_here"
}
```

#### GET
An HTTP `GET` request sent to `/login` will return the information of the user that is stored on the authentication 
database. It returns all the information necessary for authentication including a salted SHA-256 hash of the user's 
password.

#### PATCH
An HTTP `PATCH` request sent to `/login` will send a request to change the password to the authentication server.
If the password matches the given password inside of the authentication database, it will return the following JSON
Form:

```json
{ "update_password":"success" }
```

### /account 

|HTTP request type| JSON fields                                   | Description                     |
|-----------------|-----------------------------------------------|---------------------------------|
| `GET`           | "username", "password"                        | Gets the user's account details |
| `POST`          | "username", "password", "recipient", "amount" | Performs a transaction          |

#### GET
An HTTP `GET` request sent to `/account` will return the user's account details should the authentication process
be successful.

#### POST
An HTTP `POST` request sent to `/account` will perform a transaction to send money to another valid user should they
exist in the account database.


### /account/create
|HTTP request type| JSON fields                                                       | Description                            |
|-----------------|-------------------------------------------------------------------|----------------------------------------|
| `POST`          | "username", "password", "first_name", "last_name", "init_balance" | creates a new account in  the database |

#### POST
An HTTP `POST` request sent to `/account/create` will create a new user in both the authentication and account databases.
