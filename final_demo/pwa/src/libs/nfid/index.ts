
import { SignIdentity } from "@dfinity/agent";
import { AuthClientStorage } from "@dfinity/auth-client";
import { NFID } from "@nfid/embed";

type IdleOptions = {
  onIdle?: () => unknown;                   // callback after the user has gone idle
  idleTimeout?: number;                     // timeout in ms, default is 600000 (10 minutes)
  captureScroll?: boolean;                  // capture scroll events
  scrollDebounce?: number;                  // scroll debounce time in ms, default is 100
  disableIdle?: boolean;                    // disables idle functionality
  disableDefaultIdleCallback?: boolean;     // disables default idle behavior - call logout & reload window
}

type NFIDConfig = {
  origin?: string;                // default is "https://nfid.one"
  application?: {                 // your application details to display in the NFID iframe
    name?: string;                // your app name user can recognize
    logo?: string;                // your app logo user can recognize
  };
  identity?: SignIdentity;
  storage?: AuthClientStorage;
  keyType?: "ECDSA" | "Ed25519"   // default is "ECDSA"
  idleOptions?: IdleOptions;
};

export default async function NFIDInit() {
  const init = await NFID.init({
    application: {
      name: "MEDBLOCK",
      logo: "https://dev.nfid.one/static/media/id.300eb72f3335b50f5653a7d6ad5467b3.svg",
    },
  } as NFIDConfig);
  
  return init;
}