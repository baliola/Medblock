import Head from 'next/head';
import React from 'react';

const Header = () => {
  return (
    <Head>
      <meta name="viewport" content="width=device-width,initial-scale=1" />
      <title>Medblock</title>
      <meta name="description" content="Medblock" />
      <link rel="shortcut icon" href="/logo.svg" />
      <link rel="mask-icon" href="/logo.svg" color="#FFFFFF" />
      <meta name="theme-color" content="#ffffff" />
      <meta name="twitter:card" content="summary" />
      <meta name="twitter:url" content="https://medblock.ic" />
      <meta name="twitter:title" content="Medblock" />
      <meta name="twitter:description" content="Medblock" />
      <meta name="twitter:image" content="/logo.svg" />
      <meta name="twitter:creator" content="@Baliola" />
      <meta property="og:type" content="website" />
      <meta property="og:title" content="Medblock" />
      <meta property="og:description" content="Medblock" />
      <meta property="og:site_name" content="Medblock" />
      <meta property="og:url" content="https://medblock.ic" />
      <meta property="og:image" content="/logo.svg" />

     
    </Head>
  );
};

export default Header;
