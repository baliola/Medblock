import axios from 'axios'

export const MedBlockAPI = axios.create({
  baseURL: `${process.env.NEXT_PUBLIC_DEV_API_URL}/dev`,
  headers: {
    'Content-Type': 'application/json'
  }
});
