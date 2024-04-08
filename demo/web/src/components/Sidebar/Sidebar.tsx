'use client';

import Image from 'next/image';
import {
  Activity,
  ArrowRight2,
  Blogger,
  Calendar,
  Document,
  Element3,
  Folder2,
  FolderConnection,
  Headphone,
  Location,
  Logout,
  MoneyAdd,
  Profile2User,
  SecurityUser,
  Setting2,
  Setting4,
  ShoppingCart,
  Star,
  Timer1,
  Triangle,
  UserOctagon,
  UserSquare,
} from 'iconsax-react';
import Link from 'next/link';
import { useCentralStore } from '@/Store';
import { useEffect } from 'react';
import { useRouter } from 'next/router';
import { logo } from '@/lib/assets';

function Sidebar() {
  const router = useRouter();
  const { pathname } = router;
  const { setIsSidebarOpen, isSidebarOpen } = useCentralStore();

  // useEffect(() => {
  //     if (!isSidebarOpen) setIsSidebarOpen(!isSidebarOpen)
  // }, [pathname])

  return (
    <div className="fixed w-60 shrink-0 md:block h-screen  top-0 overflow-hidden">
      <div className="w-full h-full bg-white border-r">
        {/* logo */}
        <div className="p-4 md:p-6 flex cursor-pointer group items-center gap-2">
          {/* <div className="h-10 outline outline-violet-300 w-10 flex items-center bg-gradient-to-br justify-center rounded-full from-violet-500 to-violet-400 text-white"> */}
          <Image
            src={logo}
            alt="User"
            width={189}
            height={36}
            // className="rounded-full"
          />
          {/* </div> */}
        </div>

        {/* section divider */}
        <hr className="bg-gray-400 mx-2" />

        {/* other section */}
        <div className="flex flex-col h-full justify-between">
          {/* top */}
          <div className="pt-6 text-gray-500 font-medium space-y-2 md:px-2 text-xs">
            <Link
              href={'/'}
              className={`flex ${
                pathname === '/' ? 'text-[#3E48D6]' : ''
              } hover:px-8 duration-200 rounded-md w-full py-2 px-6 items-center gap-2`}
            >
              <Element3 variant="Outline" size={16} />
              Dashboard
            </Link>
          </div>

          <div>
            <div className="text-gray-500 text-xs font-medium md:px-2">
              <button
                className={`flex  hover:px-8 duration-200 px-6 py-2 items-center gap-2 text-red-500`}
                // onClick={handleLogout}
              >
                <Logout size={16} />
                Logout
              </button>

              {/* <button
                className={`flex ${
                  pathname === '/app/support' ? 'text-primary' : ''
                } hover:px-8 duration-200 px-6 py-2 items-center gap-2`}
              >
                <Headphone size={16} />
                Support
              </button> */}
            </div>

            <hr className="bg-gray-400 mx-2 my-4" />

            {/* bottom */}
          </div>
        </div>
      </div>
    </div>
  );
}

export default Sidebar;
