import { MdBloodtype } from "react-icons/md";
import { FaHeartbeat } from "react-icons/fa";
import { FaTemperatureHalf } from "react-icons/fa6";
import { LuWind } from "react-icons/lu";
import { PiLeafFill } from "react-icons/pi";

export const emrForm = {
  info: {
    visit_date: {
      label: "Visit Date",
      placeholder: "Visit Date"
    },
    discharge_date: {
      label: "Discharge Date",
      placeholder: "Discharge Date"
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
  reports: {
    subjective: {
      label: "Subjective",
      placeholder: "Subjective",
      name: "subjective"
    },
    diagnosis: {
      label: "Diagnosis",
      placeholder: "Diagnosis",
      name: "diagnosis"
    },
    planning: {
      label: "Planning",
      placeholder: "Planning",
      name: "planning"
    },
    medication: {
      label: "Medication",
      placeholder: "Medication",
      name: "medication"
    }
  },
  recipe: {
    label: "Recipe",
    placeholder: "Recipe",
    name: "recipe"
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