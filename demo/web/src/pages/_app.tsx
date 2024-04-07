import '@/globals.css';
import 'react-toastify/dist/ReactToastify.css';

import type { AppProps } from 'next/app';

import CommonLayout from '@/layouts/CommonLayout';
import { NextPageWithLayout } from '@/types';
import EmptyLayout from '@/layouts/EmptyLayout';
import { ToastContainer } from 'react-toastify';

export type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout;
};

function MyApp({ Component, pageProps }: AppPropsWithLayout) {
  const getLayout = Component.getLayout ?? CommonLayout;
  const getEmptyLayout = Component.getLayout ?? EmptyLayout;
  if (Component.disableLayout) {
    return <> {getEmptyLayout(<Component {...pageProps} />)}</>;
  }
  return (
    <>
      <ToastContainer
        position="top-center"
        hideProgressBar={false}
        pauseOnFocusLoss={false}
        theme="light"
      />
      {getLayout(<Component {...pageProps} />)}
    </>
  );
}

export default MyApp;
