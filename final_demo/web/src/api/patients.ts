import { MedBlockAPI } from "@/api";

export const getAllPatients = async({
  search = "",
  page = 1,
  limit = 10
}: {
  search?: string;
  page?: number;
  limit?: number;
}) => {
  try {
    const response = await MedBlockAPI.get(`/patient?page=${page}&limit=${limit}&search=${search}`);
    return response.data;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(error.message);
    }
    throw new Error("An error occurred while fetching patients");
  }
}

export const getPatientById = async (id: string) => {
  try {
    const response = await MedBlockAPI.get(`/patient/${id}`);
    console.log(response);
    return response.data.data;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(error.message);
    }
    throw new Error("An error occurred while fetching patient");
  }
}