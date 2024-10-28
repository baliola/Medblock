import axios from 'axios';

export const APIKycMedblock = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL,
  headers: {
    'Content-Type': 'application/json',
    'api_key': process.env.NEXT_PUBLIC_API_KEY
  }
});
