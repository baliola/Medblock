import { assets } from "@/constants/assets";
import { FcGoogle } from "react-icons/fc";
import { GoPasskeyFill } from "react-icons/go";

export const ButtonPasskey = {
  label: 'Continue With Passkey',
  icon: GoPasskeyFill
};

export const ButtonGoogle = {
  label: null,
  icon: FcGoogle
};

export const ButtonEID = {
  icon: assets.eid_logo,
  label: "EID"
}

export const OtherLoginOptions = {
  label: "Other Login Options",
  divider: "OR CONTINUE WITH"
}

export const ButtonNFID = {
  signIn: {
    label: "Sign In",
    icon: null,
    onSuccess: {
      redirect: "/auth/hospital-registration",
      title: "Login Success",
      description: "You have been logged in to the app",
    }
  },
  signOut: {
    label: "Sign Out",
    icon: null,
    onSuccess: {
      redirect: null,
      title: "Logout Success",
      description: "You have been logout from the app",
    }
  }
}