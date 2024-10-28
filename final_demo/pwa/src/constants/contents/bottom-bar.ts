import { type Active } from "@/components/bottom-bar";

import { BsShieldFillPlus } from "react-icons/bs";
import { FaFolderOpen, FaHouse, FaPeopleGroup } from "react-icons/fa6";
import { IoSettings } from "react-icons/io5";

export interface Link {
  name: string;
  icon: React.ElementType;
  href: string;
  active: Active
}

export const bottomBarLinks: Link[] = [
  { 
    name: "My Emr", 
    icon: FaFolderOpen,
    href: "/emr",
    active: "emr" 
  },
  // { 
  //   name: "My Family", 
  //   icon: FaPeopleGroup,
  //   href: "/family",
  //   active: "family" 
  // },
  { 
    name: "Home", 
    icon: FaHouse,
    href: "/home",
    active: "home" 
  },
  // { 
  //   name: "Insurance", 
  //   icon: BsShieldFillPlus,
  //   href: "/insurance",
  //   active: "insurance" 
  // },
  { 
    name: "Setting", 
    icon: IoSettings,
    href: "/setting",
    active: "setting" 
  },
];