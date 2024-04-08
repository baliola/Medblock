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

function PatientInfo() {
  const router = useRouter();
  const { pathname } = router;
  const { setIsSidebarOpen, isSidebarOpen } = useCentralStore();

  // useEffect(() => {
  //     if (!isSidebarOpen) setIsSidebarOpen(!isSidebarOpen)
  // }, [pathname])

  return (
    <div className="fixed w-80 shrink-0 md:block h-screen  top-0 overflow-hidden">
      <div className="w-full h-full bg-white border-r">
        {/* logo */}
        <div className="justify-start p-4 md:p-6 flex cursor-pointer group items-center gap-2">
          {/* <div className="h-10 outline outline-violet-300 w-10 flex items-center bg-gradient-to-br justify-center rounded-full from-violet-500 to-violet-400 text-white"> */}
          <p className="text-xl font-medium text-gray-800">
            Patient Information
          </p>
          {/* </div> */}
        </div>

        {/* section divider */}
        <hr className="bg-gray-400 mx-2" />

        {/* other section */}
        <div className="flex flex-col h-full gap-2">
          {/* top */}
          <div className="pt-6 text-gray-500 font-medium space-y-2 md:px-2 text-sm">
            <p
              className={`flex duration-200 rounded-md w-full py-2 px-6 items-center gap-2`}
            >
              Name: Max{' '}
            </p>
          </div>
          <div className=" text-gray-500 font-medium space-y-2 md:px-2 text-sm">
            <p
              className={`flex duration-200 rounded-md w-full py-2 px-6 items-center gap-2`}
            >
              Gender: male{' '}
            </p>
          </div>{' '}
          <div className=" text-gray-500 font-medium space-y-2 md:px-2 text-sm">
            <p
              className={`flex duration-200 rounded-md w-full py-2 px-6 items-center gap-2`}
            >
              Address: Jl Renon Denpasar
            </p>
          </div>{' '}
          <div className=" text-gray-500 font-medium space-y-2 md:px-2 text-sm">
            <p
              className={`flex duration-200 rounded-md w-full py-2 px-6 items-center gap-2`}
            >
              Martial States: Single
            </p>
          </div>{' '}
          <div className=" text-gray-500 font-medium space-y-2 md:px-2 text-sm">
            <p
              className={`flex duration-200 rounded-md w-full py-2 px-6 items-center gap-2`}
            >
              Date of Birth: 1 August 2000
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default PatientInfo;
