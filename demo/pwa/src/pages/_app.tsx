import '@/globals.css';

import type { AppProps } from 'next/app';
import Head from 'next/head';

import CommonLayout from '@/layouts/CommonLayout';
import { NextPageWithLayout } from '@/types';

export type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout;
};

function MyApp({ Component, pageProps }: AppPropsWithLayout) {
  const getLayout = Component.getLayout ?? CommonLayout;
  return getLayout(
    <>
      <Head>
        <meta name="viewport" content="width=device-width,initial-scale=1" />
        <title>Medblock Passport</title>
        <meta name="description" content="Medblock Passport" />
        <link rel="shortcut icon" href="/logo.png" />
        <link rel="mask-icon" href="/logo.png" color="#FFFFFF" />
        <meta name="theme-color" content="#ffffff" />
        <link rel="apple-touch-icon" href="/logo.png" />
        <link rel="apple-touch-icon" sizes="152x152" href="/logo.png" />
        <link rel="apple-touch-icon" sizes="180x180" href="/logo.png" />
        <link rel="apple-touch-icon" sizes="167x167" href="/logo.png" />
        <link rel="manifest" href="/manifest.json" />
        <meta name="twitter:card" content="summary" />
        <meta name="twitter:url" content="https://medblock.ic" />
        <meta name="twitter:title" content="Medblock Passport" />
        <meta name="twitter:description" content="Medblock Passport" />
        <meta name="twitter:image" content="/logo.png" />
        <meta name="twitter:creator" content="@Baliola" />
        <meta property="og:type" content="website" />
        <meta property="og:title" content="Medblock Passport" />
        <meta property="og:description" content="Medblock Passport" />
        <meta property="og:site_name" content="Medblock Passport" />
        <meta property="og:url" content="https://medblock.ic" />
        <meta property="og:image" content="/logo.png" />

        <link
          rel="apple-touch-startup-image"
          href="/logo.png"
          sizes="2048x2732"
        />
        <link
          rel="apple-touch-startup-image"
          href="/logo.png"
          sizes="1668x2224"
        />
        <link
          rel="apple-touch-startup-image"
          href="/logo.png"
          sizes="1536x2048"
        />
        <link
          rel="apple-touch-startup-image"
          href="/logo.png"
          sizes="1125x2436"
        />
        <link
          rel="apple-touch-startup-image"
          href="/logo.png"
          sizes="1242x2208"
        />
        <link
          rel="apple-touch-startup-image"
          href="/logo.png"
          sizes="750x1334"
        />
        <link
          rel="apple-touch-startup-image"
          href="/logo.png"
          sizes="640x1136"
        />
      </Head>
      <Component {...pageProps} />
    </>,
  );
}

export default MyApp;
