/** @type {import('next').NextConfig} */
const nextConfig = {
  poweredByHeader: false,
  reactStrictMode: true,
  swcMinify: true,
  output: 'export',

  images: {
    unoptimized: true,
  },
};

module.exports = nextConfig;
