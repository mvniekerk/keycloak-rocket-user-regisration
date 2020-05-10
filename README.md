# Keycloak user registration service (done in Rust) with example Expo.io / React Native client

An example Rocket (made with Rust) service that allows users to register with 
a Keycloak service. There is an expo.io example client provided as well.

First the user is asked to provide a cellphone number. An OTP (one time pin) is sent to the 
user's handset. The user is then asked to provide this OTP. This OTP is then verified with
 this service. 
 
 After confirming the OTP, the user is then asked to provide the user's details. This includes
 username, password, firstname, lastname and email. The service will then register the
 user with Keycloak or let the user know what went wrong. 

The service itself uses environmental variables for its configuration. A list can be seen in this document.

The included Dockerfile will compile to an image that can be run on OpenFaas or Knative. The whole
ecosystem of services can be spun up by a simple `docker-compose up` command.

## Prerequisites
* docker and docker-compose
* yarn or npm
* An SMS Portal account or a Twilio account and a Twilio phone number

## Compile and run the service in docker
In the `docker` folder, run `docker-compose up`. The included Dockerfile will compile to
an image that can be run on OpenFaas or Knative. Note the options in the docker-compose.yml file (see section
 "Service Configuration").

You can reach the Keycloak instance on http://localhost:8083 (username admin, password Pa55w0rd).
The registration service can be reached on http://localhost:8082

## Run as an OpenFaas function
You'll need [OpenFaas deployed](https://docs.openfaas.com) and its faas-cli utility installed.

The provided stack.yml file must be edited with your instances of Keycloak, Redis and the sms backend's 
setup (either for Twilio or SMS Portal).
The example config had a Redis instance set up (and subsequent REDIS_HOST variable set up as is) with Helm with:  

```shell script
helm install openfaas-redis stable/redis --namespace openfaas-fn --set usePassword=false --set master.persistence.enabled=false
``` 

To just build it:
```shell script
faas-cli build -f stack.yml
``` 

To deploy it:
```shell script
faas-cli up -f stack.yml
```

## Run the expo.io / React native client
* Install expo.io with `yarn global add expo-cli` or `npm install -g expo-cli`
* In the `user-registration-app` folder, run `yarn` or `npm install`
* Run `expo start`
* Take a photo of the resulting QR-code (iOS) or use the Expo app's QR-code scanner (Android) to run
the app on your device


## Curl examples
* Send the OTP:
```shell script
curl -v -X POST -d '{"number":"<my number>"}' \
-H 'Content-Type: application/json' http://localhost:8082/phone/register
```
The result is the UUID to be used below

* Validate OTP:
```shell script
curl -v -X PUT -d '{"number":"<my number>","otp":"<4 digit OTP>"}' \
-H 'Content-Type: application/json' http://localhost:8082/phone/register/<UUID> 
```

* Do the user registration:
```shell script
curl -v -X POST -d '{"username":"<username>","password":"<password>","first_name":"<First name>","last_name":"<Last name>","email":"<email>"}' \
-H 'Content-Type: application/json' http://localhost:8082/user/register/<UUID>
```

## Service configuration
The docker-compose setup already has these set up (except the SMS_PORTAL_* settings).

| Variable                   | Example                              | Meaning |
| -------------------------- | ------------------------------------ | ----- |
| REDIS_HOST                 | "redis://127.0.0.1/"                 | The Redis host to connect to to do the TTL OTP with |
| SMS_PORTAL_CLIENT_ID       |                                      | SMS portal client ID |
| SMS_PORTAL_SECRET          |                                      | SMS portal secret    |
| TWILIO_ACCOUNT_SSID        |                                      | Twilio account SSID |
| TWILIO_AUTH_TOKEN          |                                      | Twilio authentication token |
| TWILIO_NUMBER_FROM         |                                      | Twilio number you're sending from |
| KC_BASE                    | http://localhost:8083/auth           | Keycloak base URL    |
| KC_REALM                   | my-realm                             | Realm to register user with |
| KC_CLIENT_SECRET           | 8541486f-30a6-4fea-8265-b37410a033ad | Client secret for realm |
| KC_CLIENT_ID               | user-sms-registration                | Client ID |
| KC_SEND_VERIFICATION_EMAIL | false                                | Only set this to true if your Keycloak instance has a correctly set up e-mail credentials |

## Keycloak Client setup
The docker-compose setup has an example of a client (user-sms-registration) set up to do this. In short you
need a server to server setup.

When creating a client for a realm for this service, you'll need:
* Access type to be "Confidential"
* Service Accounts Enabled to be enabled
* On the Service Account Roles, the client should have from the client "realm-management" the role "manage-users".
* KC_CLIENT_SECRET is then from the Credentials tab.


