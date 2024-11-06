# Medblock KYC

## Overview

The MedBlock KYC is a web application that allows users to verify their identity by uploading their documents.

## Prerequisites

Before starting development or deployment, ensure you have the following:

1. Create .env file in the root directory of the project and add the following:

```
NEXT_PUBLIC_API_URL="PUT_THE_API_ENDPOINT_HERE"
NEXTAUTH_SECRET="PLEASE_GENERATE_SECRET_KEY_FOR_NEXTAUTH_HERE"
NEXTAUTH_URL="PUT_THE_DEPLOYED_URL_HERE_IF_IN_LOCAL_USE_LOCALHOST"

```

2. NEXT_PUBLIC_API_URL: The API endpoint for the backend.

3. NEXTAUTH_SECRET: A secret key for NextAuth.

- To generate a secret key, run the following command in the terminal:

```
node -e "console.log(require('crypto').randomBytes(64).toString('hex'))"
```

- Copy the generated key and paste it in the .env file as the value for NEXTAUTH_SECRET.

4. NEXTAUTH_URL: The URL where the application is deployed.

- If you are running the application locally, set the value to `http://localhost:3000`.
- If you are deploying the application, set the value to the deployed URL.

To deploy the application,

1. Clone the repository.

```
git clone https://github.com/baliola/MedBlock-WebAdmin.git
```

2. run the following command to install the dependencies:

```bash
yarn install
```

3. run the following command to build the application:

```bash
yarn build
```

5. Development:

- run the following command to start the development server:

```bash
yarn dev
```

6. Production:

- run the following command to start the production server:

```bash
yarn build
yarn start
```
