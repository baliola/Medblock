import * as yup from 'yup';

export const HospitalRegistrationSchema = yup.object().shape({
  name: yup.string()
    .matches(/^[a-zA-Z\s]*$/, 'Hospital name must be alphabetic')
    .required('Hospital name is required'),
  address: yup.string()
    .matches(/^[a-zA-Z0-9\s]*$/, 'Hospital address must be alphanumeric')
    .required('Hospital address is required'),
});