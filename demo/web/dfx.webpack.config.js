const path = require("path");

let localCanisters, prodCanisters, canisters;

function initCanisterIds() {
  try {
    localCanisters = require(path.resolve(".dfx", "ic", "canister_ids.json"));
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production");
  }
  try {
    prodCanisters = require(path.resolve("canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }

  const network = process.env.DFX_NETWORK || process.env.NODE_ENV === "ic";
  console.info(`initCanisterIds: network=${network}`);

  canisters = network === "local" ? localCanisters : prodCanisters;

  for (const canister in canisters) {
    console.info(`initCanisterIds: canister=${canister.toUpperCase()}`);

    process.env[`NEXT_PUBLIC_${canister.toUpperCase()}_CANISTER_ID`] =
      canisters[canister][network];
  }
}

module.exports = {
  initCanisterIds: initCanisterIds,
};
