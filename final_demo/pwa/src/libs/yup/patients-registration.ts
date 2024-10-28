import * as yup from "yup";

export interface PatientRegister {
  nik: string;
  address: string;
  name: string;
  place_of_birth: string;
  date_of_birth: string;
  gender: string;
  martial_status: string;
  idcard_upload: boolean;
  agree: boolean;
}

const MIN_AGE = 17;
const MAX_AGE = 100;

export const PatientRegistrationSchema = yup.object().shape({
  nik: yup
    .string()
    .min(16, "NIK must be at least 16 characters!")
    .max(16, "NIK cannot exceed 16 characters!")
    .test("is-numeric", "", function (value) { 
      const hasNumbers = /\d/.test(value as string);

      if (!hasNumbers) {
        return this.createError({
          message: "Only numbers are allowed in NIK!"
        });
      }

      return true
    })
    .required("NIK is Required for identity!"),
  address: yup
    .string()
    .min(10, "Address must be at least 10 characters!")
    .required("Address is Required!"),
  name: yup
    .string()
    .max(60, "Name cannot exceed 60 characters including spaces!")
    .test("no-special-chars-or-numbers", "", function (value) {
      const hasNumbers = /\d/.test(value as string);
      const hasSpecialChars = /[^a-zA-Z0-9\s]/.test(value as string); 

      if (hasNumbers && hasSpecialChars) {
        return this.createError({
          message: "Special characters and numbers are not allowed!",
        });
      }

      if (hasNumbers) {
        return this.createError({ message: "Numbers are not allowed!" });
      }

      if (hasSpecialChars) {
        return this.createError({
          message: "Special characters are not allowed!",
        });
      }

      return true;
    })
    .required("Name is required!"),
  place_of_birth: yup
    .string()
    .min(4, "Place of Birth must be at least 4 characters!")
    .max(60, "Place of Birth cannot exceed 60 characters including spaces!")
    .matches(/^[a-zA-Z\s]*$/, "Only letters are allowed in Place of Birth!")
    .required("Place of Birth is Required!"),
  date_of_birth: yup
    .date()
    .required("Date of birth is required!")
    .test(
      "date_of_birth",
      "Future dates are not allowed!",
      function (value) {
        return value ? value <= new Date() : false;
      }
    ),
    // .test("date_of_birth", `You must be at least ${MIN_AGE} years old!`, function (value) {
    //   return value
    //     ? new Date().getFullYear() - value.getFullYear() >= MIN_AGE
    //     : false;
    // })
    // .test("date_of_birth", `You cannot be older than ${MAX_AGE} years!`, function (value) {
    //   return value
    //     ? new Date().getFullYear() - value.getFullYear() <= MAX_AGE
    //     : false;
    // })
  gender: yup
    .string()
    .matches(/^[a-zA-Z]*$/, "Gender must be alphabetic")
    .oneOf(["male", "female"], "Gender must be either 'male' or 'female'")
    .required("Gender is Required!"),
  martial_status: yup
    .string()
    .matches(/^[a-zA-Z]*$/, "Marital status must be alphabetic")
    .oneOf(
      ["married", "single"],
      "Marital status must be either 'married' or 'single'"
    )
    .required("Marital Status is Required!"),
  idcard_upload: yup
    .boolean()
    .test("idcard_upload", "ID Card is required!", function (value) {
      return value;
    })
    .required("ID Card is required!"),
  agree: yup
    .boolean()
    .oneOf([true], "You must accept the terms and conditions.")
    .required("You must accept the terms and conditions."),
});
