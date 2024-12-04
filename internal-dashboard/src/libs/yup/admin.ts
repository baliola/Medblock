import * as Yup from "yup";

export const addAdminSchema = Yup.object().shape({
  nik: Yup.string().required("NIK is required"),
    // .matches(/^\d+$/, 'Only numbers are allowed')
    // .length(16, 'Length must be 16 characters'),
  principal: Yup.string()
    .required("Principal is required")
});