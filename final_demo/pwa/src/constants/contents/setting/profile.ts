import { FaCalendarAlt } from "react-icons/fa";
import { HiHome } from "react-icons/hi2";
import { IoDocumentAttach } from "react-icons/io5";

export const settingProfile = {
  title: 'Personal Information',
  contents: {
    home_address:{
      icon: HiHome,
      label: 'Home Address',
    },
    martial_status:{
      icon: IoDocumentAttach,
      label: 'Marital Status',
    },
    birthdate:{
      icon: FaCalendarAlt,
      label: 'Birthdate & Place',
    }
  }
}