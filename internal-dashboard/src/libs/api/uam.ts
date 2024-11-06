import { APIKycMedblock } from "@/libs/api";

interface GetUamProps {
  page: string;
  limit: string;
};

export type Verification = "pending" | "accepted" | "rejected";

interface UAM {
  data: {
    fullName: string;
    nikHash: string;
    verification: Verification;
  }[];
  pagination: {
    perPage: number;
    currentPage: number;
    totalPage: number;
  };
}

export const getUam = async ({
  page, limit 
}: GetUamProps): Promise<UAM> => {
  try {
    const { data } = await APIKycMedblock.get(`/kyc?page=${page}&perPage=${limit}`);
    return data
  } catch (error) {
    throw error;
  }
};

interface UAMDetail {
  fullName: string;
  nik: string;
  address: string;
  placeBirth: string;
  dateBirth: string;
  marital: string;
  gender: string;
  verification: Verification;
  idCard: string;
}

export const getUamDetail = async ({
  user
}: { user: string }): Promise<UAMDetail> => { 
  try {
    const { data } = await APIKycMedblock.get(`/kyc/${user}`);
    return data.data;
  } catch (error) {
    throw error;
  }
}

interface UpdateUamProps {
  user: string;
  message?: string;
  verification: Verification;
}

export const updateUam = async({
  user, message, verification
}: UpdateUamProps): Promise<void> => {
  try {
    const response = await APIKycMedblock.put(`/kyc/${user}`, { 
      verification: verification,
      message: message || ''
    });
    return response.data;
  } catch (error) {
    throw error;
  }
}