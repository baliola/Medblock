'use-client';
import { faker } from '@faker-js/faker';
import useMockData from './useMockData';

export type MockPatients = {
  id: string;
  firstName: string;
  gender: string;
  dob: Date;
  pob: string;
  address: string;
  phoneNumber: string;
  email: string;
  createdAt: Date;
};

const mockPatientData = (): MockPatients => {
  return {
    id: faker.datatype.uuid(),
    firstName: faker.person.firstName(),
    gender: faker.helpers.arrayElement(['Male', 'Female', 'Other']),
    dob: faker.date.past(),
    pob: faker.address.city(),
    address: faker.address.streetAddress(),
    phoneNumber: faker.phone.number(),
    email: faker.internet.email(),
    createdAt: faker.date.past(),
  };
};

const usePatientMock = () => {
  const generateMockPatients = useMockData(100, mockPatientData);

  return {
    generateMockPatients,
  };
};

export default usePatientMock;
