import { APIKycMedblock } from ".";

export interface KYC { 
  nikHash: string;
  fullName: string;
  nik: string;
  address: string;
  gender: string;
  placeBirth: string;
  dateBirth: string;
  marital: string;
  card: File;
}

export const createKYC = async (data: KYC) => {
  try {
    let formData = new FormData();

    console.log(data)
    console.log(data.card)
    
    formData.append('nikHash', data.nikHash);
    formData.append('fullName', data.fullName);
    formData.append('nik', data.nik);
    formData.append('address', data.address);
    formData.append('gender', data.gender);
    formData.append('placeBirth', data.placeBirth);
    formData.append('dateBirth', data.dateBirth);
    formData.append('marital', data.marital);
    formData.append('card', data.card);
    
    const response = await APIKycMedblock.post('/kyc', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    });
    console.log(response);
    return response;
  } catch (error) {
    throw error;
  }
}

export type Verification = "pending" | "accepted" | "rejected";

interface KYCStatus {
  verification: Verification;
  verificationHistory: {
    timestamp: string;
    message: string;
  }[];
}

export const getKYCStatus = async (nik: string): Promise<KYCStatus> => { 
  try {
    const response = await APIKycMedblock.get(`/kyc/status/${nik}`);
    console.log(response)
    return response.data.data;
  } catch (error) {
    throw error;
  }
}
