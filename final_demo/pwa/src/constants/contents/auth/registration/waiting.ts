import { assets } from "@/constants/assets";
import { redirect } from "next/dist/server/api-utils";

export const kycWaiting = {
  title: "Your account is currently undergoing KYC verification",
  description: "Please contact Customer Service for the solution for your account.",
  image: assets.illustration_female_doctor,
  button: {
    refresh: {
      label: "Refresh Data",
      redirect: "/home"
    },
    back: {
      label: "Back",
      redirect: "/auth/unverified"
    }
  }
};