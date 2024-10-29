const APP_NAME = 'Medblock';
const APP_LOGO = 'https://nfid.one/icons/favicon-96x96.png';
const CONFIG_QUERY = `?applicationName=${APP_NAME}&applicationLogo=${APP_LOGO}`;

const identityProvider = `https://nfid.one/authenticate${CONFIG_QUERY}`;

const dfxNetwork =
  process.env.NODE_ENV === 'development' ? 'local' : 'ic';

/**
 * 
 * CHANGE TO identityProvider
 * if production
 * if local just stay in localhost
 * 
 */
export const loginHost =
  dfxNetwork === 'ic'
    ? identityProvider
    : process.env.NEXT_PUBLIC_II;
