export const registrationInitialValues = {
  nik: "",
  address: "",
  name: "",
  gender: "",
  place_of_birth: "",
  date_of_birth: "",
  martial_status: "",
  idcard_upload: false,
  aggree: false
};

export const registrationFormTerms = {
  label: "I Agree to term of service’s Medblock and all of information"
}

export const registrationFormButton = {
  label: "Submit"
}

export const registrationForm = {
  nik: {
    label: "Valid Identity Number (NIK)",
    placeholder: "Enter your Valid identity number"
  },
  address: {
    label: "Address",
    placeholder: "Enter your address"
  },
  name: {
    label: "Full Name",
    placeholder: "Enter your full name"
  },
  gender: {
    label: "Gender",
    placeholder: "Select Gender",
    options: [
      { value: "male", label: "Male" },
      { value: "female", label: "Female" }
    ]
  },
  place_of_birth: {
    label: "Place of Birth",
    placeholder: "Enter your place of birth"
  },
  date_of_birth: {
    label: "Date Of Birth",
    placeholder: "Enter your date of birth"
  },
  martial_status: {
    label: "Marital Status",
    placeholder: "Select marital Status",
    options: [
      { value: "single", label: "Single" },
      { value: "married", label: "Married" }
    ]
  }
}

export const registrationFormAction = {
  onError: {
    title: "Register Error",
    description: "Something went wrong, try again!",
  }
}