import { assets } from "@/constants/assets";

export const kycWaiting = {
  title: "Your account is currently undergoing KYC verification",
  description: "Please contact Customer Service for the solution for your account.",
  image: assets.illustration_female_doctor,
  button: {
    label: "Back",
    redirect: "/auth/unverified"
  }
};