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
  report: [
    {
      title: "Reason to visit",
      key: "subjective",
    },
    {
      title: "Diagnosis",
      key: "diagnosis",
    },
    {
      title: "Planning",
      key: "planning",
    },
    {
      title: "Medication",
      key: "medication",
    },
    {
      title: "Recipe",
      key: "recipe",
    },
  ],
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
  }
}