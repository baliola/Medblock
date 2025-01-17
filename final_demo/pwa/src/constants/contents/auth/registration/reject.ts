import { assets } from "@/constants/assets";

export const kycRejected = {
  title: "We Can’t Verified You",
  description: "Please resubmit your data",
  image: assets.illustration_female_doctor_reject,
  button: {
    label: "Resubmit Your Data",
    redirect: "/auth/unverified/registration"
  }
};