import * as Yup from "yup";

export const addPatientGroupSchema = Yup.object().shape({
  name: Yup.string()
    .required("Group Name is required"),
});