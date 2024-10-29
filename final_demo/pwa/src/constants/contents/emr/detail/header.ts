import { FaHospital } from "react-icons/fa6";
import { MdMonetizationOn } from "react-icons/md";

export const emrDetailHeader = {
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
  report: [
    {
      title: "Visit Date",
      key: "visit_date"
    },
    {
      title: "Medical Officer",
      key: "medical_officer"
    }
  ],
  information: [
    {
      icon: FaHospital,
      title: "Medic-Act Agreement",
      bgColor: "primary.200",
      textColor: "primary.600"
    },
    {
      icon: MdMonetizationOn,
      title: "Payment Information",
      bgColor: "success.200",
      textColor: "success.600"
    }
  ]
}