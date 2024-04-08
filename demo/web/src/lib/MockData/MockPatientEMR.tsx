'use-client';
import { faker } from '@faker-js/faker';
import useMockData from './useMockData';

const hospitalNames = [
  'Rumah Sakit Sanglah',
  'Rumah Sakit Bros',
  'Rumah Sakit Puri Bunda',
];
const diagnosis = ['Asma', 'Sakit Jantung', 'Diabetes'];

export type MockMedicalRecord = {
  id: string;
  patientId: string; // Assuming patientId is used to link to patients
  doctorName: string;
  hospitalName: string;
  diagnosis: string;
  treatment: string;
  createdAt: Date;
  updateAt: Date;
};

const mockMedicalRecordData = (): MockMedicalRecord => {
  return {
    id: faker.datatype.uuid(),
    patientId: faker.datatype.uuid(), // Assuming patientId is generated in this mock
    doctorName: faker.person.fullName(), // Generate doctor name
    hospitalName: getRandomHospitalName(), // Get random hospital name
    diagnosis: getRandomDiagnosis(), // Generate diagnosis
    treatment: faker.lorem.words(10), // Generate treatment
    createdAt: faker.date.past(),
    updateAt: faker.date.future(),
  };
};

const getRandomHospitalName = () => {
  const index = Math.floor(Math.random() * hospitalNames.length);
  return hospitalNames[index];
};
const getRandomDiagnosis = () => {
  const index = Math.floor(Math.random() * diagnosis.length);
  return diagnosis[index];
};

const useMedicalRecordMock = () => {
  const generateMockMedicalRecords = useMockData(100, mockMedicalRecordData);

  return {
    generateMockMedicalRecords,
  };
};

export default useMedicalRecordMock;
