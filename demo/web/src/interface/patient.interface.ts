export interface PatientListResponse {
  patients: Patient[];
}

export type Patient = {
  V1: V1;
};

export interface V1 {
  martial_status: string;
  place_of_birth: string;
  address: string;
  gender: string;
  date_of_birth: string;
}
