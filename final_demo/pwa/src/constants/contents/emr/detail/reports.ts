import { FaHeartbeat } from "react-icons/fa";
import { FaTemperatureHalf } from "react-icons/fa6";
import { LuWind } from "react-icons/lu";
import { MdBloodtype } from "react-icons/md";
import { PiLeafFill } from "react-icons/pi";

export const emrDetailReports = {
  /**
   * IMPORTANT
   * 
   * Don't change the key value.
   * It is for mapping the data from 
   * the provided canister.
   * 
   * Change it only if the canister gives
   * a different response.
   */
  header: {
    report: {
      title: "Visit Summary",
    },
    vital_sign: {
      title: "Vital Sign",
    }
  },
  vital_signs: {
    blood_pressure: {
      label: "Blood Pressure",
      unit: "mmHG",
      icon: MdBloodtype
    },
    temperature: {
      label: "Temperature",
      unit: "℃",
      icon: FaTemperatureHalf
    },
    heart_rate: {
      label: "Heart Rate",
      unit: "Bpm",
      icon: FaHeartbeat
    },
    respiration: {
      label: "Respiration",
      unit: "Bpm",
      icon: LuWind
    },
    oxygen_saturation: {
      label: "02 Saturation",
      unit: "%",
      icon: PiLeafFill
    }
  },
  history: [
    {
      title: "Circuit Reason",
      key: "circuit_reason",
    },
    {
      title: "Illness History",
      key: "illness_history",
    },
  ],
  allergies: [
    {
      title: "Physical Exam",
      key: "pyhsical_exam",
    },
    {
      title: "Drug Allergy",
      key: "drug_allergy",
    },
    {
      title: "Other Allergy",
      key: "other_allergy",
    },
    {
      title: "Food Allergy",
      key: "food_allergy",
    },
  ],
  result: [
    {
      title: "Additional Exam (Laboratory, Rontgen, Anatomic Pathology and Consultation)",
      key: "additional_exam",
    },
    {
      title: "Primary Diagnosis",
      key: "primary_diagnosis",
    },
    {
      title: "Secondary Diagnosis",
      key: "secondary_diagnosis",
    },
    {
      title: "Surgery",
      key: "surgery",
    },
    {
      title: "Procedures and Therapies",
      key: "procedures_and_therapies",
    },
    {
      title: "Discharge Condition",
      key: "discharge_condition",
    },
  ],
}