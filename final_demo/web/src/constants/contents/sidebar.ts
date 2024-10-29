import { TbHomeFilled } from "react-icons/tb";
import { HiUsers } from "react-icons/hi2";
import { IoMdSettings } from "react-icons/io";

export const sidebarLinks = [
  { name: 'Overview', icon: TbHomeFilled, href: '/dashboard/overview', active: 'overview' },
  { name: 'Patients', icon: HiUsers, href: '/dashboard/patients', active: 'patients' },
  { name: 'Settings', icon: IoMdSettings, href: '/dashboard/settings', active: 'settings' }
] as const;