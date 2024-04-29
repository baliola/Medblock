import { HttpAgent, Identity } from '@dfinity/agent';

const APP_NAME = 'Medblock';
const APP_LOGO = 'https://nfid.one/icons/favicon-96x96.png';
const CONFIG_QUERY = `?applicationName=${APP_NAME}&applicationLogo=${APP_LOGO}`;

const identityProvider = `https://nfid.one/authenticate${CONFIG_QUERY}`;

export const dfxNetwork =
  process.env.NODE_ENV === 'development' ? 'local' : 'ic';

export const loginHost =
  process.env.DFX_NETWORK === 'ic'
    ? identityProvider
    : `http://bw4dl-smaaa-aaaaa-qaacq-cai.localhost:4943/`;

export const host =
  process.env.DFX_NETWORK === 'ic'
    ? identityProvider
    : `http://bw4dl-smaaa-aaaaa-qaacq-cai.localhost:4943/`;

export const AppAgent = (identity: Identity | null) => {
  // console.log('identity from config', identity);
  let newAgents = new HttpAgent({ host, identity: identity as Identity });
  return newAgents;
};
