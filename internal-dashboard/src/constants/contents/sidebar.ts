// import { TbHomeFilled } from "react-icons/tb";
// import { IoMdSettings } from "react-icons/io";
import { FaHeadset, FaHospital, FaHospitalUser } from "react-icons/fa6";

export type Active = 'dashboard' | 'ham' | 'uam' | 'settings' | 'admin';

interface SidebarLink {
  name: string;
  icon: React.ComponentType;
  href: string;
  active: Active;
}

export const sidebarLinks = [
  // { name: 'Overview', icon: TbHomeFilled, href: '/', active: 'dashboard' },
  { name: 'Hospital Management', icon: FaHospital, href: '/', active: 'dashboard' },
  { name: 'User Management', icon: FaHospitalUser, href: '/uam', active: 'uam' },
  { name: 'Admin Management', icon: FaHeadset , href: '/admin', active: 'admin' },
  // { name: 'Setting', icon: IoMdSettings, href: '/settings', active: 'settings' }
] as SidebarLink[]; 