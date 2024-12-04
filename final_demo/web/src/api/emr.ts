import { MedBlockAPI } from "@/api"

export const getPatientEMR = async ({ id }: { id: number }) => {
  try {
    const response = await MedBlockAPI.get(`/emr/${id}`);
    return response.data.data;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(error.message);
    }
    throw new Error("An error occurred while fetching patient EMR");
  }
}

export const getLatestEMR = async ({
  patientId,
  recordId
}: {
  patientId: string,
  recordId: number | null
}) => { 
  try {
    const response = await MedBlockAPI.get(`/emr/${patientId}/latest`);
    return response.data.data;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(error.message);
    }
    throw new Error("An error occurred while fetching latest patient EMR");
  }
}

export const getHistoryEMR = async ({ id }: { id: string }) => {
  try {
    const response = await MedBlockAPI.get(`/emr/${id}/history`);
    return response.data.data;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(error.message);
    }
    throw new Error("An error occurred while fetching patient EMR history");
  }
}