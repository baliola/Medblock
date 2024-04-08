export interface DoctorType {
  id: string;
  name: string;
  specialty: string;
}
export const dummyDoctorTypes: DoctorType[] = [
  { id: '1', name: 'Dr. John Smith', specialty: 'Cardiologist' },
  { id: '2', name: 'Dr. Jane Doe', specialty: 'Neurologist' },
  { id: '3', name: 'Dr. David Lee', specialty: 'Orthopedic Surgeon' },
  // Add more doctor types as needed
];
