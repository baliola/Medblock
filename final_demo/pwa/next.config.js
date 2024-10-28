const envList = require("dotenv").config({ path: "./.env" }).parsed || {}
const { version } = require("./package.json")

const { Crypto } = require("@peculiar/webcrypto");

const crypto = new Crypto();
Object.defineProperty(global, "crypto", {
  value: crypto,
});

const withPWA = require("next-pwa")({
  dest: "public"
});

envList.NEXT_PUBLIC_IC_HOST =
  envList.DFX_NETWORK === "ic"
  ? "https://icp-api.io"
  : "http://127.0.0.1:4943"

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
        source: "/",
        destination: "/home",
        permanent: true
      }
    ]
  },
};

module.exports = withPWA(nextConfig);