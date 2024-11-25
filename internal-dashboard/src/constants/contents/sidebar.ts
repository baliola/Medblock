// import { TbHomeFilled } from "react-icons/tb";
// import { HiUsers } from "react-icons/hi2";
// import { IoMdSettings } from "react-icons/io";
import { FaHospital } from "react-icons/fa6";

export type Active = 'dashboard' | 'ham' | 'uam' | 'settings';

interface SidebarLink {
  name: string;
  icon: React.ComponentType;
  href: string;
  active: Active;
}

export const sidebarLinks = [
  // { name: 'Overview', icon: TbHomeFilled, href: '/', active: 'dashboard' },
  { name: 'Hospital Management', icon: FaHospital, href: '/', active: 'dashboard' },
  // { name: 'User Management', icon: HiUsers, href: '/uam', active: 'uam' },
  // { name: 'Setting', icon: IoMdSettings, href: '/settings', active: 'settings' }
] as SidebarLink[]; 