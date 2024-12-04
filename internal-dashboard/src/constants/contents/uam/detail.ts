import { BsFillPersonVcardFill, BsPersonCheckFill } from "react-icons/bs";
import { FaCalendarAlt, FaHome } from "react-icons/fa";
import { IoDocumentAttachSharp } from "react-icons/io5";
import { MdOutlineLocationCity } from "react-icons/md";

export const uamDeniedModal = {
  title: "Reason for Denied",
  reasons: [
    { value: "Full Name not match with ID Card", label: "Full Name not match with ID Card" },
    { value: "Address not match with ID Card", label: "Address not match with ID Card" },
    { value: "ID not match with the picture", label: "ID not match with the picture" }
  ],
  others: {
    label: "Others Reason",
    placeholder: "Write the reason..."
  }
}

export const uamDetailHeader = {
  title : "User Details"
}

export const uamDetailProfile = {
  header: {
    title: "Self Data Completeness"
  },
  full_name: {
    label: "Full Name",
    icon: BsPersonCheckFill
  },
  address: {
    label: "Address",
    icon: FaHome
  },
  place_of_birth: {
    label: "Place of Birth",
    icon: MdOutlineLocationCity
  },
  birthdate: {
    label: "Birthdate",
    icon: FaCalendarAlt
  },
  martial_status: {
    label: "Martial Status",
    icon: IoDocumentAttachSharp
  },
  identity_card: {
    label: "Identity Card",
    icon: BsFillPersonVcardFill
  }
}

export const uamDetailButton = {
  approved: {
    label: "Approve",
  },
  denied: {
    label: "Denied",
  }
}