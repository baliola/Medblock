export const emrHospitalInfo = {
  header: {
    title: "Hospital Information",
  },
  contents: {
    hospital_id: {
      label: "Hospital ID",
    },
    hospital_name: {
      label: "Hospital Name",
    },
    hospital_address: {
      label: "Hospital Address",
    },
    registered_at: {
      label: "Registered At",
    },
    hospital_status: {
      label: "Hospital Status",
    },
  },
  status: {
    active: {
      label: "ACTIVE",
      colorScheme: "green", 
    },
    suspended: {
      label: "SUSPENDED",
      colorScheme: "red", 
    },
    unknown: {
      label: "UNKNOWN",
      colorScheme: "gray", 
    },
  },
  footer: {
    button: {
      label: "Close",
    },
  },
};
