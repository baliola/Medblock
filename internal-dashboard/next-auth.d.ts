import NextAuth from "next-auth";

declare module "next-auth" {
  /**
   * Returned by `useSession`, `getSession` and received as a prop on the `SessionProvider` React Context
   */
  interface Session {
    user: {
      email: string,
      username: string,
      accessToken: string,
    } & DefaultSession
  }

  interface User extends DefaultUser {
    email: string,
    accessToken: string,
    username: string
  }
}

declare module "next-auth/jwt" {
  interface JWT extends DefaultJWT {
    email: string,
    accessToken: string,
    username: string
  }
}