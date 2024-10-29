import { FaChevronRight, FaLock } from "react-icons/fa6";
import { FiLogOut } from "react-icons/fi";
import { IoNotifications } from "react-icons/io5";

export const settingButton = {
  signOut: {
    label: 'Logout',
    icon: FiLogOut,
    color: 'red',
  },
  notification: {
    label: 'Notification Setting',
    leftIcon: IoNotifications,
    rightIcon: FaChevronRight
  },
  change_pin: {
    label: 'Change PIN',
    leftIcon: FaLock,
    rightIcon: FaChevronRight,
    redirect: '/pin/add'
  }
}