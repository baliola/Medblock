'use-client';
import '@/globals.css';
import 'react-toastify/dist/ReactToastify.css';

import type { AppProps } from 'next/app';
import { InternetIdentityProvider } from 'ic-use-internet-identity';

import CommonLayout from '@/layouts/CommonLayout';
import { NextPageWithLayout } from '@/types';
import EmptyLayout from '@/layouts/EmptyLayout';
import { ToastContainer } from 'react-toastify';
import PatientLayout from '@/layouts/PatientLayout';
import { NFIDS } from '@/interface/nfid.interface';
import { useEffect, useState } from 'react';
import { AgentProvider } from '@/config/agent';
import SplashScreen from '@/scenes/Splash/SplashScreen';

export type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout;
};

function MyApp({ Component, pageProps }: AppPropsWithLayout) {
  const [windowLoaded, setWindowLoaded] = useState(false);

  useEffect(() => {
    // Check if window is loaded
    if (typeof window !== 'undefined') {
      setWindowLoaded(true);
    }
  }, []);

  const getLayout = Component.getLayout ?? CommonLayout;
  const getEmptyLayout = Component.getLayout ?? EmptyLayout;
  const getPatientLayout = Component.getLayout ?? PatientLayout;

  const initNFID = async () => {
    await NFIDS;
  };

  useEffect(() => {
    if (windowLoaded) {
      initNFID();
    }
  }, [windowLoaded]);

  if (!windowLoaded) {
    // Render a loading state while waiting for window to load
    return <SplashScreen />;
  }

  if (Component.disableLayout) {
    return (
      <div suppressHydrationWarning>
        <AgentProvider>
          {getEmptyLayout(<Component {...pageProps} />)}
        </AgentProvider>
      </div>
    );
  } else if (Component.patientLayout) {
    return (
      <AgentProvider>
        {getPatientLayout(<Component {...pageProps} />)}
      </AgentProvider>
    );
  }
  return (
    <AgentProvider>
      <ToastContainer
        position="top-center"
        hideProgressBar={false}
        pauseOnFocusLoss={false}
        theme="light"
      />
      {getLayout(<Component {...pageProps} />)}
    </AgentProvider>
  );
}

export default MyApp;
