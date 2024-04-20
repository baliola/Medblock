import { SignIdentity } from '@dfinity/agent';
import { AuthClientStorage, IdleOptions } from '@dfinity/auth-client';
import { NFID } from '@nfid/embed';

type NFIDConfig = {
  origin?: string; // default is "https://nfid.one"
  application?: {
    // your application details to display in the NFID iframe
    name?: string; // your app name user can recognize
    logo?: string; // your app logo user can recognize
  };
  identity?: SignIdentity;
  storage?: AuthClientStorage;
  keyType?: 'ECDSA' | 'Ed25519'; // default is "ECDSA"
  idleOptions?: IdleOptions;
};

export async function NFIDS() {
  const nfid = await NFID.init({
    application: {
      name: 'MEDBLOCK',
      logo: 'https://dev.nfid.one/static/media/id.300eb72f3335b50f5653a7d6ad5467b3.svg',
    },
  } as NFIDConfig);
  return nfid;
}
