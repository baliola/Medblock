import * as Yup from "yup";

export const grantGroupAccessSchema = Yup.object().shape({
  grantee_nik: Yup.string()
    .required("NIK is required"),
});