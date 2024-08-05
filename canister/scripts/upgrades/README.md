# Overview
This contains the scripts related to upgrading canisters in different environment, to use them simply go to the scripts directory and execute the script. For example upgrading the patient registry on staging environment would be :

go to the directory
```bash
cd staging
```

execute the script
```bash
./upgrade-patient.sh
```
> this will rebuild the canister that you want to upgrade, inject all the necessary candid info and upgrade it.
