const envList = require("dotenv").config({ path: "./.env" }).parsed || {}

const { version } = require("./package.json")

envList.NEXT_PUBLIC_IC_HOST =
  envList.DFX_NETWORK === "ic"
    ? "https://icp-api.io"
    : "http://localhost:4943"

envList.NEXT_PUBLIC_VERSION = version

/** @type {import('next').NextConfig} */
const nextConfig = {
  env: envList,
  redirects: async () => {
    return [
      {
        source: "/api",
        destination: envList.NEXT_PUBLIC_IC_HOST,
        permanent: true
      },
      {
        source: '/',
        destination: '/dashboard/overview',
        permanent: true
      }
    ]
  },
  compiler: {
    removeConsole: process.env.NODE_ENV === 'production' ? true : false
  }
};

module.exports = nextConfig;