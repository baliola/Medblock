import { MedBlockAPI } from "@/api";

export const getOverview = async ({
  polyclinic,
  dateFrom,
  dateTo
}: {
  polyclinic: string;
  dateFrom: string;
  dateTo: string;
  }) => {
  try {
    const response = await MedBlockAPI.get("/overview");
    return response.data.data;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(error.message);
    }
    throw new Error("An error occurred while fetching overview");
  }
}