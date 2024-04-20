import '@/globals.css';
import 'react-toastify/dist/ReactToastify.css';

import type { AppProps } from 'next/app';
import { useEffect, useState } from 'react';
import { ToastContainer } from 'react-toastify';

import Header from '@/components/Head/Header';
import { AgentProvider } from '@/config/agent';
import { NFIDS } from '@/interface/nfid.interface';
import CommonLayout from '@/layouts/CommonLayout';
import { HomeLayout } from '@/layouts/HomeLayout/HomeLayout';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';
import SplashScreen from '@/scenes/Splash/Splash.scene';
import { NextPageWithLayout } from '@/types';

export type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout;
};

function MyApp({ Component, pageProps }: AppPropsWithLayout) {
  const [windowLoaded, setWindowLoaded] = useState<boolean>(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setWindowLoaded(true);
    }
  }, []);

  const getLayout = Component.getLayout ?? CommonLayout;
  const getHomeLayout = Component.getLayout ?? HomeLayout;
  const getScaffold = Component.getLayout ?? Scaffold;

  const initNFID = async () => {
    await NFIDS();
  };

  useEffect(() => {
    if (windowLoaded) {
      initNFID();
    }
  }, [windowLoaded]);

  if (!windowLoaded) {
    return (
      <>
        <Header />
        <SplashScreen />;
      </>
    );
  }

  if (Component.ScaffoldLayout) {
    return (
      <AgentProvider>
        <div>
          <Header />
          <ToastContainer
            position="top-center"
            hideProgressBar={false}
            pauseOnFocusLoss={false}
            theme="light"
            autoClose={true}
          />
          {getScaffold(<Component {...pageProps} />)}
        </div>
      </AgentProvider>
    );
  } else if (Component.HomeLayout) {
    return (
      <AgentProvider>
        <>
          <Header />
          <ToastContainer
            position="top-center"
            hideProgressBar={false}
            pauseOnFocusLoss={false}
            theme="light"
            autoClose={true}
          />
          {getHomeLayout(<Component {...pageProps} />)}
        </>
      </AgentProvider>
    );
  }

  return getLayout(
    <AgentProvider>
      <>
        <Header />
        <ToastContainer
          position="top-center"
          hideProgressBar={false}
          pauseOnFocusLoss={false}
          theme="light"
          autoClose={true}
        />
        {getLayout(<Component {...pageProps} />)}
      </>
    </AgentProvider>,
  );
}

export default MyApp;
