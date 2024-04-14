import * as Yup from 'yup';

const RegisterProviderValidations = Yup.object({
  // country_code: Yup.string().required('country_code is required'),
  displayName: Yup.string().required('Hospital name is required'),
  address: Yup.string().required('Hospital address is required'),

  // .required('Numeric Code is required'),
});

export default RegisterProviderValidations;
