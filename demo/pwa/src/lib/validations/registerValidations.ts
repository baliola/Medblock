import * as Yup from 'yup';

const registrationValidation = Yup.object().shape({
  name: Yup.string().required(' Fullname is required'),
  address: Yup.string().required(' Address is required'),
  nik: Yup.string()
    .required('Identity Number is required')
    .length(16, 'Identity Number must be exactly 16 digits'),
  martial_status: Yup.string().required(' Martial status is required'),
  gender: Yup.string().required(' Gender is required'),
  place_of_birth: Yup.string().required(' Place of birth is required'),
  date_of_birth: Yup.string().required(' Date of birth is required'),
});

export default registrationValidation;
