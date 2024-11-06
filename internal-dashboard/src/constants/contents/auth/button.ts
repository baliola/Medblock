export const ButtonNFID = {
  signIn: {
    label: "Sign In with NFID",
    icon: null,
    onSuccess: {
      redirect: "/",
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