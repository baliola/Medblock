import React from 'react';
import Image from 'next/image';
import {
  Add,
  CalendarEdit,
  DirectNotification,
  SearchNormal1,
  SidebarLeft,
} from 'iconsax-react';

function Navbar({
  isOpen,
  sidebarChange,
}: {
  isOpen: boolean;
  sidebarChange: (value: boolean) => void;
}) {
  return (
    <div>
      <div className="flex p-4 md:p-6 justify-between items-center">
        {/* profile/left section */}
        <div className="flex items-center justify-between gap-2">
          <div className="">
            <p className="text-2xl font-semibold text-gray-800">
              Medblock Dashboard
            </p>
          </div>
        </div>

        <button
          onClick={() => sidebarChange(!isOpen)}
          className="all-center text-gray-500 h-8 w-8 md:hidden"
        >
          <SidebarLeft size={16} />
        </button>

        {/* right section */}
      </div>

      <hr className="bg-gray-400 mx-2" />
    </div>
  );
}

export default Navbar;
