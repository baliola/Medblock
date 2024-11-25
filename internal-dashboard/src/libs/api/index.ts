import axios from 'axios';
import { getSession, signOut } from 'next-auth/react';

export const APIKycMedblock = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL,
  headers: {
    'Content-Type': 'application/json'
  }
});

APIKycMedblock.interceptors.request.use(async (config) => {
  const session = await getSession();
  const token = session?.user.accessToken;

  if (token) {
    config.headers.Authorization = `Bearer ${token}`; 
  }

  return config;
}, (error) => {
  return Promise.reject(error);
});

APIKycMedblock.interceptors.response.use(
  (response) => {
    return response;
  },
  (error) => {
    if (
      error.response &&
      error.response.status === 401
    ) {
      signOut({
        callbackUrl: '/auth/login',
        redirect: true
      });
    }
    return Promise.reject(error);
  }
);