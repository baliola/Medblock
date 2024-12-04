import { NextAuthOptions, SessionStrategy } from 'next-auth';
import CredentialsProvider from 'next-auth/providers/credentials';

export const authOptions: NextAuthOptions = {
  providers: [
    CredentialsProvider({
      name: 'Credentials',
      credentials: {
        email: { type: 'email', label: 'Email' },
        password: { type: 'password', label: 'Password' }
      },

      async authorize(credentials) {
        if (!credentials) {
          throw new Error('No credentials provided');
        }

        const { email, password } = credentials;
        
        const response = await fetch(
          `${process.env.NEXT_PUBLIC_API_URL}/auth/login`,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password })
          }
        );

        if (response.ok) {
          const user = await response.json();
          const { username, email, accessToken } = user?.data;
          return {
            username: username,
            email: email,
            accessToken: accessToken,
            id: user.id,
          }
        }
        throw new Error("Login failed");
      }
    })
  ],

  secret: process.env.NEXTAUTH_SECRET,
  session: {
    strategy: "jwt" as SessionStrategy,
    maxAge: 60 * 60,
  },
  jwt: {
    maxAge: 60 * 60,
  },

  callbacks: {
    async session({ session, token }) {
      if (session?.user) {
        session.user.accessToken = token.accessToken;
        session.user.email = token.email;
        session.user.username = token.username;
      }
      return session;
    },

    async jwt({ token, user }) {
      if (user) {
        token.accessToken = user.accessToken;
        token.email = user.email;
        token.username = user.username;
      }
      return token;
    }
  },
  pages: {
    signIn: "/auth/login",
  },
};