import * as Yup from "yup";

export const addHospitalSchema = Yup.object().shape({
  hospital: Yup.string()
    .required("Hospital is required"),
  address: Yup.string()
    .required("Address is required"),
  principal: Yup.string()
    .required("Principal is required")
});