import { assets } from "@/constants/assets";

export const registrationSuccess = {
  header: {
    image: assets.registration_success,
    alt: 'success illustration',
    title: "Your account is verified!",
    description: "Now you can share and control your medical record to hospital."
  },
  button: {
    label: "Continue",
    redirect: "/home"
  }
}