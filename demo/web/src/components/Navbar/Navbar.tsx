import React from 'react';
import Image from 'next/image';
import {
  Add,
  CalendarEdit,
  DirectNotification,
  SearchNormal1,
  SidebarLeft,
} from 'iconsax-react';
import { useCentralStore } from '@/Store';
import { UserCircleIcon } from '@heroicons/react/20/solid';

function Navbar({
  isOpen,
  sidebarChange,
}: {
  isOpen: boolean;
  sidebarChange: (value: boolean) => void;
}) {
  const { provider } = useCentralStore();
  return (
    <div>
      <div className="flex p-4 md:p-6 justify-between items-center">
        {/* profile/left section */}
        <div className="flex items-center justify-between gap-2">
          <div className="flex items-center justify-between gap-2">
            <UserCircleIcon width={40} height={40} />
            <div className="">
              <p className="text-sm font-semibold text-gray-800 capitalize">
                {provider ? provider.V1.display_name : ''}
              </p>
              <p className="text-xs font-medium text-gray-500 capitalize">
                {' '}
                {provider ? provider.V1.address : ''}
              </p>
            </div>
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
