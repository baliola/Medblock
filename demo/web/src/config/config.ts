export const host =
  process.env.DFX_NETWORK === 'ic'
    ? 'https://identity.ic0.app'
    : `http://bw4dl-smaaa-aaaaa-qaacq-cai.localhost:4943`;
