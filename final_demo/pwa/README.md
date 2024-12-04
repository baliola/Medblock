# Medblock WebApp
## Overview
The MedBlock WebApp repository contains the primary user interface for Medblock, including both the PWA (Patient) and the WebApp (Hospital).

## Prerequisites
Before starting development or deployment, ensure you have the following:

- [Medblock canister](https://github.com/baliola/Medblock) is set up.
- `npm` and `yarn` are installed.

## Deployment
To deploy the application, you will need to set up the PWA.

1. Clone the repository.
```
git clone https://github.com/baliola/MedBlock-WebApp.git
```
2. Start the Medblock canister.
3. Checkout branches: `feat/pwa-dev`
4. Install dependencies:
```bash
yarn install
```
5. Development:
```
yarn dev
```

6. Production:
```
yarn build
yarn start
```