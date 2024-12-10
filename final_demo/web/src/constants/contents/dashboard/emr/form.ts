import { MdBloodtype } from "react-icons/md";
import { FaHeartbeat } from "react-icons/fa";
import { FaTemperatureHalf } from "react-icons/fa6";
import { LuWind } from "react-icons/lu";
import { PiLeafFill } from "react-icons/pi";

export const emrForm = {
  info: {
    visit_date: {
      label: "Visit Date",
      placeholder: "Visit Date",
      name: "visit_date"
    },
    discharge_date: {
      label: "Discharge Date",
      placeholder: "Discharge Date",
      name: "discharge_date"
    },
    visit_time: {
      label: "Visit Time",
      placeholder: "Visit Time",
      name: "visit_time"
    },
    discharge_time: {
      label: "Discharge Time",
      placeholder: "Discharge Time",
      name: "discharge_time"
    },
    room: {
      label: "Room / Unit",
      placeholder: "Room / Unit Name"
    },
    medical_officer: {
      label: "Medical Officer",
      placeholder: "Medical Officer Name"
    }
  },
  history: {
    circuit_reason: {
      label: "Circuit Reason",
      placeholder: "Circuit Reason",
      name: "circuit_reason"
    },
    illness_history: {
      label: "Illness History",
      placeholder: "History of present illness",
      name: "illness_history"
    },
    pyhsical_exam: {
      label: "Physical Exam",
      placeholder: "Physical Exam",
      name: "pyhsical_exam"
    },
    drug_allergy: {
      label: "Drug Allergy",
      placeholder: "Drug Allergy",
      name: "drug_allergy"
    },
    food_allergy: {
      label: "Food Allergy",
      placeholder: "Food Allergy",
      name: "food_allergy"
    },
    other_allergy: {
      label: "Other Allergy",
      placeholder: "Other Allergy",
      name: "other_allergy"
    },
  },
  reports: {
    additional_exam: {
      label: "Additional Exam",
      placeholder: "(Laboratory, Rontgen, Anatomic Pathology and Consultation)",
      name: "additional_exam"
    },
    primary_diagnosis: {
      label: "Primary Diagnosis",
      placeholder: "Primary Diagnosis",
      name: "primary_diagnosis"
    },
    secondary_diagnosis: {
      label: "Secondary Diagnosis",
      placeholder: "Secondary Diagnosis",
      name: "secondary_diagnosis"
    },
    surgery: {
      label: "Surgery",
      placeholder: "Surgery",
      name: "surgery"
    },
    procedures_and_therapies: {
      label: "Procedures and Therapies",
      placeholder: "Procedures and Therapies",
      name: "procedures_and_therapies"
    }
  },
  history_of_allergy: {
    drug_allergy: {
      label: "Drug Allergy",
      placeholder: "Drug Allergy",
      name: "drug_allergy"
    },
    food_allergy: {
      label: "Food Allergy",
      placeholder: "Food Allergy",
      name: "food_allergy"
    },
    other_allergy: {
      label: "Other Allergy",
      placeholder: "e.g. Pet, Dust, Asthma",
      name: "other_allergy"
    },
  },
  discharge_condition: {
    label: "Discharge Condition",
    name: "discharge_condition"
  }
}

export const vitalSignHeader = {
  label: 'Vital Sign',
}

export const vitalSignInput = [
  { id: 1, label: 'Blood Pressure', icon: MdBloodtype, name: 'blood_pressure', placeholder: '.....', unit: 'mmHG' },
  { id: 2, label: 'Heart Rate', icon: FaHeartbeat, name: 'heart_rate', placeholder: '.....', unit: 'Bpm' },
  { id: 3, label: 'Temperature', icon: FaTemperatureHalf, name: 'temperature', placeholder: '.....', unit: '°C' },
  { id: 4, label: 'Respiration', icon: LuWind, name: 'respiration', placeholder: '.....', unit: 'Bpm' },
  { id: 5, label: 'Oxygen Saturation', icon: PiLeafFill, name: 'o2_saturation', placeholder: '.....', unit: '%' },
] as const;