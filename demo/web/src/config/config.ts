import { HttpAgent, Identity } from '@dfinity/agent';

export const dfxNetwork =
  process.env.NODE_ENV === 'development' ? 'local' : 'ic';
export const host =
  process.env.DFX_NETWORK === 'ic'
    ? 'https://identity.ic0.app'
    : `http://bw4dl-smaaa-aaaaa-qaacq-cai.localhost:4943`;

export const AppAgent = (identity: Identity | null) => {
  console.log('identity from config', identity);
  let newAgents = new HttpAgent({ host, identity: identity as Identity });
  return newAgents;
};
